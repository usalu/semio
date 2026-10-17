import fs from "node:fs";
import path from "node:path";
const R = "/Users/ueli/Documents/semio/";
const SDK = "🧰️framework/📦️packages/🟦️typescript";
const write = process.argv[2] === "--write";
const spec = JSON.parse(fs.readFileSync(process.env.SPEC ?? "rhu3-tools/facade-spec.json", "utf8"));
const plan = JSON.parse(fs.readFileSync(process.env.PLAN ?? "🗑️generated/rhu-plan1-all.json", "utf8"));
const org = JSON.parse(fs.readFileSync(process.env.ORG ?? "🗑️generated/rhu-origins2b.json", "utf8"));
const byTo = new Map(plan.map(o => [o.to.normalize("NFC"), o]));
const pkgPath = R + SDK + "/package.json";
let pkgText = fs.readFileSync(pkgPath, "utf8");
const pkg = JSON.parse(pkgText);
const report = [];
for (const f of spec) {
  const abs = R + SDK + "/" + f.file;
  const exists = fs.existsSync(abs);
  let text = exists ? fs.readFileSync(abs, "utf8") : "";
  const lines = [];
  for (const src of f.sources) {
    const o = byTo.get(src.normalize("NFC")); if (!o) { report.push(`${f.sub}: no usage for ${src}`); continue; }
    const have = new Set(exists ? Object.keys(org[SDK + "/" + f.file] ?? {}).filter(n => org[SDK + "/" + f.file][n] === (org[src] ?? {})[n]) : []);
    const names = o.names.filter(n => n.name !== "default" && n.name !== "*" && !have.has(n.name));
    const bad = names.filter(n => (org[src] ?? {})[n.name] === undefined);
    if (bad.length) report.push(`${f.sub}: NOT EXPORTED by ${src}: ${bad.map(b => b.name).join(",")}`);
    const ok = names.filter(n => !bad.includes(n));
    const rel = path.relative(path.dirname(abs), R + src).normalize("NFC");
    const spec2 = rel.startsWith(".") ? rel : "./" + rel;
    const values = ok.filter(n => !n.type).map(n => n.name).sort(); const types = ok.filter(n => n.type).map(n => n.name).sort();
    if (values.length) lines.push(`export { ${values.join(", ")} } from "${spec2}";`);
    if (types.length) lines.push(`export type { ${types.join(", ")} } from "${spec2}";`);
  }
  if (!lines.length) continue;
  const doc = `/** ${f.emoji} SDK entry \`@semio-tech/framework${f.sub.slice(1)}\`: explicit public surface of the framework module that plugins build on. */\n`;
  const next = exists ? text.replace(/\n*$/, "\n") + lines.join("\n") + "\n" : doc + lines.join("\n") + "\n";
  report.push(`${exists ? "EXTEND" : "CREATE"} ${f.file}\n   ${lines.join("\n   ")}`);
  if (write) { fs.mkdirSync(path.dirname(abs), { recursive: true }); fs.writeFileSync(abs, next); }
  if (!(f.sub in pkg.exports)) { pkg.exports[f.sub] = "./" + f.file; report.push(`EXPORT ${f.sub} -> ./${f.file}`); }
}
if (write) {
  const cur = fs.readFileSync(pkgPath, "utf8");
  if (cur !== pkgText) { console.error("package.json changed under us, re-reading"); const curPkg = JSON.parse(cur); for (const [k, v] of Object.entries(pkg.exports)) if (!(k in curPkg.exports)) curPkg.exports[k] = v; fs.writeFileSync(pkgPath, JSON.stringify(curPkg, null, 2) + "\n"); }
  else fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n");
}
console.log(report.join("\n"));
