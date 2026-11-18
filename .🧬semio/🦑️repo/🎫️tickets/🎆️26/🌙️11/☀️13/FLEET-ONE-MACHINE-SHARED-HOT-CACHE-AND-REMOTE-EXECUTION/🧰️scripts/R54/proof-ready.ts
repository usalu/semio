import{chmodSync,mkdirSync,mkdtempSync,readFileSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{READY_POLICY_V1 as P,runReady}from"../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";

/** 🧪️ Real-process proof of the `ready` role with stub tools in a private root, home and store: first run, a failing stage with back-off, then ready. */
const scratch=process.argv[2]??mkdtempSync(join(tmpdir(),"r54-proof-"));
const root=join(scratch,"root"),home=join(scratch,"home"),store=join(scratch,"store"),failFlag=join(scratch,"fail");
const bootstrap=join(root,...P.bootstrap.unix);mkdirSync(dirname(bootstrap),{recursive:true});mkdirSync(join(home,".local","bin"),{recursive:true});mkdirSync(join(root,".cache","f"),{recursive:true});
writeFileSync(bootstrap,`#!/bin/bash\necho "bootstrap $* skip=$SKIP_REPO_BOOTSTRAP"\nmkdir -p "${home}/.local/bin"\ncp "${join(scratch,"repo-stub")}" "${home}/.local/bin/repo"\nchmod +x "${home}/.local/bin/repo"\n`);
writeFileSync(join(scratch,"repo-stub"),`#!/bin/sh\necho "repo $* lang=$SEMIO_LANGUAGE"\nif [ "$1 $2" = "run workspace:setup" ] && [ -f "${failFlag}" ]; then echo "simulated failure" >&2; exit 7; fi\nexit 0\n`);
writeFileSync(join(root,"package.json"),"{}");
writeFileSync(failFlag,"1");
const controller=new AbortController(),file=join(store,"v1","origins","g1-m99","ready.json");
const read=()=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};
const seen:string[]=[];
const watch=setInterval(()=>{const s=read();if(s){const line=`${s.state} stage=${s.stage} ${s.stageIndex}/${s.stageCount} attempts=${s.attempts} exit=${s.exit} refusal=${s.refusal?.code ?? "-"} ready=${s.ready}`;if(seen.at(-1)!==line){seen.push(line);console.log(line);if(s.refusal?.code==="stage-failed"){console.log("  en:",s.refusal.en,"\n  de:",s.refusal.de);rmFlag();}}if(s.ready)controller.abort();}},200);
function rmFlag(){try{require("node:fs").rmSync(failFlag);}catch{}}
process.env.SEMIO_TEST_FAST="1";
const timer=setTimeout(()=>{console.log("timeout");controller.abort();},120000);
await runReady({root,environment:{HOME:home,PATH:"/usr/bin:/bin",SEMIO_FLEET_ORIGIN:"g1-m99",SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_STORE_ROOT:store,SEMIO_LANGUAGE:"de"},signal:controller.signal,write:line=>console.log("[say]",line)});
clearTimeout(timer);clearInterval(watch);
console.log("marker:",readFileSync(join(root,".cache","f",P.markerName),"utf8").trim());
console.log("final:",JSON.stringify(read()).slice(0,400));
