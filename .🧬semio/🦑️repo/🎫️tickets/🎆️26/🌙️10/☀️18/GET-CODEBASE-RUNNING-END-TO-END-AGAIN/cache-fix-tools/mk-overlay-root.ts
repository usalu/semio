/** 🧪 Builds a scratch workspace of COPIES (every tracked manifest, nx.json, the taxonomy, the staged policy) to run the law against the post-apply state. */
import { copyFileSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const WS = resolve(import.meta.dir, "../../../../../../../.."), C = join(import.meta.dir, "..");
const out = join(C, "🗑️generated/cachefix-ovroot");
rmSync(out, { recursive: true, force: true });
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const copy = (from: string, to: string) => { mkdirSync(dirname(join(out, to)), { recursive: true }); copyFileSync(from, join(out, to)); };
const list = readFileSync(join(C, "🗑️generated/cachefix-x/pj-list.txt"), "utf8").split("\n").filter(Boolean);
for (const file of list) copy(join(WS, file), file);
copy(join(WS, "nx.json"), "nx.json"); copy(join(WS, L, "🔣️taxonomy.json"), `${L}/🔣️taxonomy.json`);
copy(join(import.meta.dir, "staged", L, "⚡️caching/🔣️policy.json"), `${L}/⚡️caching/🔣️policy.json`);
console.log("overlay root", out, list.length, "manifests");
