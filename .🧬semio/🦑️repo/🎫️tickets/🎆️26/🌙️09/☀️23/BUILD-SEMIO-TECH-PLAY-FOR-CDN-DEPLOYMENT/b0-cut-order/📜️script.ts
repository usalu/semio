#!/usr/bin/env bun
/**
 * 📏️ B0 lane (ticket 26/09/23 BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT): bottom-up cut order for the repository Source boundary.
 *
 * `census <metadata-dir> <manifests.txt> <out.json>`: every first-party crate of every workspace (from `cargo metadata --offline --no-deps` JSON, falling
 * back to direct manifest parsing for workspaces whose metadata fails) measured with the canonical Source reader exactly as `admitCargoSourceBoundariesV1`
 * measures a unit: full bytes, file count (source + embedded) and cfg-selected active lines, limits tightened by `package.metadata.semio.rust-source-boundary`.
 * Units: lib on native aarch64-macOS and wasm32-wasip2 under graph-unified features (plus native default-only and native all-features), build script, bins, lib test.
 *
 * `plan <census.json> <owners.json> <out.json>`: first-party DAG (normal + build edges form the build closure, dev edges the test closure), depth, the
 * over-ceiling crates in each closure, reverse reach (crates and shipped plugin components blocked), and bottom-up waves.
 * @see ../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts
 */
