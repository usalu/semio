/**
 * 🛂️ S3-STDIO: resolves the hand-added `patch-snapshot` leaves (gltf domain operation, bmp flat owner) through the repository
 * taxonomy's TypeScript twin of the derive's mutation source authority (`mutationOwnerIdentity`, `mutationDomainOwnersProblems`),
 * and checks each descriptor's `owner` / `semanticKind` against it. Exits 1 on any mismatch.
 *
 * Usage (repo root): bun ./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s3-stdio-leaf-authority.ts
 */
import { readFileSync } from "node:fs";
import { loadCatalogTaxonomy, mutationDomainOwnersProblems, mutationOwnerIdentity } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const taxonomy = loadCatalogTaxonomy();
const leaves = [
  { root: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations", owner: "📸️snapshot/🩹️patch" },
  { root: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations", owner: "🩹️patch-snapshot" },
];
let failures = mutationDomainOwnersProblems(taxonomy.mutationDomainOwners).length;
for (const { root, owner } of leaves) {
  const registered = Object.hasOwn(taxonomy.mutationDomainOwners, root);
  const identity = registered ? mutationOwnerIdentity(root, owner, taxonomy) : null;
  const descriptor = JSON.parse(readFileSync(`${root}/${owner}/🔣️.json`, "utf8")) as { owner: string; semanticKind: string };
  const ok = descriptor.owner === `${root}/${owner}` && descriptor.semanticKind === "patch-snapshot" && (registered ? identity === "patch-snapshot" : !owner.includes("/"));
  if (!ok) failures += 1;
  console.log(`${ok ? "ok  " : "FAIL"} ${root.split("🗿️artifacts/")[1]}/${owner} registeredRoot=${registered} identity=${identity} descriptorOwnerMatches=${descriptor.owner === `${root}/${owner}`}`);
}
process.exit(failures === 0 ? 0 : 1);
