import {test,beforeAll,expect} from "bun:test";
import {AsyncLocalStorage} from "node:async_hooks";
const owned=new AsyncLocalStorage<string>();
let enter!:<T>(run:()=>T)=>T;
beforeAll(()=>owned.run("held",async()=>{await Promise.resolve();enter=AsyncLocalStorage.snapshot();}));
test("plain asynchronous control",async()=>{await new Promise(resolve=>setImmediate(resolve));expect(true).toBe(true);},2000);
test("snapshot captured in beforeAll",async()=>enter(async()=>{expect(owned.getStore()).toBe("held");await new Promise(resolve=>setImmediate(resolve));expect(true).toBe(true);}),2000);
