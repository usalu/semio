/** 🔎️ Read-only inventory of the generated crate manifests of every carved artifact: features, sibling dev-dependency features, forwards and required-features, classified per package. Writes only to the path given as argv[2]. */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const root = "/Users/ueli/Documents/semio";
const out = process.argv[2]!;
const owners = (JSON.parse(readFileSync(join(root, "📋️build-owners.json"), "utf8")) as { owners: { name: string; root: string }[] }).owners;
const artifacts = new Map<string, string[]>();
for (const owner of owners) {
  const match = /^(✏️s\/🔌️plugins\/[^/]+\/🗿️artifacts\/[^/]+)\/(.*)$/u.exec(owner.root);
  if (match) artifacts.set(match[1]!, [...(artifacts.get(match[1]!) ?? []), owner.root]);
}
type Toml = Record<string, any>;
const parse = (path: string): Toml | null => { try { return Bun.TOML.parse(readFileSync(path, "utf8")) as Toml; } catch { return null; } };
const rows: any[] = [];
for (const [artifact, roots] of [...artifacts].sort()) {
  const facadeDir = `${artifact}/📦️packages/🦀️rust`;
  const generated = roots.filter((dir) => dir !== facadeDir && existsSync(join(root, dir, "Cargo.toml")));
  const facade = parse(join(root, facadeDir, "Cargo.toml"));
  const genManifests = generated.map((dir) => ({ dir, toml: parse(join(root, dir, "Cargo.toml")) })).filter((entry) => entry.toml && entry.toml.lib?.path === "🦀️.rs" && entry.toml.package?.metadata?.semio?.role === "library");
  if (genManifests.length < 2) continue;
  const names = new Set(genManifests.map((entry) => entry.toml!.package.name as string));
  const siblingNames = new Set([...names, facade?.package?.name].filter(Boolean));
  const facadeFeatures = Object.keys(facade?.features ?? {});
  const classes: Record<string, string[]> = { ownerFeatures: [], forwardedFeatures: [], devDepSiblingFeatures: [], requiredFeatures: [], testTargets: [], externalForwards: [], depsFeatures: [] };
  for (const { dir, toml } of genManifests) {
    const name = toml!.package.name as string;
    for (const [feature, entries] of Object.entries(toml!.features ?? {}) as [string, string[]][]) {
      const external = entries.filter((entry) => entry.includes("/") && !siblingNames.has(entry.split("/")[0]!));
      const sibling = entries.filter((entry) => entry.includes("/") && siblingNames.has(entry.split("/")[0]!));
      classes[facadeFeatures.includes(feature) ? "forwardedFeatures" : "ownerFeatures"]!.push(`${name}: ${feature} = [${entries.join(", ")}]`);
      if (external.length && !facadeFeatures.includes(feature)) classes.externalForwards!.push(`${name}: ${feature} -> ${external.join(", ")}`);
      void sibling;
    }
    for (const [dep, spec] of Object.entries(toml!["dev-dependencies"] ?? {}) as [string, any][]) if (siblingNames.has(dep) && typeof spec === "object" && spec.features) classes.devDepSiblingFeatures!.push(`${name} -> ${dep} features ${JSON.stringify(spec.features)}`);
    for (const [dep, spec] of Object.entries(toml!.dependencies ?? {}) as [string, any][]) if (siblingNames.has(dep) && typeof spec === "object" && spec.features) classes.depsFeatures!.push(`${name} -> ${dep} features ${JSON.stringify(spec.features)}`);
    for (const test of toml!.test ?? []) { classes.testTargets!.push(`${name}: ${test.name} ${test.path ?? ""}`); if (test["required-features"]) classes.requiredFeatures!.push(`${name}: ${test.name} ${JSON.stringify(test["required-features"])}`); }
  }
  for (const test of facade?.test ?? []) { classes.testTargets!.push(`facade: ${test.name} ${test.path ?? ""}`); if (test["required-features"]) classes.requiredFeatures!.push(`facade: ${test.name} ${JSON.stringify(test["required-features"])}`); }
  for (const [dep, spec] of Object.entries(facade?.["dev-dependencies"] ?? {}) as [string, any][]) if (names.has(dep) && typeof spec === "object" && spec.features) classes.devDepSiblingFeatures!.push(`facade -> ${dep} features ${JSON.stringify(spec.features)}`);
  rows.push({ artifact, facade: facade?.package?.name, crates: genManifests.length, facadeFeatures, ...classes });
}
writeFileSync(out, JSON.stringify(rows, null, 1));
const summary = rows.map((row) => `${row.crates}\t${row.ownerFeatures.length}\t${row.forwardedFeatures.length}\t${row.devDepSiblingFeatures.length}\t${row.requiredFeatures.length}\t${row.testTargets.length}\t${row.externalForwards.length}\t${row.depsFeatures.length}\t${row.artifact.replace("✏️s/🔌️plugins/", "")}`);
console.log("crates\townerF\tfwdF\tdevDepF\treqF\ttestT\textFwd\tdepsF\tartifact\n" + summary.join("\n"));
