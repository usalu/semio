import fs from "node:fs";
import path from "node:path";
const R = "/Users/ueli/Documents/semio/";
const SDK = "🧰️framework/📦️packages/🟦️typescript";
const [sub, file, emoji, src, planFile, originsFile, mode] = process.argv.slice(2);
const write = mode === "--write";
const plan = JSON.parse(fs.readFileSync(planFile, "utf8"));
const origins = JSON.parse(fs.readFileSync(originsFile, "utf8"));
const names = new Map();
for (const e of plan) for (const n of e.names) { const o = (origins[n.name] ?? "").split("#")[0]; if (o === src) names.set(n.name, names.has(n.name) ? names.get(n.name) && n.type : n.type); }
const abs = R + SDK + "/" + file;
const exists = fs.existsSync(abs);
const prior = exists ? fs.readFileSync(abs, "utf8") : "";
const already = new Set([...prior.matchAll(/export (?:type )?\{([^}]*)\}/g)].flatMap(m => m[1].split(",").map(s => s.trim()).filter(Boolean)));
const ok = [...names].filter(([n]) => !already.has(n));
const rel = path.relative(path.dirname(abs), R + src).normalize("NFC");
const spec = rel.startsWith(".") ? rel : "./" + rel;
const values = ok.filter(([, t]) => !t).map(([n]) => n).sort(), types = ok.filter(([, t]) => t).map(([n]) => n).sort();
const lines = [];
if (values.length) lines.push(`export { ${values.join(", ")} } from "${spec}";`);
if (types.length) lines.push(`export type { ${types.join(", ")} } from "${spec}";`);
console.log((exists ? "EXTEND " : "CREATE ") + file + "\n" + lines.join("\n"));
if (!lines.length) process.exit(0);
if (write) {
  const doc = `/** ${emoji} SDK entry \`@semio-tech/framework${sub.slice(1)}\`: explicit public surface of the framework module that plugins build on. */\n`;
  fs.mkdirSync(path.dirname(abs), { recursive: true });
  fs.writeFileSync(abs, exists ? prior.replace(/\n*$/, "\n") + lines.join("\n") + "\n" : doc + lines.join("\n") + "\n");
  const pkgPath = R + SDK + "/package.json"; const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
  if (!(sub in pkg.exports)) { pkg.exports[sub] = "./" + file; fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n"); console.log("EXPORT", sub); }
}
