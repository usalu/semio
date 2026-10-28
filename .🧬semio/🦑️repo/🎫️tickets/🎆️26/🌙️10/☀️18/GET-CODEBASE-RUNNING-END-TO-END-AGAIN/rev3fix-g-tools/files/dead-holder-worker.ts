/** 🪦️ One holder that dies: takes a pooled lane, writes a receipt of its own and is killed by SIGKILL inside its body, so its retention never runs. `<root> <shared>`. */
import {mkdirSync,writeFileSync,writeSync} from "node:fs";
import {join} from "node:path";
import {acquirePooledLaneV1} from "../../../../🏊️pool/🟦️.ts";
import {withLaneRetentionV1} from "../../../🟦️.ts";

const [root,shared]=process.argv.slice(2) as [string,string],lane=await acquirePooledLaneV1(join(root,"u"),{crates:["k"],quotaBytes:Number.MAX_SAFE_INTEGER});
await withLaneRetentionV1({lane,shared,language:"en"},async()=>{
 const ledger=join(lane.paths.build,"semio-cargo-provenance");mkdirSync(ledger,{recursive:true});
 writeFileSync(join(ledger,"cargo-unit-provenance-killed.json"),'{"killed":true}');
 writeSync(1,lane.paths.id+"\n");process.kill(process.pid,"SIGKILL");await new Promise(()=>undefined);
});
