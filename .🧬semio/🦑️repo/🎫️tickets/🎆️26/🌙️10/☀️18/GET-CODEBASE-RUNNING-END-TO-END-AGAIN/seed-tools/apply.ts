import { createHash } from "node:crypto";
import { copyFileSync, readFileSync, renameSync, writeFileSync } from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const OUT = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/seed-run/";
const hash = (p: string) => createHash("sha256").update(readFileSync(p, "utf8")).digest("hex").slice(0, 16);
const log = JSON.parse(readFileSync(OUT + "fold-log.json", "utf8"));
const launch = R + ".vscode/launch.json", seed = R + ".vscode/🧩️launch.seed.jsonc";
if (hash(launch) !== log.sources.launch || hash(seed) !== log.sources.seed) { console.error("ABORT: .vscode files changed since the fold; rerun fold.ts", hash(launch), log.sources.launch, hash(seed), log.sources.seed); process.exit(2); }
copyFileSync(launch, OUT + "old-launch.json");
copyFileSync(seed, OUT + "old-seed.jsonc");
writeFileSync(seed + ".tmp", readFileSync(OUT + "new-seed.jsonc", "utf8"));
writeFileSync(launch + ".tmp", readFileSync(OUT + "new-launch.json", "utf8"));
renameSync(seed + ".tmp", seed);
renameSync(launch + ".tmp", launch);
console.log("applied");
