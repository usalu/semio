// 📮️ Publishes the canonical members/exclude of the given artifact workspaces through the repository's own publisher (strict: authored literal exclusions are replaced by the patterns plus child workspaces).
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const lib = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts"));
const owners = lib.discoverCargoWorkspaces(ROOT);
let wrote = 0;
for (const artifact of process.argv.slice(2)) {
  const owner = owners.find((o: any) => o.manifest === `${artifact}/Cargo.toml`);
  if (!owner) throw new Error(`not admitted: ${artifact}`);
  const changed = lib.publishSelectedCargoWorkspaceMembershipV1(ROOT, owner, "write", owners, false);
  if (changed) wrote++;
  console.log(artifact.split("/").slice(-1)[0], changed ? "published" : "current");
}
console.log("published", wrote);
