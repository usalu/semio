import fs from "node:fs";
import path from "node:path";
process.env.SPLIT_SOURCE = process.argv[3];
const { run } = await import("./split.mjs");
const { glueDir } = await import(process.argv[2]);
const res = await run(process.argv[2]);
const { a, modUnits, modules, P } = res;
const rel = (to) => { let r = path.relative(glueDir, to); if (!r.startsWith(".")) r = "./" + r; return r; };
const blocks = [];
const publicNames = new Map();
for (const m of modules) {
  const us = modUnits.get(m.key).filter((u) => u.exported);
  if (!us.length) continue;
  const seen = new Set();
  const values = [], types = [];
  for (const u of us) for (const n of u.names) {
    if (seen.has(n)) continue;
    seen.add(n);
    const root = a.find(u.idx);
    const typeOnly = a.units.filter((x) => a.find(x.idx) === root).every((x) => x.isType);
    (typeOnly ? types : values).push(n);
    publicNames.set(n, m.key);
  }
  const spec = JSON.stringify(rel(res.fileOf(m.key)));
  const fmt = (kw, xs) => (xs.length ? `export ${kw}{ ${xs.sort((x, y) => x.localeCompare(y)).join(", ")} } from ${spec};` : null);
  const lines = [fmt("", values), fmt("type ", types)].filter(Boolean);
  const wrapped = lines.map((l) => (l.length > 170 ? l.replace(/\{ (.*) \}/, (_, body) => `{\n${body.split(", ").map((b) => "  " + b).join(",\n")}\n}`) : l));
  blocks.push(wrapped.join("\n"));
}
const kept = [];
for (const u of modUnits.get(res.keep)) {
  if (!u.exportDecl) continue;
  const text = a.text.slice(res.spans[u.idx].full, res.spans[u.idx].end).replace(/^\s+/, "");
  kept.push(text.replace(/from "\.\//g, 'from "../../'));
}
const header = `/** 📦️ \`@semio-tech/framework-kernel\` — the package entry of the \`🎠️kernel\` module: plugin runtime, leases, invocation responses, and playground boot. It lists the public names of the kernel children (each child owns one concern) and re-exports the modules the kernel has always fronted; domain code lives in the children, never here. */`;
const out = [header, "", "//#region 🎠️Kernel", ...blocks, "", ...kept, "//#endregion 🎠️Kernel", ""].join("\n");
fs.writeFileSync(process.argv[4], out);
fs.writeFileSync(process.argv[4] + ".names.json", JSON.stringify([...publicNames.keys()].sort()));
console.log("glue lines", out.split("\n").length, "names", publicNames.size);
