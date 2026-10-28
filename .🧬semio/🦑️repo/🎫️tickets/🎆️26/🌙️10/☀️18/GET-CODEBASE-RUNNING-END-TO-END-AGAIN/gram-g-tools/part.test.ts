// [gram-g] runs full.mjs inside the bun test runner so the build test invocation (an issued Original) exists; PART=1 only.
import{test}from"bun:test";
test("graph part harness",async()=>{await import("./full.mjs");await new Promise(()=>{});},7_200_000);
