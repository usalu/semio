import{expect,test}from"bun:test";import{active}from"./lifetime.ts";test("second",()=>{console.log("[DEBUG] second",active());expect(active()).toBe(true);});
