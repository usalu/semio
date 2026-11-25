import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join, relative, dirname, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const tx = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const manifests = new Map<string, string>();
import { execSync } from "node:child_process";
for (const f of execSync("git ls-files -z -co --exclude-standard -- '*package.json'", { cwd: root, maxBuffer: 1 << 26 }).toString().split("\0")) { if (!f || f.includes("node_modules")) continue; const p = join(root, f); try { const n = JSON.parse(readFileSync(p, "utf8")).name; if (n && !manifests.has(n)) manifests.set(n, p); } catch {} }
const BUN = process.execPath;
if (process.argv[2] === "probe") { const mod = await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧲️registry-catalog-projection/🟦️.ts`);
try { console.log("OK", mod.registryCatalogInputPaths(root).length); } catch (e) { console.log("ERR", String((e as Error).message).split(" (imported")[0]); } process.exit(0); }
import { spawnSync } from "node:child_process";
for (let i = 0; i < 80; i++) {
  { const out = spawnSync(BUN, [process.argv[1]!, "probe"], { encoding: "utf8" }).stdout; if (out.startsWith("OK")) { console.log(out.trim()); break; } var e: any = new Error(out.replace(/^ERR /, "")); if (!out.startsWith("ERR")) { console.log("OTHER", out.slice(0, 500)); break; } }
  {
    const m = /not schema-owned: (\S+) in /.exec(String((e as Error).message));
    if (!m) { console.log("OTHER", String((e as Error).message).slice(0, 600)); break; }
    const spec = m[1]!; const [scope, name, ...sub] = spec.split("/"); const pkg = `${scope}/${name}`; console.log("pkg", pkg, manifests.has(pkg));
    const manifest = manifests.get(pkg); if (!manifest) { console.log("NOPKG", spec); break; }
    const ex = JSON.parse(readFileSync(manifest, "utf8")).exports?.[sub.length ? "./" + sub.join("/") : "."];
    if (typeof ex !== "string") { console.log("NOEXPORT", spec, ex); break; }
    const j = JSON.parse(readFileSync(tx, "utf8"));
    const w = j.generatorContracts["plugin-registry"].inputDiscovery.workspaceImports;
    w[spec] = { manifestPath: relative(root, manifest), entryPath: relative(root, resolve(dirname(manifest), ex)) };
    j.generatorContracts["plugin-registry"].inputDiscovery.workspaceImports = Object.fromEntries(Object.entries(w).sort(([a], [b]) => a < b ? -1 : 1));
    writeFileSync(tx, JSON.stringify(j, null, 2) + "\n"); console.log("added", spec);
  }
}
