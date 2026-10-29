import{expect,test}from"bun:test";import{active}from"./lifetime.ts";test("first",()=>{console.log("[DEBUG] first",active());expect(active()).toBe(true);});
