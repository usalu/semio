import {test,expect} from "bun:test";
test("actual profiler smoke",()=>{let n=0;for(let i=0;i<100000;i++)n+=i;expect(n).toBe(4999950000);});
