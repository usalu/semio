/** 🔎️ Compares the manifests that gtk-regen.ts wrote below /Users/ueli/.cache/gtk/<tag>/gen with the live manifests, keeping only the wiring rows (features, sibling dependency rows, optional flags, package names); external dependency path drift is counted apart. Read-only. Usage: bun gtk-compare.ts <tag>... */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const walk = (directory: string, visit: (path: string) => void): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) walk(path, visit); else visit(path);
  }
};
const flat = (value: unknown): Map<string, string> => {
  const map = new Map<string, string>();
  const visit = (node: unknown, path: string): void => {
    if (Array.isArray(node) && node.every((entry) => typeof entry === "string")) { map.set(path, JSON.stringify([...node].sort())); return; }
    if (node && typeof node === "object" && !Array.isArray(node)) { for (const [key, child] of Object.entries(node)) visit(child, `${path}/${key}`); return; }
    map.set(path, JSON.stringify(node));
  };
  visit(value, "");
  return map;
};

for (const tag of process.argv.slice(2)) {
  const gen = join("/Users/ueli/.cache/gtk", tag, "gen");
  if (!existsSync(gen)) { console.log(`${tag}: no generated files`); continue; }
  const manifests: string[] = [];
  walk(gen, (path) => { if (path.endsWith("/Cargo.toml")) manifests.push(path); });
  let facadeName = "";
  for (const path of manifests) { const toml = Bun.TOML.parse(readFileSync(path, "utf8")) as any; if (toml.package?.metadata?.semio?.role === "artifact") facadeName = toml.package.name; }
  const tally = { crates: 0, equal: 0, wiringDiff: 0, driftOnly: 0, missingLive: 0 };
  const rows: string[] = [];
  for (const path of manifests) {
    const rel = relative(gen, path);
    if (rel.endsWith("/Cargo.toml") && rel.split("/").length < 3) continue;
    const generated = Bun.TOML.parse(readFileSync(path, "utf8")) as any;
    if (!generated.package || (generated.workspace && !generated.package)) continue;
    tally.crates += 1;
    const live = join(repo, rel);
    if (!existsSync(live)) { tally.missingLive += 1; rows.push(`MISSING-LIVE ${generated.package.name}`); continue; }
    const a = flat(Bun.TOML.parse(readFileSync(live, "utf8")));
    const b = flat(generated);
    const keys = [...new Set([...a.keys(), ...b.keys()])].filter((key) => a.get(key) !== b.get(key));
    const wiring = keys.filter((key) => {
      if (/^\/(dependencies|dev-dependencies)\/([^/]+)\//u.test(key)) {
        const name = /^\/(?:dependencies|dev-dependencies)\/([^/]+)\//u.exec(key)![1]!;
        return (name.startsWith(`${facadeName}-`) && !/\/path$/u.test(key)) || /\/optional$/u.test(key) || (name.startsWith(`${facadeName}-`) && /\/path$/u.test(key) && (a.get(key) === undefined) !== (b.get(key) === undefined));
      }
      return key.startsWith("/features") || key.startsWith("/package/name") || key.startsWith("/package/description");
    });
    if (!keys.length) tally.equal += 1;
    else if (!wiring.length) tally.driftOnly += 1;
    else { tally.wiringDiff += 1; rows.push(`${generated.package.name.replace(facadeName, "~")}: ${wiring.slice(0, 4).map((key) => `${key} live=${a.get(key) ?? "-"} gen=${b.get(key) ?? "-"}`.slice(0, 220)).join(" | ")}`); }
  }
  console.log(`${tag}\tcrates ${tally.crates}\tequal ${tally.equal}\tdrift-only ${tally.driftOnly}\twiring-diff ${tally.wiringDiff}\tmissing-live ${tally.missingLive}`);
  for (const row of rows.slice(0, 12)) console.log(`   ${row}`);
}
