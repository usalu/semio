import {test,expect,afterAll} from "bun:test";
import {AsyncLocalStorage} from "node:async_hooks";
let run:any;
test("capture first callback context",async()=>{run=AsyncLocalStorage.snapshot();expect(true).toBe(true);});
test("register end hook from first callback context",()=>run(async()=>{afterAll(()=>{});expect(true).toBe(true);}),2000);
