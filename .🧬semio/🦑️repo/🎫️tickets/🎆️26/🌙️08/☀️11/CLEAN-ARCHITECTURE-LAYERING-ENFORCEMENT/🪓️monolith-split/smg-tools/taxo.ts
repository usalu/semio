// 🗂️ Resolves the directory chain of each given path against the live taxonomy with the real matcher, and reports unresolved or ambiguous segments.
import {loadTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts";
import {matchDirectoryKind} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
const taxonomy = loadTaxonomy({repoRoot: "/Users/ueli/Documents/semio"});
for (const path of process.argv.slice(2)) {
  let parent: string | undefined;
  const ancestors: string[] = [];
  const trail: string[] = [];
  for (const segment of path.split("/")) {
    const match = matchDirectoryKind(segment, taxonomy, parent, ancestors);
    trail.push(`${segment}=${match.kind?.id ?? "UNRESOLVED"}${match.ambiguous.length ? " AMBIGUOUS" : ""}`);
    if (match.kind) { ancestors.push(match.kind.id); parent = match.kind.id; }
    else { parent = undefined; }
  }
  const last = trail.slice(-3).join(" > ");
  const bad = trail.filter(row => row.includes("UNRESOLVED") || row.includes("AMBIGUOUS"));
  console.log(bad.length === 0 ? "RESOLVED" : "PROBLEM " + bad.join(", "), "|", last);
}
