import { discoverCargoWorkspaces, cargoWorkspaceMembers } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
const root = process.cwd(), name = "semio-s-artifact-gis-gismap";
const scopes = discoverCargoWorkspaces(root);
console.log("[DEBUG] scopes", scopes.length);
for (const scope of scopes) {
  try {
    const hits = cargoWorkspaceMembers(root, scope).filter(pkg => pkg.name === name || pkg.name.startsWith(name));
    if (scope.directory === "." || hits.length) console.log("[DEBUG]", scope.directory, hits.map(h => h.name + " " + h.manifest));
  } catch (error) { console.log("[DEBUG] FAIL", scope.directory, String(error).slice(0, 300)); }
}
