/** 🌊️ The identity of a captured compiler stream for the comparison of two results of one key: every JSON diagnostic line without its `rendered` texts (Cargo's message format, colour and terminal width pick the rendering, and the unit key leaves them out on purpose, so two honest machines store the same diagnostics in another rendering), the number of rustc's `long-type-<digits>.txt` file name set to 0, other lines without ANSI escapes, and the lines sorted (the parallel front end may emit independent diagnostics in another order). Two streams with the same identity carry the same diagnostics; the kept one is replayed. */
export function provenanceStreamIdentityV1(text:string):string{
 const strip=(value:unknown):unknown=>Array.isArray(value)?value.map(strip):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value as Record<string,unknown>).filter(([name])=>name!=="rendered").map(([name,row])=>[name,strip(row)])):value;
 return text.split("\n").filter(line=>line!=="").map(line=>{
  let row:string;
  try{row=JSON.stringify(strip(JSON.parse(line)));}catch{row=line.replace(/\u001b\[[0-9;]*m/gu,"");}
  return row.replace(/long-type-[0-9]+\.txt/gu,"long-type-0.txt");
 }).sort().join("\n");
}

/** 🌊️ Whether the differing outputs of two results are only captured streams (`⟨stdout⟩`, `⟨stderr⟩`) whose texts have the same identity (see {@link provenanceStreamIdentityV1}), so the later write is the same result in another rendering and is no divergence. `read` returns the bytes of a blob, `undefined` when it is gone (then the streams are not known to be equal). */
export async function provenanceStreamsEquivalentV1(differing:readonly string[],kept:readonly Readonly<{path:string;hash:string}>[],refused:readonly Readonly<{path:string;hash:string}>[],read:(hash:string)=>Promise<Uint8Array|undefined>):Promise<boolean>{
 if(differing.length===0||differing.some(path=>path!=="⟨stdout⟩"&&path!=="⟨stderr⟩"))return false;
 const decoder=new TextDecoder();
 for(const path of differing){
  const left=kept.find(row=>row.path===path)?.hash,right=refused.find(row=>row.path===path)?.hash;
  if(left===undefined||right===undefined)return false;
  const [a,b]=await Promise.all([read(left).catch(()=>undefined),read(right).catch(()=>undefined)]);
  if(a===undefined||b===undefined||provenanceStreamIdentityV1(decoder.decode(a))!==provenanceStreamIdentityV1(decoder.decode(b)))return false;
 }
 return true;
}

