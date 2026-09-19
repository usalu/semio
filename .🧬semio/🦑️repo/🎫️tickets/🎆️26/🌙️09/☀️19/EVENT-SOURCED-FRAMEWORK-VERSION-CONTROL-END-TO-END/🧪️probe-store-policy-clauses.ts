/** 🧪️ Splits the store policy predicates into their `&&` clauses and prints every clause that is false. */
import { readFileSync } from "node:fs";
import { toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact } from "../../../../../../../📜️script.ts";
const root = "/Users/ueli/Documents/semio/";
const read = (path: string) => readFileSync(root + path, "utf8");
const sources: Record<string, string> = {
  store: read("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"),
  vcs: read("🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs"),
  causal: read("🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs"),
  semio: read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"),
};
for (const predicate of [toolJobHistoryLedgerAdmissionExact, toolJobArtifactStoreStructuralOwnersExact]) {
  const text = predicate.toString();
  for (const match of text.matchAll(/(!?)(store|vcs|causal|semio)\.includes\(("(?:[^"\\]|\\.)*")\)/g)) {
    const expected = match[1] !== "!";
    const needle = JSON.parse(match[3]!) as string;
    if (sources[match[2]!]!.includes(needle) !== expected) console.log(`${predicate.name}: ${expected ? "MISSING" : "FORBIDDEN"} ${match[2]}: ${needle.slice(0, 200)}`);
  }
}
