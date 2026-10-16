// 📬️ Read-only: shows how the repository's membership publisher would differ from the authored members/exclude of artifact workspaces.
import {readFileSync} from "node:fs";
import {join, relative, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const lib = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts"));
const owners = lib.discoverCargoWorkspaces(ROOT);
for (const artifact of process.argv.slice(2)) {
  const owner = owners.find((o: any) => o.manifest === `${artifact}/Cargo.toml`);
  if (!owner) { console.log(artifact, "NOT ADMITTED"); continue; }
  const doc = Bun.TOML.parse(readFileSync(join(ROOT, owner.manifest), "utf8")) as any;
  const members = lib.cargoWorkspaceMembers(ROOT, owner).map((r: any) => relative(resolve(ROOT, owner.directory), resolve(ROOT, r.directory)) || ".");
  const children = owners.filter((r: any) => r.directory !== owner.directory && r.directory.startsWith(owner.directory + "/")).map((r: any) => relative(resolve(ROOT, owner.directory), resolve(ROOT, r.directory)));
  const exclude = [...new Set([...owner.contribution.exclude, ...children])].sort();
  const declared: string[] = doc.workspace.members ?? [];
  const dExclude: string[] = doc.workspace.exclude ?? [];
  const miss = members.filter((m: string) => !declared.includes(m)), extra = declared.filter((m: string) => !members.includes(m));
  const exMiss = exclude.filter((m: string) => !dExclude.includes(m)), exExtra = dExclude.filter((m: string) => !exclude.includes(m));
  console.log(JSON.stringify({a: artifact.split("/").slice(-1)[0], members: members.length, declared: declared.length, missingInDeclared: miss, extraDeclared: extra, membersSorted: JSON.stringify(declared) === JSON.stringify([...members].sort()), excludeMissing: exMiss, excludeExtra: exExtra, excludeSorted: JSON.stringify(dExclude) === JSON.stringify([...exclude].sort())}));
}
