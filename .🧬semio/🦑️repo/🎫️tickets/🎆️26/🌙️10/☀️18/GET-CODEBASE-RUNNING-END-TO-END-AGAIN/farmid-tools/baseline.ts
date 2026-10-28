// [farmid] RSS / heap of a farm worker after its session opened (library bootstrap + inference environment), physical mode, read-only: bun baseline.ts <root>. Not a library file.
import { heapStats } from "bun:jsc";
const root = process.argv[2], B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/", mb = (n: number) => Math.round(n / 1048576);
const service = await import(B + "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs"), nodes = await import(B + "🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
const { mkdtempSync } = await import("node:fs"), { tmpdir } = await import("node:os"), { join } = await import("node:path");
const before = process.memoryUsage().rss;
await service.open({ version: 3, workspaceRoot: root, registry: null, contracts: nodes.projectGeneratorContractsV1(root), commandInputs: null, lateClosures: false, projectsByRoot: [], componentRoots: [], stagingBase: mkdtempSync(join(tmpdir(), "fi-base-")) }, { slot: 0, generation: 1 });
const open = process.memoryUsage().rss; Bun.gc(true); const h = heapStats();
console.log(JSON.stringify({ rssBeforeOpenMB: mb(before), rssAfterOpenMB: mb(open), liveHeapMB: mb(h.heapSize + h.extraMemorySize) }));
process.exit(0);
