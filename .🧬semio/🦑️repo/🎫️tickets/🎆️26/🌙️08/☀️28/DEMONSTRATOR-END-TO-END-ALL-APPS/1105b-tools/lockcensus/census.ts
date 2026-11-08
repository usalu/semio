/** 🧮 Lists path dependencies whose key or package name differs from the target manifest's package name. */
import { readdirSync, readFileSync, realpathSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = process.argv[2]!;
const skip = new Set(["node_modules", "target", "dist", ".cache", ".git", ".🧬semio", "🗑️generated"]);
const manifests: string[] = [];
const walk = (d: string) => {
  for (const e of readdirSync(d, { withFileTypes: true })) {
    if (e.isDirectory() && !skip.has(e.name)) walk(join(d, e.name));
    else if (e.name === "Cargo.toml") manifests.push(join(d, e.name));
  }
};
walk(root);
const docs = new Map<string, any>();
for (const m of manifests) {
  try { docs.set(realpathSync(m), (Bun as any).TOML.parse(readFileSync(m, "utf8"))); }
  catch (e) { console.log(`UNPARSABLE ${m}: ${e}`); }
}
const kinds = ["dependencies", "dev-dependencies", "build-dependencies"];
let bad = 0;
for (const [m, doc] of docs) {
  const tabs = [...kinds.map((k) => doc[k]), doc.workspace?.dependencies,
    ...Object.values(doc.target ?? {}).flatMap((t: any) => kinds.map((k) => t[k]))];
  for (const tab of tabs) for (const [key, spec] of Object.entries<any>(tab ?? {})) {
    if (typeof spec !== "object" || !spec.path) continue;
    const t = resolve(dirname(m), spec.path, "Cargo.toml");
    if (!existsSync(t)) { console.log(`MISSING ${m}: ${key} -> ${spec.path}`); bad++; continue; }
    const got = docs.get(realpathSync(t))?.package?.name;
    if (got !== (spec.package ?? key)) { console.log(`NAME ${m}: ${key} wants ${spec.package ?? key}, target is ${got}`); bad++; }
  }
}
console.log(`manifests=${docs.size} bad=${bad}`);
