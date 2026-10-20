// 🧩 Stages a scratch-host variant of the Bazel adapter next to the harness: relative specifiers re-pointed, plus an exported storage-free owner entry (the library is frozen).
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
const findRoot=(from:string):string=>{for(let at=from;;at=dirname(at)){if(existsSync(join(at,"bunfig.toml"))&&existsSync(join(at,"bun.lock")))return at;if(dirname(at)===at)throw Error("repository root not found");}};
const root=findRoot(import.meta.dir),B=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"),out=join(import.meta.dir,"staged","adapter.ts");
let text=readFileSync(join(B,"📜️script.ts"),"utf8");
text=text.replace(/(from\s*|import\()(["'])(\.{1,2}\/[^"']+)\2/gu,(_m,head,quote,path)=>{let target=relative(dirname(out),resolve(B,path));if(!target.startsWith("."))target="./"+target;return head+quote+target+quote;});
const replaceOnce=(from:string,to:string)=>{const parts=text.split(from);if(parts.length!==2)throw Error("anchor count "+(parts.length-1)+": "+from.slice(0,60));text=parts.join(to);};
replaceOnce("async function writeBazelRuntimeV1(","export async function writeBazelRuntimeV1(");
replaceOnce("async function bazelCancellationControl(","export async function bazelCancellationControl(");
text+=readFileSync(join(import.meta.dir,"scratch-host.part.ts"),"utf8");
writeFileSync(out,text);
console.log(out,text.length);
