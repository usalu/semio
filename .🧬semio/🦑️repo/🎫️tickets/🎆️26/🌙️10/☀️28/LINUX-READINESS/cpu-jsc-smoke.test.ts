import {test,expect,afterAll} from "bun:test";
import {profile} from "bun:jsc";
import {writeFileSync} from "node:fs";
let finish!:()=>void;const held=new Promise<void>(resolve=>finish=resolve),observed=profile(()=>held,10000);
test("actual JSC sampling smoke",()=>{let n=0;const until=performance.now()+50;while(performance.now()<until)n++;expect(n).toBeGreaterThan(0);});
afterAll(async()=>{finish();const result=await observed;writeFileSync(new URL("./🗑️generated/cargo-cpu/jsc-smoke.json",import.meta.url),JSON.stringify(result));console.log("[DEBUG] JSC profile "+JSON.stringify({keys:Object.keys(result),functions:result.functions?.length}));});