import { readFile } from "node:fs/promises";
import { readdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { playRuntimeComponentIds } from "../../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const POLICY = { maxActiveLines: 4000, maxSourceBytes: 262144, maxSourceFiles: 32 };
type Dep = { name: string; kind: "normal" | "build" | "dev"; optional: boolean; target: string | null; rename: string | null; features: string[]; usesDefault: boolean };
type Pkg = { name: string; manifest: string; source: "metadata" | "manifest"; targets: { kind: string[]; name: string; src_path: string }[]; features: Record<string, string[]>; deps: Dep[]; metadata: any };

/** 🎛️ The compiler cfg snapshot the admission port observes, normalised the same way. */
function rustcCfg(target?: string): string[] {
  return Bun.spawnSync(["rustc", "--print", "cfg", ...(target ? ["--target", target] : [])], { cwd: REPO }).stdout.toString().trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
}

/** 🧾️ Reads every package from metadata JSON, then every remaining tracked manifest directly. */
function collect(metadataDir: string, manifestList: string): { packages: Map<string, Pkg>; fallback: string[] } {
  const packages = new Map<string, Pkg>(), fallback: string[] = [];
  for (const file of readdirSync(metadataDir).filter(name => name.endsWith(".json"))) {
    const text = readFileSync(join(metadataDir, file), "utf8");
    if (!text.trim()) continue;
    for (const pkg of JSON.parse(text).packages) {
      const manifest = relative(REPO, pkg.manifest_path);
      if (packages.has(manifest) || manifest.includes("🎫️tickets")) continue;
      packages.set(manifest, { name: pkg.name, manifest, source: "metadata", targets: pkg.targets, features: pkg.features ?? {}, metadata: pkg.metadata, deps: pkg.dependencies.map((dep: any) => ({ name: dep.name, kind: dep.kind ?? "normal", optional: dep.optional, target: dep.target, rename: dep.rename, features: dep.features ?? [], usesDefault: dep.uses_default_features !== false })) });
    }
  }
  const tables = (toml: any) => [["dependencies", "normal", null, toml], ["build-dependencies", "build", null, toml], ["dev-dependencies", "dev", null, toml], ...Object.entries(toml.target ?? {}).flatMap(([target, table]) => [["dependencies", "normal", target, table], ["build-dependencies", "build", target, table], ["dev-dependencies", "dev", target, table]])] as [string, Dep["kind"], string | null, any][];
  for (const manifest of readFileSync(manifestList, "utf8").split("\n").filter(Boolean)) {
    if (packages.has(manifest) || manifest.includes("🎫️tickets") || !existsSync(join(REPO, manifest))) continue;
    const toml = Bun.TOML.parse(readFileSync(join(REPO, manifest), "utf8")) as any;
    if (!toml.package) continue;
    fallback.push(manifest);
    const directory = dirname(join(REPO, manifest)), libPath = toml.lib?.path ? resolve(directory, toml.lib.path) : join(directory, "src/lib.rs");
    const lib = toml.lib || existsSync(libPath) ? [{ kind: [toml.lib?.["proc-macro"] ? "proc-macro" : "lib"], name: toml.package.name, src_path: libPath }] : [];
    const buildPath = typeof toml.package.build === "string" ? resolve(directory, toml.package.build) : toml.package.build !== false && existsSync(join(directory, "build.rs")) ? join(directory, "build.rs") : null;
    const bins = (toml.bin ?? []).filter((bin: any) => bin.path).map((bin: any) => ({ kind: ["bin"], name: bin.name, src_path: resolve(directory, bin.path) }));
    const deps: Dep[] = [];
    for (const [table, kind, target, scope] of tables(toml)) for (const [key, value] of Object.entries(scope?.[table] ?? {}) as [string, any][]) deps.push({ name: typeof value === "object" && value.package ? value.package : key, kind, optional: Boolean(value?.optional), target, rename: typeof value === "object" && value.package ? key : null, features: typeof value === "object" ? value.features ?? [] : [], usesDefault: typeof value !== "object" || value["default-features"] !== false });
    packages.set(manifest, { name: toml.package.name, manifest, source: "manifest", targets: [...lib, ...(buildPath ? [{ kind: ["custom-build"], name: "build-script-build", src_path: buildPath }] : []), ...bins], features: toml.features ?? {}, metadata: toml.package.metadata ?? null, deps });
  }
  return { packages, fallback };
}

/** 🧮️ Expands features transitively inside one manifest and reports which optional deps they switch on. */
function expand(pkg: Pkg, seeds: string[]): { features: string[]; optional: Set<string>; forwards: Map<string, Set<string>> } {
  const enabled = new Set<string>(), optional = new Set<string>(), forwards = new Map<string, Set<string>>(), queue = [...seeds];
  while (queue.length) {
    const feature = queue.pop()!;
    if (feature.startsWith("dep:")) { optional.add(feature.slice(4)); continue; }
    if (feature.includes("/")) { const [head, tail] = feature.split("/") as [string, string], key = head.replace(/\?$/u, ""); if (!head.endsWith("?")) optional.add(key); forwards.set(key, (forwards.get(key) ?? new Set()).add(tail)); continue; }
    if (enabled.has(feature)) continue;
    if (!(feature in pkg.features)) { if (pkg.deps.some(dep => dep.optional && (dep.rename ?? dep.name) === feature)) optional.add(feature); continue; }
    enabled.add(feature);
    queue.push(...pkg.features[feature]!);
  }
  return { features: [...enabled].map(name => "feature=" + name), optional, forwards };
}

/** 🔗️ Cargo-style feature unification across the whole first-party graph (normal + build edges): a package's features are its defaults plus every
 * feature any consumer requests directly or forwards through `dep/feature`, iterated to a fixpoint. An upper bound for any single selection.
 * @see https://doc.rust-lang.org/cargo/reference/features.html#feature-unification */
function unify(packages: Map<string, Pkg>, byName: Map<string, Pkg[]>): Map<string, ReturnType<typeof expand>> {
  const requested = new Map<string, Set<string>>([...packages.values()].map(pkg => [pkg.manifest, new Set(["default"])]));
  for (let changed = true; changed;) {
    changed = false;
    for (const pkg of packages.values()) {
      const enabled = expand(pkg, [...requested.get(pkg.manifest)!]);
      for (const dep of pkg.deps) {
        const local = byName.get(dep.name)?.[0], key = dep.rename ?? dep.name;
        if (!local || dep.kind === "dev" || dep.optional && !enabled.optional.has(key)) continue;
        const target = requested.get(local.manifest)!, before = target.size;
        for (const feature of [...dep.features, ...(enabled.forwards.get(key) ?? [])]) target.add(feature);
        if (target.size !== before) changed = true;
      }
    }
  }
  return new Map([...packages.values()].map(pkg => [pkg.manifest, expand(pkg, [...requested.get(pkg.manifest)!])]));
}

/** 📐️ One unit's closure, measured as the admission measures it. */
async function measure(pkg: Pkg, entry: string, cfg: string[]) {
  const boundary = pkg.metadata?.semio?.["rust-source-boundary"], limits = boundary ? { maxActiveLines: Math.min(POLICY.maxActiveLines, boundary.maxActiveLines), maxSourceBytes: Math.min(POLICY.maxSourceBytes, boundary.maxSourceBytes), maxSourceFiles: Math.min(POLICY.maxSourceFiles, boundary.maxSourceFiles) } : POLICY;
  try {
    const inputs = await readRustSourceClosureV1([entry], REPO, cfg, async path => await readFile(path).catch(error => ["ENOENT", "ENOTDIR", "EISDIR"].includes(error.code) ? null : Promise.reject(error)), { manifestDirectory: dirname(join(REPO, pkg.manifest)), generated: () => undefined });
    const lines = inputs.reduce((sum, input) => sum + input.activeLines, 0), bytes = inputs.reduce((sum, input) => sum + input.bytes, 0), files = inputs.length;
    const binds = [lines > limits.maxActiveLines ? "lines" : "", bytes > limits.maxSourceBytes ? "bytes" : "", files > limits.maxSourceFiles ? "files" : ""].filter(Boolean);
    return { lines, bytes, files, over: binds.length > 0, binds, ratio: Math.max(lines / limits.maxActiveLines, bytes / limits.maxSourceBytes, files / limits.maxSourceFiles), limits: boundary ? limits : undefined, largest: [...inputs].sort((a, b) => b.bytes - a.bytes).slice(0, 3).map(input => ({ path: input.path, lines: input.activeLines, bytes: input.bytes, kind: input.kind })) };
  } catch (error) {
    return { lines: 0, bytes: 0, files: 0, over: true, binds: ["refused"], ratio: Infinity, error: String(error instanceof Error ? error.message : error).slice(0, 300) };
  }
}

/** 📏️ Measures every package and writes the census. */
async function census(metadataDir: string, manifestList: string, output: string): Promise<void> {
  const NATIVE = rustcCfg(), WASM = rustcCfg("wasm32-wasip2"), { packages, fallback } = collect(metadataDir, manifestList), play = new Set(playRuntimeComponentIds());
  const byName = new Map<string, Pkg[]>();
  for (const pkg of packages.values()) byName.set(pkg.name, [...(byName.get(pkg.name) ?? []), pkg]);
  const unified = unify(packages, byName), rows: any[] = [];
  for (const pkg of [...packages.values()].sort((a, b) => a.manifest.localeCompare(b.manifest))) {
    const lib = pkg.targets.find(target => target.kind.some(kind => ["lib", "rlib", "cdylib", "staticlib", "dylib", "proc-macro"].includes(kind)));
    const build = pkg.targets.find(target => target.kind.includes("custom-build")), bins = pkg.targets.filter(target => target.kind.includes("bin"));
    const defaults = expand(pkg, ["default"]), features = unified.get(pkg.manifest)!, all = expand(pkg, Object.keys(pkg.features)), proc = Boolean(lib?.kind.includes("proc-macro"));
    const deps = pkg.deps.flatMap(dep => { const local = byName.get(dep.name); return local ? [{ name: dep.name, manifest: local[0]!.manifest, kind: dep.kind, optional: dep.optional, enabled: !dep.optional || features.optional.has(dep.rename ?? dep.name), target: dep.target }] : []; });
    const component = pkg.metadata?.component?.package as string | undefined;
    const row: any = { name: pkg.name, manifest: pkg.manifest, source: pkg.source, proc, component: component ?? null, shipped: component ? play.has(component.replace(/^semio:/u, "")) : false, deps, duplicates: (byName.get(pkg.name)?.length ?? 1) - 1 };
    if (lib) {
      row.unifiedFeatures = features.features.map(feature => feature.slice(8)).sort();
      row.native = await measure(pkg, lib.src_path, [...NATIVE, ...features.features]);
      if (!proc) row.wasm = await measure(pkg, lib.src_path, [...WASM, ...features.features]);
      row.nativeDefault = await measure(pkg, lib.src_path, [...NATIVE, ...defaults.features]);
      row.allFeatures = await measure(pkg, lib.src_path, [...NATIVE, ...all.features]);
      row.test = await measure(pkg, lib.src_path, [...NATIVE, ...features.features, "test"]);
    }
    if (build) row.build = await measure(pkg, build.src_path, [...NATIVE]);
    if (bins.length) row.bins = await Promise.all(bins.map(async bin => ({ name: bin.name, ...(await measure(pkg, bin.src_path, [...NATIVE, ...features.features])) })));
    row.over = Boolean(row.native?.over || row.wasm?.over || row.build?.over);
    rows.push(row);
  }
  writeFileSync(output, JSON.stringify({ policy: POLICY, fallback, packages: rows }, null, 1) + "\n");
  console.log(`[DEBUG] census ${rows.length} packages (${fallback.length} from manifest fallback), ${rows.filter(row => row.over).length} over (lib native/wasm or build script), ${rows.filter(row => row.native?.error).length} refused`);
}

/** 🏷️ First matching owner rule for a package: `{ lane, status, name?, manifest? }` with regex sources. */
function ownerOf(rules: any[], row: any): { lane: string; status: string } {
  const rule = rules.find(rule => (!rule.name || new RegExp(rule.name, "u").test(row.name)) && (!rule.manifest || new RegExp(rule.manifest, "u").test(row.manifest)));
  return rule ? { lane: rule.lane, status: rule.status } : { lane: "", status: "unowned" };
}

/** 🕸️ Builds the DAG, the closures, the waves and the ranking. */
function plan(censusPath: string, ownersPath: string, output: string): void {
  const { packages } = JSON.parse(readFileSync(censusPath, "utf8")), rules = JSON.parse(readFileSync(ownersPath, "utf8")).rules;
  const index = new Map<string, any>(packages.map((row: any) => [row.manifest, row]));
  const edges = new Map<string, Set<string>>(), dev = new Map<string, Set<string>>(), kinds = { normal: 0, build: 0, dev: 0, optionalSkipped: 0 };
  for (const row of packages) {
    edges.set(row.manifest, new Set()); dev.set(row.manifest, new Set());
    for (const dep of row.deps) {
      if (!dep.enabled) { kinds.optionalSkipped++; continue; }
      kinds[dep.kind as "normal" | "build" | "dev"]++;
      if (dep.manifest === row.manifest) continue;
      (dep.kind === "dev" ? dev : edges).get(row.manifest)!.add(dep.manifest);
    }
  }
  const closure = new Map<string, Set<string>>(), onStack = new Set<string>(), cycles: string[][] = [];
  const reach = (node: string): Set<string> => {
    const known = closure.get(node);
    if (known) return known;
    if (onStack.has(node)) { cycles.push([...onStack, node].map(manifest => index.get(manifest)?.name)); return new Set(); }
    onStack.add(node);
    const set = new Set<string>();
    for (const next of edges.get(node)!) { set.add(next); for (const deep of reach(next)) set.add(deep); }
    onStack.delete(node);
    closure.set(node, set);
    return set;
  };
  const depth = new Map<string, number>();
  const depthOf = (node: string): number => depth.get(node) ?? (depth.set(node, 0), depth.set(node, Math.max(0, ...[...edges.get(node)!].map(next => depthOf(next) + 1))).get(node)!);
  for (const row of packages) { reach(row.manifest); depthOf(row.manifest); }
  const testClosure = (node: string) => { const set = new Set(closure.get(node)); for (const next of dev.get(node)!) { set.add(next); for (const deep of closure.get(next)!) set.add(deep); } return set; };
  const over = packages.filter((row: any) => row.over).map((row: any) => row.manifest) as string[], overSet = new Set(over);
  const blockers = new Map<string, Set<string>>();
  for (const row of packages) blockers.set(row.manifest, new Set([...closure.get(row.manifest)!, row.manifest].filter(node => overSet.has(node))));
  const wave = new Map<string, number>();
  const waveOf = (node: string): number => wave.get(node) ?? (wave.set(node, 1 + Math.max(0, ...[...closure.get(node)!].filter(next => overSet.has(next)).map(waveOf))).get(node)!);
  const result = over.map(node => {
    const row = index.get(node), upstream = [...closure.get(node)!].filter(next => overSet.has(next));
    const frontier: string[] = [], seen = new Set<string>(), queue = [...edges.get(node)!];
    while (queue.length) { const next = queue.pop()!; if (seen.has(next)) continue; seen.add(next); if (overSet.has(next)) frontier.push(next); else queue.push(...edges.get(next)!); }
    const dependents = packages.filter((other: any) => other.manifest !== node && closure.get(other.manifest)!.has(node));
    const testDependents = packages.filter((other: any) => other.manifest !== node && testClosure(other.manifest).has(node));
    const sole = packages.filter((other: any) => !overSet.has(other.manifest) && blockers.get(other.manifest)!.size === 1 && blockers.get(other.manifest)!.has(node));
    const testWave = Math.max(waveOf(node), 1 + Math.max(0, ...[...testClosure(node)].filter(next => next !== node && overSet.has(next)).map(waveOf)));
    const unit = row.native ?? row.build;
    return {
      name: row.name, manifest: node, ...ownerOf(rules, row), lines: unit?.lines ?? 0, bytes: unit?.bytes ?? 0, files: unit?.files ?? 0,
      binds: [...new Set([...(row.native?.binds ?? []), ...(row.wasm?.binds ?? []), ...(row.build?.over ? ["build-script"] : [])])], error: row.native?.error ?? row.wasm?.error ?? row.build?.error, wasmOnly: !row.native?.over && Boolean(row.wasm?.over),
      ratio: unit?.ratio ?? 0, largest: unit?.largest ?? [], depth: depth.get(node), wave: waveOf(node), testWave,
      overDeps: upstream.map(next => index.get(next).name).sort(), frontierOverDeps: frontier.map(next => index.get(next).name).sort(),
      blocked: dependents.length, blockedShipped: dependents.filter((other: any) => other.shipped).length, blockedComponents: dependents.filter((other: any) => other.component).length,
      testBlocked: testDependents.length, soleBlocker: sole.length, soleBlockerShipped: sole.filter((other: any) => other.shipped).length, shipped: row.shipped, component: row.component,
      directDependents: packages.filter((other: any) => edges.get(other.manifest)!.has(node)).length,
    };
  }).sort((a: any, b: any) => a.wave - b.wave || b.soleBlocker - a.soleBlocker || b.blocked - a.blocked || a.name.localeCompare(b.name));
  const shipped = packages.filter((row: any) => row.shipped), compliantShipped = shipped.filter((row: any) => blockers.get(row.manifest)!.size === 0);
  const summary = { packages: packages.length, over: over.length, edges: kinds, cycles: cycles.slice(0, 20), maxDepth: Math.max(...depth.values()), waves: Math.max(...result.map((row: any) => row.wave)), shipped: shipped.length, shippedFullyCompliant: compliantShipped.map((row: any) => row.name), compliantClosureCrates: packages.filter((row: any) => blockers.get(row.manifest)!.size === 0).length };
  writeFileSync(output, JSON.stringify({ summary, over: result, shippedBlockers: shipped.map((row: any) => ({ name: row.name, component: row.component, blockers: [...blockers.get(row.manifest)!].map(node => index.get(node).name).sort() })) }, null, 1) + "\n");
  console.log(`[DEBUG] plan ${JSON.stringify(summary).slice(0, 600)}`);
}

/** 🕰️ For named crates: current measure, newest mtime in the native lib closure (+ manifest), and the newest peer report mentioning the package name or its
 * owner directory; peer-active when the source changed within `hours`. */
async function custody(censusPath: string, peerDir: string, hours: number, output: string, names: string[]): Promise<void> {
  const { statSync } = await import("node:fs"), NATIVE = rustcCfg(), { packages } = JSON.parse(readFileSync(censusPath, "utf8")), now = Date.now();
  const reports = readdirSync(peerDir).filter(name => name.endsWith(".md")).map(name => ({ name, mtime: statSync(join(peerDir, name)).mtimeMs, text: readFileSync(join(peerDir, name), "utf8") })).sort((a, b) => b.mtime - a.mtime);
  const rows = [];
  for (const name of names) {
    const row = packages.find((candidate: any) => candidate.name === name);
    if (!row) { rows.push({ name, missing: true }); continue; }
    const manifest = join(REPO, row.manifest), toml = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any, entry = resolve(dirname(manifest), toml.lib?.path ?? "src/lib.rs");
    const inputs = await readRustSourceClosureV1([entry], REPO, NATIVE, async path => await readFile(path).catch(() => null), { manifestDirectory: dirname(manifest), generated: () => undefined }).catch(() => []);
    const stamped = [manifest, ...inputs.map(input => join(REPO, input.path))].map(path => ({ path: relative(REPO, path), mtime: statSync(path).mtimeMs })).sort((a, b) => b.mtime - a.mtime);
    const owner = relative(join(REPO, "🧰️framework"), dirname(manifest)).replace(/\/📦️packages\/🦀️rust$/u, ""), tail = owner.split("/").slice(-2).join("/");
    const pattern = new RegExp(`${name}(?![a-z0-9-])|${tail.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}(?!/)`, "u");
    const report = reports.find(candidate => pattern.test(candidate.text)), line = report?.text.split("\n").find(text => pattern.test(text));
    const ageHours = (now - stamped[0]!.mtime) / 3600_000;
    rows.push({ name, over: row.over, lines: row.native?.lines, bytes: row.native?.bytes, files: row.native?.files, owner, newest: stamped[0]!.path, newestMtime: new Date(stamped[0]!.mtime).toISOString(), ageHours: Number(ageHours.toFixed(2)), status: ageHours <= hours ? "peer-active" : "idle", report: report ? { name: report.name, mtime: new Date(report.mtime).toISOString(), line: line!.trim().slice(0, 240) } : null });
  }
  writeFileSync(output, JSON.stringify(rows, null, 1) + "\n");
  for (const row of rows as any[]) console.log(`[DEBUG] ${row.name} over=${row.over} ${row.lines}/${row.bytes}/${row.files} newest=${row.newestMtime} (${row.ageHours} h) ${row.status} report=${row.report?.name ?? "-"}`);
}

const [command, ...rest] = process.argv.slice(2);
if (command === "census" && rest.length === 3) await census(resolve(rest[0]!), resolve(rest[1]!), resolve(rest[2]!));
else if (command === "plan" && rest.length === 3) plan(resolve(rest[0]!), resolve(rest[1]!), resolve(rest[2]!));
else if (command === "custody" && rest.length > 4) await custody(resolve(rest[0]!), resolve(rest[1]!), Number(rest[2]), resolve(rest[3]!), rest.slice(4));
else throw Error("usage: census <metadata-dir> <manifests.txt> <out.json> | plan <census.json> <owners.json> <out.json> | custody <census.json> <peer-ticket-dir> <hours> <out.json> <crate>...");
