// usage: bun closure.mjs <manifest...>  -> prints package names (name\tmanifest) of the path-dependency closure (normal+dev+build, workspace=true resolved)
import fs from "node:fs";
import path from "node:path";
const read = (file) => Bun.TOML.parse(fs.readFileSync(file, "utf8"));
const cache = new Map();
const manifest = (file) => { if (!cache.has(file)) cache.set(file, read(file)); return cache.get(file); };
const workspaceRoot = (file, doc) => { const w = doc.package?.workspace; const dir = path.dirname(file); return w ? path.resolve(dir, w, "Cargo.toml") : (doc.workspace ? file : null); };
const sections = (doc) => { const out = []; for (const key of ["dependencies", "dev-dependencies", "build-dependencies"]) if (doc[key]) out.push(doc[key]); for (const target of Object.values(doc.target ?? {})) for (const key of ["dependencies", "dev-dependencies", "build-dependencies"]) if (target[key]) out.push(target[key]); return out; };
const seen = new Map();
const visit = (file) => {
  file = path.resolve(file);
  if (seen.has(file)) return;
  const doc = manifest(file);
  seen.set(file, doc.package?.name ?? file);
  const root = workspaceRoot(file, doc);
  const wsdeps = root && fs.existsSync(root) ? (manifest(root).workspace?.dependencies ?? {}) : {};
  for (const section of sections(doc)) for (const [name, spec] of Object.entries(section)) {
    if (typeof spec !== "object") continue;
    let resolved = spec, base = path.dirname(file);
    if (spec.workspace === true) { resolved = wsdeps[name]; base = path.dirname(root); }
    if (resolved && typeof resolved === "object" && resolved.path) { const next = path.resolve(base, resolved.path, "Cargo.toml"); if (fs.existsSync(next)) visit(next); }
  }
};
for (const arg of process.argv.slice(2)) visit(arg);
for (const [file, name] of seen) console.log(`${name}\t${path.relative(process.cwd(), file)}`);
