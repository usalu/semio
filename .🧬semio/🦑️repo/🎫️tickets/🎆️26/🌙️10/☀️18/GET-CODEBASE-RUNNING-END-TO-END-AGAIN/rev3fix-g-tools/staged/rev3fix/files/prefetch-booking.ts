/** 🎟️ Contract: a prefetching run asks for the capacity of its rebuild before it queues for anything, and books the learned size of the rebuild only once it holds a claim to compute. The upgrade is a wait in the machine-wide queue that is cheap to hold; the booking is a reservation that every other run counts, so a prefetcher that waits for a compute slot must not hold it (with several waiting prefetchers the ledger went negative and the farm request of the very same run could not fit beside its own parent's booking). */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
const method=source.slice(source.indexOf("async prefetch(request:GenerationPrefetchRequestV1)"),source.indexOf("\n",source.indexOf("async prefetch(request:GenerationPrefetchRequestV1)")));
const prefetcher=source.slice(source.indexOf("async function prefetchStaleGenerationV1("),source.indexOf("export function edgeRefreshOwnersV1("));

test("the prefetch of a graph asks for its capacity first and hands the booking to the claimed computation, never booking before the slots",()=>{
 expect(source,"the upgrade before the custody (rev3-g/upgrade-before-custody) lands first").toContain("import{markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1}from");
 expect(method).toContain("await upgradeBuildCapacityBeforeRebuildV1();");
 expect(method).toContain('book:()=>markBuildCapacityPhaseV1("rebuild")');
 expect(method.indexOf("upgradeBuildCapacityBeforeRebuildV1")).toBeLessThan(method.indexOf("prefetchStaleGenerationV1"));
 expect(method.match(/markBuildCapacityPhaseV1\(/gu)?.length).toBe(1);
});

test("the booking happens inside the computation of a claimed chunk, once, before the first project is inferred",()=>{
 const claim=prefetcher.indexOf("await prepareStaleProjectsV1("),run=prefetcher.indexOf("run:async owners=>",claim),book=prefetcher.indexOf("await input.book()",run),infer=prefetcher.indexOf("await prefetchBuildGraphProjectsV1(",run);
 expect(claim).toBeGreaterThan(0);expect(run).toBeGreaterThan(claim);expect(book).toBeGreaterThan(run);expect(infer).toBeGreaterThan(book);
 expect(prefetcher).toContain("if(!booked){booked=true;await input.book();}");
 expect(prefetcher.match(/input\.book\(/gu)?.length).toBe(1);
 expect(prefetcher.slice(0,claim)).not.toContain("input.book(");
});
