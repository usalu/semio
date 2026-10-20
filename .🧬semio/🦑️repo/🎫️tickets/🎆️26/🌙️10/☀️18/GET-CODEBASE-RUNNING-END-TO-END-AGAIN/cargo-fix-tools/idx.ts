import { readFileSync, writeFileSync, existsSync } from "node:fs";
// 🔎 Symbol definition index of the overlay: `bun idx.ts build` then `bun idx.ts q Name...`
const R = process.env.CF_ROOT ?? "/private/tmp/cargo-fix/r";
const IDX = "/private/tmp/cargo-fix/defs.json";
const re = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(?:extern\s+"[^"]*"\s+)?(struct|enum|trait|type|fn|const|static|union|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)/;
const reAlias = /\bas\s+([A-Za-z_][A-Za-z0-9_]*)\s*[,;}]/;
if (process.argv[2] === "build") {
  const list = readFileSync(`${process.env.LIST}`, "utf8").split("\0").filter(Boolean).filter((f) => f.endsWith(".rs"));
  const idx: Record<string, string[]> = Object.create(null);
  for (const f of list) {
    let t: string;
    try { t = readFileSync(`${R}/${f}`, "utf8"); } catch { continue; }
    const ls = t.split("\n");
    for (let i = 0; i < ls.length; i++) {
      const m = re.exec(ls[i]);
      if (m) (idx[m[2]] ??= []).push(`${f}:${i + 1} ${m[1]}${/^\s*pub/.test(ls[i]) ? " pub" : ""}`);
      else if (ls[i].includes("macro_rules!")) { const mm = /macro_rules!\s+([A-Za-z_]\w*)/.exec(ls[i]); if (mm) (idx[mm[1]] ??= []).push(`${f}:${i + 1} macro`); }
    }
  }
  writeFileSync(IDX, JSON.stringify(idx));
  console.log(Object.keys(idx).length, "names");
} else {
  const idx: Record<string, string[]> = Object.assign(Object.create(null), JSON.parse(readFileSync(IDX, "utf8")));
  for (const n of process.argv.slice(3)) {
    console.log("##", n);
    for (const l of (idx[n] ?? []).slice(0, Number(process.env.N ?? 10))) console.log("  ", l.replace(/^.*?(🧰️framework|✏️s|🌎️hub|🎓️teaching)/, "$1").slice(-230));
  }
}
