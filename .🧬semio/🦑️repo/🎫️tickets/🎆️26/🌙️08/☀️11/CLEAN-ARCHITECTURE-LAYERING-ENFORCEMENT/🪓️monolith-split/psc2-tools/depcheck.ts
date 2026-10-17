/** 🔗️ For every `workspace:*` dependency this slice added, checks that each Bun scope that installs the depending package also lists the dependency's package. Usage: bun depcheck.ts <snapshot-dir> */
import { readFileSync, readdirSync } from "node:fs";
import { join, normalize } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const nfc = (s: string) => s.normalize("NFC");
const SCOPES = [".", "✏️s", "🌎️hub", "♻️mit-bestand", "🏢️semio-tech", "🎓️teaching"];
const members = new Map<string, Set<string>>();
for (const s of SCOPES) {
  const doc = JSON.parse(readFileSync(join(ROOT, s, "package.json"), "utf8"));
  const list: string[] = Array.isArray(doc.workspaces) ? doc.workspaces : doc.workspaces?.packages ?? [];
  const set = new Set(list.map(w => nfc(normalize(join(s, w)))));
  set.add(s === "." ? "." : nfc(s));
  members.set(s, set);
}
const nameToDir = new Map<string, string>();
for (const set of members.values()) for (const d of set) { try { const m = JSON.parse(readFileSync(join(ROOT, d, "package.json"), "utf8")); nameToDir.set(m.name, nfc(normalize(d))); } catch {} }
let bad = 0, checked = 0;
for (const name of readdirSync(process.argv[2])) {
  if (!name.endsWith(".json") || name.includes(".report.") || name.includes("-apply")) continue;
  const snap: Record<string, string | null> = JSON.parse(readFileSync(join(process.argv[2], name), "utf8"));
  for (const [f, before] of Object.entries(snap)) {
    if (!f.endsWith("/package.json") || before === null) continue;
    const a = JSON.parse(before), b = JSON.parse(readFileSync(join(ROOT, f), "utf8"));
    const dir = nfc(f.slice(0, -"/package.json".length));
    for (const key of ["dependencies", "devDependencies"]) for (const [dep, v] of Object.entries<string>(b[key] ?? {})) {
      if (v !== "workspace:*" || a[key]?.[dep] === v) continue;
      checked++;
      const target = nameToDir.get(dep);
      for (const [s, set] of members) {
        if (!set.has(dir)) continue;
        if (!target || !set.has(target)) { bad++; console.log("UNRESOLVABLE", s.padEnd(14), dir.slice(-70), "->", dep); }
      }
    }
  }
}
console.log({ checked, bad });
