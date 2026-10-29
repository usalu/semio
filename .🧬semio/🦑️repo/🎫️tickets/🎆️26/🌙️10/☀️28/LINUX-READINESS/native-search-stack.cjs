
const fs=require("node:fs"),cp=require("node:child_process"),crypto=require("node:crypto");
const phaseMode=process.argv[2]==="phase";if(process.argv[2]&&!phaseMode)throw Error("Declared diagnostic mode required");
const binary="/home/vscode/.local/bin/repo",args=["--repo=/workspaces/semio","--format=json","search","linux","--limit=8"],started=Date.now();
const child=cp.spawn(binary,args,{cwd:"/workspaces/semio",env:{...process.env,SEMIO_LANGUAGE:"en",GOTRACEBACK:"system"},stdio:["ignore","pipe","pipe"]});
let stdoutBytes=0,stderrBytes=0,carry="",timer,signalled=false,armed=false;const digest=crypto.createHash("sha256");
const read=pid=>{const root="/proc/"+pid,status=fs.readFileSync(root+"/status","utf8"),argv=fs.readFileSync(root+"/cmdline","utf8").split("\0").filter(Boolean);return{pid,ppid:+status.match(/^PPid:\s+(\d+)/m)[1],state:status.match(/^State:\s+(.*)/m)[1],argv:argv.map(item=>item.length>256?"<captured argument "+item.length+" bytes>":item),rawArgv:argv,exe:fs.readlinkSync(root+"/exe"),wait:fs.readFileSync(root+"/wchan","utf8").trim()};};
const verify=()=>{const value=read(child.pid);if(value.rawArgv[0]!==binary||JSON.stringify(value.rawArgv.slice(1))!==JSON.stringify(args))throw Error("Owned native diagnostic identity changed");return value;};
function capture(){if(child.exitCode!==null||child.signalCode!==null)return;verify();const rows=[];for(const name of fs.readdirSync("/proc"))if(/^\d+$/.test(name))try{rows.push(read(+name));}catch{}const owned=new Set([child.pid]);for(let previous=-1;previous!==owned.size;){previous=owned.size;for(const row of rows)if(owned.has(row.ppid))owned.add(row.pid);}console.log("[DEBUG] native-read-path "+JSON.stringify({elapsedMs:Date.now()-started,processes:rows.filter(row=>owned.has(row.pid)).map(({rawArgv,...row})=>row)}));verify();signalled=true;child.kill("SIGQUIT");}
child.stdout.on("data",chunk=>{stdoutBytes+=chunk.length;digest.update(chunk);});
child.stderr.on("data",chunk=>{stderrBytes+=chunk.length;process.stderr.write(chunk);if(!phaseMode||armed)return;carry+=chunk.toString("utf8");let end;while((end=carry.indexOf("\n"))>=0){const line=carry.slice(0,end);carry=carry.slice(end+1);try{const row=JSON.parse(line);if(row.progress?.phase==="discovering-files"&&row.progress.current>=250000){armed=true;console.log("[DEBUG] native-phase-gate "+JSON.stringify({elapsedMs:Date.now()-started,progress:row.progress}));timer=setTimeout(capture,7000);}}catch{}}if(carry.length>65536)throw Error("Bounded progress line required");});
if(!phaseMode)timer=setTimeout(capture,7000);
const soft=phaseMode?setTimeout(()=>{if(child.exitCode===null&&child.signalCode===null){verify();child.kill("SIGTERM");}},119000):undefined;
const hard=setTimeout(()=>{if(child.exitCode===null&&child.signalCode===null){verify();child.kill("SIGKILL");}},phaseMode?120000:25000);
const clear=()=>{clearTimeout(timer);clearTimeout(soft);clearTimeout(hard);};
child.on("error",error=>{clear();throw error;});
child.on("exit",(code,signal)=>{clear();console.log("[DEBUG] native-stack-terminal "+JSON.stringify({code,signal,phaseMode,armed,signalled,elapsedMs:Date.now()-started,stdoutBytes,stdoutSha256:digest.digest("hex"),stderrBytes}));if(!signalled&&code!==0)process.exitCode=1;});
