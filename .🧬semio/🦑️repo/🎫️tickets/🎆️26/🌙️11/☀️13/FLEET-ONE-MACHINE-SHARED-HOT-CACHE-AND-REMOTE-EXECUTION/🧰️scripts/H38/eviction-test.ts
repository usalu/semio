/** ⏱️ Contract: an inline optional-cache eviction ends with its lease control or after the declared budget (schema `Protocol.inlineEvictionMilliseconds`), whichever comes first, so a reader that opens the graph store never waits for a cleanup of minutes. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {optionalEvictionCancelV1} from "../../🟨️.mjs";

const schema=JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)),"../../🧬️schema/🔣️.json"),"utf8"));

test("the declared budget is five seconds and the cancellation flips exactly after it",()=>{
 expect(schema.definitions.OptionalQuota.definitions.Protocol.const.inlineEvictionMilliseconds).toBe(5000);
 let clock=1000;const control={signal:new AbortController().signal},cancelled=optionalEvictionCancelV1(control,{now:()=>clock});
 expect(cancelled()).toBe(false);clock=1000+5000;expect(cancelled()).toBe(false);clock=1000+5001;expect(cancelled()).toBe(true);
});

test("the lease control cancels at once and a custom budget is honoured",()=>{
 const controller=new AbortController();let clock=0;const cancelled=optionalEvictionCancelV1({signal:controller.signal},{budgetMilliseconds:100,now:()=>clock});
 expect(cancelled()).toBe(false);controller.abort();expect(cancelled()).toBe(true);
 const timed=optionalEvictionCancelV1({signal:new AbortController().signal},{budgetMilliseconds:100,now:()=>clock});clock=101;expect(timed()).toBe(true);
});
