import {readFile,writeFile,mkdir,open,lstat} from "node:fs/promises";
import {dirname,join,resolve,isAbsolute,basename} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";

const maximumBytes=16*1024*1024,maximumOutput=65536;
const stop=new AbortController();
for(const signal of ["SIGINT","SIGTERM"])process.once(signal,()=>stop.abort(new Error("PDF QA canceled")));

/** 🔧 Runs one owned native renderer with bounded capture and cancellation. */
function run(program,args,cwd){
  stop.signal.throwIfAborted();
  return new Promise((accept,reject)=>{
    const child=spawn(program,args,{cwd,shell:false,windowsHide:true,stdio:["ignore","pipe","pipe"],env:{...process.env,LC_ALL:"C"}});
    let stdout="",stderr="",refusal;
    const cancel=()=>{refusal=stop.signal.reason;child.kill("SIGTERM");};
    const timer=setTimeout(()=>{refusal=new Error("Native PDF QA command deadline");child.kill("SIGTERM");},90000);
    stop.signal.addEventListener("abort",cancel,{once:true});
    const capture=(kind,chunk)=>{if(refusal)return;if(kind==="stdout")stdout+=chunk;else stderr+=chunk;if(Buffer.byteLength(stdout)+Buffer.byteLength(stderr)>maximumOutput){refusal=new Error("Bounded native PDF output exceeded");child.kill("SIGTERM");}};
    child.stdout.setEncoding("utf8");child.stderr.setEncoding("utf8");
    child.stdout.on("data",chunk=>capture("stdout",chunk));child.stderr.on("data",chunk=>capture("stderr",chunk));
    child.once("error",error=>{clearTimeout(timer);stop.signal.removeEventListener("abort",cancel);reject(error);});
    child.once("close",(code,signal)=>{clearTimeout(timer);stop.signal.removeEventListener("abort",cancel);if(refusal)reject(refusal);else if(code!==0)reject(new Error("Native PDF command failed "+JSON.stringify({program,args,code,signal,stderr})));else accept(stdout);});
  });
}

/** 📥 Fetches one bounded PDF from the verified local origin into a fresh file. */
async function download(url,path){
  const response=await fetch(url,{redirect:"error",signal:AbortSignal.any([stop.signal,AbortSignal.timeout(60000)])});
  if(response.status!==200||response.headers.get("content-type")?.split(";")[0].trim()!=="application/pdf"||Number(response.headers.get("content-length"))>maximumBytes)throw new Error("Served PDF status, MIME or size refused");
  if(!response.body)throw new Error("Served PDF body required");
  const file=await open(path,"wx"),hash=createHash("sha256");let bytes=0;
  try{for await(const chunk of response.body){stop.signal.throwIfAborted();bytes+=chunk.length;if(bytes>maximumBytes)throw new Error("Served PDF capacity exceeded");hash.update(chunk);await file.writeFile(chunk);}}finally{await file.close();}
  const saved=await readFile(path),sha256=hash.digest("hex");
  if(bytes<5||saved.length!==bytes||saved.subarray(0,5).toString("ascii")!=="%PDF-"||createHash("sha256").update(saved).digest("hex")!==sha256)throw new Error("Complete persisted PDF bytes required");
  return{bytes,sha256};
}

/** 🖼️ Renders every page of all four actual served bilingual handouts. */
async function main(){
  const plan=JSON.parse(await readFile(process.argv[2],"utf8")),base=new URL(plan.origin),expected=new Set(["parametrization-en","parametrization-de","typology-en","typology-de"]);
  if(base.protocol!=="http:"||base.hostname!=="127.0.0.1"||base.pathname!=="/"||base.username||base.password||base.search||base.hash||!/^pdf-qa-[a-z0-9-]{1,30}$/.test(plan.id)||!Array.isArray(plan.documents)||plan.documents.length!==4)throw new Error("Closed native course PDF QA plan required");
  for(const key of ["pdfinfo","pdftoppm"]){const program=plan[key];if(typeof program!=="string"||!isAbsolute(program)||!new RegExp("^"+key+"(?:\\.exe)?$","i").test(basename(program))||!(await lstat(program)).isFile())throw new Error("Actual native PDF tool required");}
  for(const row of plan.documents){const url=new URL(row.url);if(!expected.delete(row.id)||url.origin!==base.origin||url.username||url.password||url.hash)throw new Error("Four distinct same-origin course PDFs required");}
  const generated=join(dirname(fileURLToPath(import.meta.url)),"🗑️generated"),parent=await lstat(generated);
  if(!parent.isDirectory()||parent.isSymbolicLink())throw new Error("Actual ticket generated directory required");
  const output=join(generated,plan.id);if(resolve(output)!==output||join(output,"parametrization-en-100.png").length>256)throw new Error("Bounded ticket PDF paths required");
  await mkdir(output);
  const results=[];
  for(const row of plan.documents){
    stop.signal.throwIfAborted();console.log("[DEBUG] Fetching "+row.id);
    const pdf=join(output,row.id+".pdf"),saved=await download(row.url,pdf),info=await run(plan.pdfinfo,["-isodates",row.id+".pdf"],output),pages=Number(/^Pages:\s+(\d+)\s*$/m.exec(info)?.[1]);
    if(!Number.isSafeInteger(pages)||pages<1||pages>100)throw new Error("Bounded actual PDF page count required");
    await writeFile(join(output,row.id+".info.txt"),info);
    const images=[];
    for(let page=1;page<=pages;page++){const prefix=join(output,row.id+"-"+String(page).padStart(3,"0"));console.log("[DEBUG] Rendering "+row.id+" page "+page+"/"+pages);await run(plan.pdftoppm,["-f",String(page),"-l",String(page),"-singlefile","-r","110","-png",row.id+".pdf",basename(prefix)],output);const image=prefix+".png",facts=await lstat(image);if(!facts.isFile()||facts.size<8||facts.size>maximumBytes)throw new Error("Bounded actual rendered PNG required");const png=await readFile(image);if(!png.subarray(0,8).equals(Buffer.from([137,80,78,71,13,10,26,10])))throw new Error("Actual PNG rendering required");images.push(image);}
    results.push({id:row.id,url:row.url,...saved,pages,images});
  }
  await writeFile(join(output,"result.json"),JSON.stringify({origin:base.origin,results},null,2)+"\n");
  console.log("[DEBUG] Native PDF QA rendered all pages "+join(output,"result.json"));
}

main().catch(error=>{console.error("[DEBUG] "+(error.stack??error));process.exitCode=stop.signal.aborted?130:1;});
