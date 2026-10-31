import {test,expect,afterAll} from "bun:test";
test("first",async()=>{expect(true).toBe(true)});
test("second",async()=>{await Promise.resolve();afterAll(()=>{});expect(true).toBe(true)},2000);
