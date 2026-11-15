import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🌿️git/🟦️.ts",[
 ('''  if(links.length>0){
   const targets=await git(worktree,["cat-file","--batch"],{signal,stdin:`${links.map(entry=>entry.oid).join("\\n")}\\n`});
   if(targets!==null&&targets.code===0){
    let offset=0;
    for(const entry of links){
     const end=targets.out.indexOf(10,offset),size=Number(targets.out.subarray(offset,end).toString("utf8").split(" ")[2]);
     const target=targets.out.subarray(end+1,end+1+size).toString("utf8");offset=end+1+size+1;
     if(linkRefusalV1(entry.path,target)!==undefined){refused++;if(samples.length<10)samples.push(`unsafe-link ${entry.path}`);}
    }
   }
  }''','''  for(const path of await unsafeLinksV1(worktree,links,signal)){refused++;if(samples.length<10)samples.push(`unsafe-link ${path}`);}'''),
 ('const representable=(path:string):boolean=>path!==""&&!path.includes("\\\\")&&!path.includes("\\0")&&!path.split("/").some(part=>part===""||part==="."||part===".."||part===".git")&&path.length<=1000;','''const representable=(path:string):boolean=>path!==""&&!path.includes("\\\\")&&!path.includes("\\0")&&!path.split("/").some(part=>part===""||part==="."||part===".."||part===".git")&&path.length<=1000&&refusePathV1(path)===undefined;

/** 🔗 Lists the paths among the given symbolic-link entries of the stage or of a commit whose target the link policy refuses (absolute or drive-qualified targets, targets that leave the worktree); empty when the blobs cannot be read. */
async function unsafeLinksV1(worktree:string,links:readonly Readonly<{path:string;oid:string}>[],signal?:AbortSignal):Promise<readonly string[]>{
 if(links.length===0)return [];
 const targets=await git(worktree,["cat-file","--batch"],{signal,stdin:`${links.map(entry=>entry.oid).join("\\n")}\\n`});
 if(targets===null||targets.code!==0)return [];
 const found:string[]=[];
 let offset=0;
 for(const entry of links){
  const end=targets.out.indexOf(10,offset),size=Number(targets.out.subarray(offset,end).toString("utf8").split(" ")[2]);
  const target=targets.out.subarray(end+1,end+1+size).toString("utf8");offset=end+1+size+1;
  if(linkRefusalV1(entry.path,target)!==undefined)found.push(entry.path);
 }
 return found;
}'''),
 ('''function changeOf(path:string,entry:FleetManifestEntryV1):GitChangeV1|undefined{
 const name=path.slice(INDEX_PREFIX_V1.length);''','''function changeOf(path:string,entry:FleetManifestEntryV1):GitChangeV1|undefined{
 const name=path.slice(INDEX_PREFIX_V1.length);
 if(refusePathV1(name)!==undefined)return undefined;'''),
 ('''  if(!representable(change.path)){host.say({event:"skipped",stage:"git",path:change.path,reason:"unrepresentable"});continue;}
  events.push(''','''  if(!representable(change.path)){host.say({event:"skipped",stage:"git",path:change.path,reason:"unrepresentable"});continue;}
  if(unsafe.has(change.path)){host.say({event:"skipped",stage:"git",path:change.path,reason:"unsafe-link"});continue;}
  events.push('''),
 (''' const skip=new Set(marker?.paths??[]),events:Readonly<{operation:"link"|"delete";path:string;target?:string}>[]=[];''',''' const skip=new Set(marker?.paths??[]),events:Readonly<{operation:"link"|"delete";path:string;target?:string}>[]=[],unsafe=new Set(await unsafeLinksV1(worktree,changes.filter(change=>change.mode==="120000"&&!skip.has(change.path)),signal));'''),
])
