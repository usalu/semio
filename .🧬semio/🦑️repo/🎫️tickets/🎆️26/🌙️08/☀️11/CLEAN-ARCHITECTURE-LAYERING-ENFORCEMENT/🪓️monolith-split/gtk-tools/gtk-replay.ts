/** 🔁️ Level A: replays the testkit wiring analysis on the live carved crates of an artifact and compares the derived wiring with the live manifests. Read-only; the report goes to argv[2]. Usage: bun gtk-replay.ts <report.json> <artifact-dir>... */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { crateIdent, crateTreeHostCfg, measureRustCrate } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
import { buildTestkitCrate, dependencyRowKey, externalDependencies, planTestkitWiring, type TestkitCrate } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧰️testkit/🟦️.ts";
import { readManifestDependencyRows } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { forwardableFeatures, loadExternalGates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const [report, ...artifacts] = process.argv.slice(2);
type Toml = Record<string, any>;

const parse = (path: string): Toml | null => { try { return Bun.TOML.parse(readFileSync(path, "utf8")) as Toml; } catch { return null; } };
const setOf = (values: Iterable<string>): string[] => [...new Set(values)].sort();
const results: any[] = [];
const hostCfg = crateTreeHostCfg();

for (const artifact of artifacts) {
  const workspace = parse(join(root, artifact, "Cargo.toml"));
  const members: string[] = workspace?.workspace?.members ?? [];
  const infos = members.map((member) => ({ dir: join(artifact, member), toml: parse(join(root, artifact, member, "Cargo.toml")) })).filter((entry) => entry.toml?.package?.name);
  const facades = infos.filter((entry) => entry.toml!.package.metadata?.semio?.role === "artifact").sort((a, b) => b.toml!.package.name.length - a.toml!.package.name.length);
  const generated = infos.filter((entry) => !facades.includes(entry) && entry.toml!.lib?.path === "🦀️.rs" && entry.toml!.package.metadata?.semio?.role === "library");
  if (!facades.length || generated.length === 0) { results.push({ artifact, skipped: "no facade or no generated crates" }); continue; }
  const groups = new Map<string, typeof generated>();
  for (const entry of generated) {
    const owner = facades.find((facade) => (entry.toml!.package.name as string).startsWith(`${facade.toml!.package.name}-`)) ?? facades[facades.length - 1]!;
    groups.set(owner.toml!.package.name, [...(groups.get(owner.toml!.package.name) ?? []), entry]);
  }
  const tally = { crates: 0, handMade: 0, groups: groups.size, featuresEqual: 0, featuresMissing: 0, featuresExtra: 0, devEqual: 0, devMissing: 0, devExtra: 0, optionalEqual: 0, optionalMissing: 0, optionalExtra: 0 };
  const rowsOut: any[] = [];
  for (const [facadeName, members2] of groups) {
    const facade = facades.find((entry) => entry.toml!.package.name === facadeName)!;
    const names = new Set(members2.map((entry) => entry.toml!.package.name as string));
    const facadeText = readFileSync(join(root, facade.dir, "Cargo.toml"), "utf8");
    const rows = readManifestDependencyRows(facadeText);
    const declared = new Set(forwardableFeatures(facadeText).map((feature) => feature.name));
    const externals = externalDependencies(facadeText, rows).filter((entry) => !names.has(entry.key));
    const reader = await loadExternalGates(root, resolve(root, facade.dir), resolve(root, artifact), externals);
    const build = async (dir: string, name: string, entry: string, dependsOn: string[], devDependsOn: string[] = []): Promise<TestkitCrate> => {
      const manifestDir = resolve(root, dir);
      const lib = await measureRustCrate(root, entry, false, new Map(), manifestDir).catch(() => null);
      const test = await measureRustCrate(root, entry, true, new Map(), manifestDir).catch(() => null);
      const files = new Map<string, string>();
      for (const path of [...(lib?.paths ?? []), ...(test?.paths ?? [])]) if (path.endsWith(".rs") && !files.has(path) && existsSync(join(root, path))) files.set(path, readFileSync(join(root, path), "utf8"));
      const base = new Set(lib?.paths ?? []);
      const closure = async (feature: string): Promise<ReadonlySet<string>> => {
        const measure = await measureRustCrate(root, entry, false, new Map(), manifestDir, [...hostCfg, `feature=${feature}`]);
        return new Set(measure.paths.filter((path) => path.endsWith(".rs") && !base.has(path)));
      };
      return buildTestkitCrate({ name, ident: crateIdent(name), dependsOn, devDependsOn, files }, closure);
    };
    const dependsOnOf = (toml: Toml): string[] => Object.keys(toml.dependencies ?? {}).filter((key) => names.has(key));
    const devOnlyOf = (toml: Toml): string[] => Object.keys(toml["dev-dependencies"] ?? {}).filter((key) => names.has(key) && !(key in (toml.dependencies ?? {})));
    const crates: TestkitCrate[] = [];
    for (const entry of members2) crates.push(await build(entry.dir, entry.toml!.package.name, resolve(root, entry.dir, "🦀️.rs"), dependsOnOf(entry.toml!), devOnlyOf(entry.toml!)));
    crates.push(await build(facade.dir, facadeName, resolve(root, facade.dir, facade.toml!.lib.path), [...names]));
    tally.crates += crates.length;
    const wiring = planTestkitWiring(crates, externals, declared, reader);
    for (const entry of [...members2, facade]) {
      if (entry !== facade && !String(entry.toml!.package.description ?? "").startsWith("Component crate of ")) { tally.handMade += 1; continue; }
      const name = entry.toml!.package.name as string;
      const derived = wiring.get(name)!;
      const live = entry.toml!;
      const liveFeatures = new Map<string, string[]>(Object.entries(live.features ?? {}).filter(([feature]) => !declared.has(feature)).map(([feature, list]) => [feature, setOf(list as string[])]));
      const derivedFeatures = new Map(derived.features.map((feature) => [feature.name, setOf(feature.entries)]));
      const row: any = { crate: name.replace(facadeName, "~"), group: facadeName.slice(-30), features: [], dev: [], optional: [] };
      for (const feature of new Set([...liveFeatures.keys(), ...derivedFeatures.keys()])) {
        const a = liveFeatures.get(feature), b = derivedFeatures.get(feature);
        if (a && b && JSON.stringify(a) === JSON.stringify(b)) tally.featuresEqual += 1;
        else if (a && !b) { tally.featuresMissing += 1; row.features.push(`MISSING ${feature} live=${JSON.stringify(a)}`); }
        else if (!a && b) { tally.featuresExtra += 1; row.features.push(`EXTRA ${feature} derived=${JSON.stringify(b)}`); }
        else { tally.featuresMissing += 1; row.features.push(`DIFF ${feature} live=${JSON.stringify(a)} derived=${JSON.stringify(b)}`); }
      }
      const liveDev = new Map<string, string[]>();
      for (const [dep, spec] of Object.entries(live["dev-dependencies"] ?? {}) as [string, any][]) if (names.has(dep) && typeof spec === "object" && Array.isArray(spec.features)) liveDev.set(dep, setOf((spec.features as string[]).filter((feature) => !declared.has(feature))));
      for (const dep of new Set([...liveDev.keys(), ...derived.consumed.keys()])) {
        const a = (liveDev.get(dep) ?? []).filter(Boolean), b = derived.consumed.get(dep) ?? [];
        if (JSON.stringify(a) === JSON.stringify(b)) { if (a.length) tally.devEqual += 1; }
        else if (a.length && !b.length) { tally.devMissing += 1; row.dev.push(`MISSING ${dep.replace(facadeName, "~")} live=${JSON.stringify(a)}`); }
        else if (!a.length && b.length) { tally.devExtra += 1; row.dev.push(`EXTRA ${dep.replace(facadeName, "~")} derived=${JSON.stringify(b)}`); }
        else { tally.devMissing += 1; row.dev.push(`DIFF ${dep.replace(facadeName, "~")} live=${JSON.stringify(a)} derived=${JSON.stringify(b)}`); }
      }
      const liveOptional = setOf(Object.entries(live.dependencies ?? {}).filter(([, spec]) => typeof spec === "object" && (spec as any).optional === true).map(([key]) => key));
      const derivedOptional = [...derived.optionalDependencies];
      for (const key of new Set([...liveOptional, ...derivedOptional])) { const a = liveOptional.includes(key), b = derivedOptional.includes(key); if (a && b) tally.optionalEqual += 1; else if (a) { tally.optionalMissing += 1; row.optional.push(`MISSING ${key}`); } else { tally.optionalExtra += 1; row.optional.push(`EXTRA ${key}`); } }
      if (row.features.length || row.dev.length || row.optional.length) rowsOut.push(row);
    }
  }
  results.push({ artifact, ...tally, diffs: rowsOut });
  console.log(`${artifact.replace("✏️s/🔌️plugins/", "")}\tgroups ${tally.groups}\tcrates ${tally.crates}\tfeat =${tally.featuresEqual} -${tally.featuresMissing} +${tally.featuresExtra}\tdev =${tally.devEqual} -${tally.devMissing} +${tally.devExtra}\topt =${tally.optionalEqual} -${tally.optionalMissing} +${tally.optionalExtra}`);
  writeFileSync(report!, JSON.stringify(results, null, 1));
}
void dirname;
void dependencyRowKey;
