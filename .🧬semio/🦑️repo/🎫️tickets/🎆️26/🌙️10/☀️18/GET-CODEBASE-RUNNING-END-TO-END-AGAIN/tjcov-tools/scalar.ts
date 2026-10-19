import { join } from "node:path";
import { readFileSync } from "node:fs";
import { POLICY_SPLIT_SOURCE_ROOTS, policyReadModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobOwnerSourceEvidence, toolJobOwnerSourceText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const W = "/Users/ueli/Documents/semio";
const base = join(W, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command");
const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json"), "utf8"));
console.log(fixture.sources);
const files = new Map<string, string>(fixture.sources.map((file: string) => [file, toolJobOwnerSourceText(W, file)]));
const evidence = toolJobOwnerSourceEvidence(files);
console.log("failures", evidence.failures.length, "scanThenMonolith", evidence.scanThenMonolith.length);
const cat = new Map<string, number>(); for (const f of evidence.failures) { const k = String(f).split(" ").slice(0, 5).join(" ") + " " + String(f).split("\u0000").slice(-4, -3).join(); cat.set(k, (cat.get(k) ?? 0) + 1); } for (const [k, v] of cat) console.log(v, k);
for (const owner of fixture.owners) {
  const actual = evidence.rows.filter((row: any) => row.file === owner.file).map((row: any) => row.id).sort();
  const want = owner.routes.map((r: any) => r.id).sort();
  console.log(owner.id, "rows", actual.length, "fixture", want.length, "missing-in-source", want.filter((x: string) => !actual.includes(x)), "extra-in-source", actual.filter((x: string) => !want.includes(x)));
}
for (const owner of fixture.owners) {
  const rows = evidence.rows.filter((row: any) => row.file === owner.file);
  console.log(owner.id, rows.map((r: any) => r.id + ":" + r.source[0] + ":" + (evidence.dispositions.find((d: any) => d.key === `${owner.file}\0${r.id}`)?.disposition ?? "?")[0]).join(" "));
  console.log("  fixture", owner.routes.map((r: any) => r.id + ":" + r.disposition[0]).join(" "));
}
for (const owner of fixture.owners) {
  const t = files.get(owner.file)!;
  console.log("TREE", owner.id, t.length, "action_interactive_job count", (t.match(/\.action_interactive_job\(/g) ?? []).length, "disp keys", evidence.dispositions.filter((d: any) => d.key.startsWith(owner.file)).length, Object.keys(evidence));
}
