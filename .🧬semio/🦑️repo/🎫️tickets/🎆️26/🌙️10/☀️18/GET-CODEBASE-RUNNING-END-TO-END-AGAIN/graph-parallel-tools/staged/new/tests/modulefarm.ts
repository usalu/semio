import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";

const source=readFileSync(fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),"utf8");

test("the dependency pass prepares the parser records before its own loop reads them and never changes what that loop computes",()=>{
 const prefill=source.indexOf("await prefillModuleAnalysesV1("),loop=source.indexOf("await Promise.all(Array.from({length:Math.min(32,jobs.length)}");
 expect(prefill).toBeGreaterThan(0);expect(loop).toBeGreaterThan(prefill);
 expect(source.match(/prefillModuleAnalysesV1\(/gu)?.length).toBe(1);
 expect(source.match(/observedSourceAnalysisAsyncV1\(/gu)?.length).toBe(1);
});
