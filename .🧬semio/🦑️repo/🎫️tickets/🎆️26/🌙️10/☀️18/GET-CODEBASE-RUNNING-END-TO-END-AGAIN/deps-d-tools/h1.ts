import {readJavascriptLockScopesV1,checkJavascriptLockScopesV1,requireJavascriptLocksInSyncV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔒️lock-sync/🟦️.ts";
let t=performance.now();const scopes=readJavascriptLockScopesV1(process.cwd());console.log("read",(performance.now()-t).toFixed(0),"ms",scopes.map(s=>s.scope+":"+s.manifests.size).join(" "));
t=performance.now();const reports=checkJavascriptLockScopesV1(scopes);console.log("check",(performance.now()-t).toFixed(0),"ms",reports.map(r=>r.scope+":"+r.drift.length).join(" "));
t=performance.now();requireJavascriptLocksInSyncV1(process.cwd(),undefined,process.env);console.log("require ok",(performance.now()-t).toFixed(0),"ms");
