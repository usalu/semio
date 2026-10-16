import fs from "node:fs";
import path from "node:path";
import cp from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const R = `${root}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`;
const G = `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated`;
const d = JSON.parse(fs.readFileSync(`${G}/rtt-folders.json`, "utf8"));
const files = [];
for (const n of d.pr_ec) for (const e of ["🟦️.ts", "🟦️.tsx"]) { const p = `${R}/🧪️tests/${n}/${e}`; if (fs.existsSync(p)) files.push(p); }
for (const n of d.tw) for (const e of ["🟦️.ts", "🟦️.tsx"]) { const p = `${R}/🧱️elements/📃️UiDocumentStore/🧪️tests/${n}/${e}`; if (fs.existsSync(p)) files.push(p); }
const targets = new Map();
for (const f of files) {
  const t = fs.readFileSync(f, "utf8");
  const re = /["'`]((?:\.\.\/)+[^"'`]+)["'`]/g;
  let m;
  while ((m = re.exec(t))) {
    const abs = path.resolve(path.dirname(f), m[1].replace(/\?raw$/, ""));
    if (abs.startsWith(R)) continue;
    const rel = path.relative(root, abs);
    if (!/🧫️fixtures/.test(rel)) continue;
    (targets.get(rel) ?? targets.set(rel, new Set()).get(rel)).add(path.basename(path.dirname(f)));
  }
}
const own = (h) => /📺️renderer\/🧑‍🎨engine\/(🧪️tests|🧱️elements\/📃️UiDocumentStore\/🧪️tests)\//.test(h);
const rows = [];
for (const [rel, users] of [...targets].sort()) {
  const segs = rel.split("/");
  const key = segs.slice(-3).join("/");
  let out = "";
  try { out = cp.execSync(`git -c core.quotepath=false grep -l --untracked -F -e '${key}' -- '🧰️framework' '✏️s' '🌎️hub' '.vscode' 2>/dev/null || true`, { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }); } catch (e) { out = e.stdout ?? ""; }
  const hits = out.split("\n").filter(Boolean).filter((h) => !own(h) && !h.endsWith("taxonomy.json") && !h.endsWith(rel));
  rows.push([hits.length, rel.replace("🧰️framework/", "F/").replace("🛍️products/💻️os/🔨️modules/", "OSM/"), [...users].join(","), hits.slice(0, 2).map((x) => x.slice(-60)).join(" | ")]);
}
rows.sort((a, b) => a[0] - b[0]);
for (const r of rows) console.log(r.join("\t"));
