import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {fleetFetchContextV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts";
import {fleetMirrorTransportEnvironmentV1,refreshFleetMirrorStateV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🪞️mirror/🟦️.ts";

/** 🧪️ A scratch cache server with the mirror (scratch directory and port, nothing of the real fleet): the state probe turns healthy, a real Go and a real Bun fetch go through the mirror twice, and the second fetch reaches the upstream for nothing. */
const root=mkdtempSync(join(tmpdir(),"mirror-proof-")),directory=join(root,"server"),worktree=join(root,"worktree");
mkdirSync(worktree,{recursive:true});
const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"},mirror:{directory:join(root,"mirror")}});
const size=(path:string):number=>{let total=0;try{for(const name of readdirSync(path,{withFileTypes:true}))total+=name.isDirectory()?size(join(path,name.name)):statSync(join(path,name.name)).size;}catch{}return total;};
const run=async(argv:string[],cwd:string,env:Record<string,string>)=>{const started=performance.now(),child=Bun.spawn(argv,{cwd,stdout:"pipe",stderr:"pipe",env:{PATH:process.env.PATH!,HOME:join(root,"home"),...env}}),[out,err,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);return {code,seconds:(performance.now()-started)/1000,tail:(err||out).trim().split("\n").slice(-2).join(" | ")};};
try{
 const context=fleetFetchContextV1({SEMIO_WORKSPACE_ROOT:worktree},worktree,{remote:{endpoint:server.url,instance:"main",token:"w",authenticated:true}});
 const state=await refreshFleetMirrorStateV1(context);
 console.log(JSON.stringify({state,file:JSON.parse(readFileSync(join(worktree,".cache","f","mirror-state.json"),"utf8"))}));
 const ambient={SEMIO_WORKSPACE_ROOT:worktree},bunEnv=fleetMirrorTransportEnvironmentV1("bun",{},ambient),goEnv=fleetMirrorTransportEnvironmentV1("go",{},ambient);
 console.log(JSON.stringify({bunEnv,goEnv}));
 for(const round of [1,2]){
  const before=size(join(root,"mirror"));
  const go=await run(["go","mod","download","-x","golang.org/x/text@v0.14.0"],worktree,{...goEnv,GOPATH:join(root,`gopath${round}`),GOMODCACHE:join(root,`gomod${round}`),GOFLAGS:"-modcacherw",GOSUMDB:"off",GO111MODULE:"on",GOCACHE:join(root,"gocache")});
  const project=join(root,`bun${round}`);mkdirSync(project,{recursive:true});writeFileSync(join(project,"package.json"),'{"name":"p","version":"1.0.0"}');
  const bun=await run([process.execPath,"add","is-number@7.0.0"],project,{...bunEnv,BUN_INSTALL_CACHE_DIR:join(root,`bun-cache${round}`)});
  console.log(JSON.stringify({round,go:{code:go.code,seconds:go.seconds},bun:{code:bun.code,seconds:bun.seconds,tail:bun.tail},mirrorBytesAdded:size(join(root,"mirror"))-before}));
 }
}finally{await server.stop();rmSync(root,{recursive:true,force:true});}
