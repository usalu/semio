import { scan } from "../exports-stage-tools/scan.ts";
const pkg = process.argv[2]!;
const { bare } = scan();
const by = new Map<string, string[]>();
for (const b of bare) if (b.pkg === pkg) (by.get(b.sub) ?? by.set(b.sub, []).get(b.sub)!).push(b.file);
for (const [k, v] of [...by].sort()) console.log(v.length, k, v.slice(0, 3).map(x => x.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "LIB")).join(" | "));
