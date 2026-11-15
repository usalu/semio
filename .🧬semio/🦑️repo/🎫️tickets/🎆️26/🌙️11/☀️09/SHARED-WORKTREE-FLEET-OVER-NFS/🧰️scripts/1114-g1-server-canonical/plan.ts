import {writeFileSync} from "node:fs";
import {planAdoptV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧲️adopt/🟦️.ts";
const started=Date.now();
const plan=await planAdoptV1({worktree:process.argv[2]!,storeRoot:"/Users/Shared/semio-store",originId:"g1-server",groupId:"g1"},{sample:100000});
writeFileSync(process.argv[3]!,JSON.stringify(plan,null,1));
console.error("finished in ms",Date.now()-started,JSON.stringify(plan.counts),JSON.stringify(plan.git),JSON.stringify(plan.evidence));
