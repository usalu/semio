/** 🚧️ Areas owned by running slices or by the parent. Edges whose importer lies here are planned (so the owners can switch later) but never rewritten. */
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { nfc } from "./model.ts";

const HERE = fileURLToPath(new URL(".", import.meta.url));
const GENERATED = join(HERE, "../🗑️generated");

export const SKIP_PREFIXES: { prefix: string; slice: string }[] = [
  { prefix: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", slice: "rhu (renderer TS)" },
  { prefix: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/", slice: "pch (stdio artifacts)" },
  { prefix: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", slice: "parent (📚️library)" },
  { prefix: "🌎️hub/📦️packages/🦀️rust/📜️script.ts", slice: "hss (hub script)" },
].map(x => ({ prefix: nfc(x.prefix), slice: x.slice }));

let hss: Set<string> | null = null;
function hssFiles(): Set<string> {
  if (hss) return hss;
  hss = new Set();
  const p = join(GENERATED, "hss-out/report.tsv");
  if (existsSync(p)) for (const line of readFileSync(p, "utf8").split("\n")) { const c = line.split("\t")[2]; if (c) hss.add(nfc(c)); }
  return hss;
}

let ts3: string[] | null = null;
function ts3Prefixes(): string[] {
  if (ts3) return ts3;
  ts3 = [];
  const dir = join(HERE, "../ts3-tools");
  if (!existsSync(dir)) return ts3;
  for (const name of readdirSync(dir)) {
    if (!name.endsWith(".spec.json")) continue;
    let spec: any;
    try { spec = JSON.parse(readFileSync(join(dir, name), "utf8")); } catch { continue; }
    if (typeof spec.file === "string") ts3.push(nfc(spec.file));
    if (typeof spec.childRoot === "string") for (const c of spec.children ?? []) if (typeof c.folder === "string") ts3.push(nfc(spec.childRoot + "/" + c.folder + "/"));
  }
  return ts3;
}

export function skipReason(file: string): string | null {
  const f = nfc(file);
  for (const s of SKIP_PREFIXES) if (f.startsWith(s.prefix)) return s.slice;
  if (hssFiles().has(f)) return "hss (hub script child)";
  return null;
}
