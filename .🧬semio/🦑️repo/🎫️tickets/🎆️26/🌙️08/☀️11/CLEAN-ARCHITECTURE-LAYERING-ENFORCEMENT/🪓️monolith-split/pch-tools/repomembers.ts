// 🧮️ Read-only: compares an artifact workspace's declared cargo members with the repo's own discovered members (member-manifests minus exclude-patterns).
import {readFileSync} from "node:fs";
import {relative, resolve, join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const W = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts");
const {cargoWorkspaceForManifest, cargoWorkspaceMembers} = await import(W);
for (const artifact of process.argv.slice(2)) {
  const scope = cargoWorkspaceForManifest(ROOT, `${artifact}/📦️packages/🦀️rust/Cargo.toml`);
  const discovered = cargoWorkspaceMembers(ROOT, scope).map((r: any) => relative(resolve(ROOT, scope.directory), resolve(ROOT, r.directory)) || ".").sort();
  const doc = Bun.TOML.parse(readFileSync(join(ROOT, scope.manifest), "utf8")) as any;
  const declared = [...(doc.workspace.members ?? [])].sort();
  const onlyDeclared = declared.filter((d: string) => !discovered.includes(d));
  const onlyDiscovered = discovered.filter((d: string) => !declared.includes(d));
  console.log(JSON.stringify({artifact, ws: scope.manifest, declared: declared.length, discovered: discovered.length, onlyDeclared: onlyDeclared.length, onlyDiscovered, sampleOnlyDeclared: onlyDeclared.slice(0, 2)}));
}
