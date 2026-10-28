import {pathToFileURL} from "node:url";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const m=await import(pathToFileURL(lib+"⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts").href);
const R=process.cwd()+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/nxperf-g/nsroot";const tool="b".repeat(64),impl="c".repeat(64),w=process.cwd();
const f=(g,i=impl,t=tool)=>m.boundedNxNamespaceV1(R,R,t,w,g,i);
const a=f("1".repeat(64)),b=f("2".repeat(64)),c=f(null),d=f("1".repeat(64),"d".repeat(64)),e=f("1".repeat(64),impl,"e".repeat(64));
const dir=x=>x.environment.NX_WORKSPACE_DATA_DIRECTORY.replace(R,"");
console.log({graphA:dir(a),graphB:dir(b),noGraph:dir(c),otherImpl:dir(d),otherTool:dir(e)});
