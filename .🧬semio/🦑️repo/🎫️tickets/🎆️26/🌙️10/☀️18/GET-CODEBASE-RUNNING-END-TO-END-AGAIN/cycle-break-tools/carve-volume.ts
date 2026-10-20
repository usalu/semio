import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { model } from "./lib.ts";
const G = "🧰️framework/📦️packages/🟦️typescript";
const fw = model.byName.get("@semio-tech/framework")!;
const { bare } = scan();
const dirOfKey = (sub: string): string => { const v = fw.exports[sub]; const t = typeof v === "string" ? v : ""; const m = /^\.\/(.+)$/u.exec(t); return m ? m[1]! : ""; };
const by = new Map<string, { importers: Set<string>; keys: Set<string> }>();
for (const b of bare) { if (b.pkg !== "@semio-tech/framework") continue; const d = dirOfKey(b.sub); const top = d.split("/").slice(0, d.startsWith("🛍️products") ? 4 : 1).join("/") || "(root/unknown " + b.sub + ")"; const r = by.get(top) ?? by.set(top, { importers: new Set(), keys: new Set() }).get(top)!; r.importers.add(b.file); r.keys.add(b.sub); }
for (const [k, v] of [...by].sort((a, b) => b[1].importers.size - a[1].importers.size)) console.log(String(v.importers.size).padStart(5), "importers", String(v.keys.size).padStart(3), "keys", k);
void readFileSync; void resolve;
