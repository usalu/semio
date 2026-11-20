#!/usr/bin/env repo
import { resolve } from "node:path";
import { statfsSync } from "node:fs";
const root = resolve(import.meta.dirname, "../../../../../../..");
const store = resolve(root, ".cache/b/r/m");
const { collectComponentRunsV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🟦️.ts"));
const free = () => { const volume = statfsSync(store); return volume.bavail * volume.bsize; };
const target = 18 * 1024 ** 3;
await collectComponentRunsV1(store, {
  dryRun: false,
  needBytes: () => Math.max(0, target - free()),
  satisfied: () => free() >= target,
  report: { language: "en", write: (line: string) => console.log(line) },
});
