#!/usr/bin/env bun
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { runNativeBuildDriverV1, nativeBuildPassArgsV1, type NativeBuildPlanV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { captureCargoCompilationGraphV1, actualCargoCompilerPlanningPortV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";
import { admitCargoSourceBoundariesV1, actualCargoSourceAdmissionPortV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { repositoryStrictStorageV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🟦️.ts";
import { cargoDirectories, componentPassPlanV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { playRuntimeComponentIds } from "../../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts";

/** 🧭️ R1 ticket probes for the release admission (ticket 26/09/23 BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
 * `legacy <plugin-id>` replays the pre-fix release call shape (no storage, no Source boundary) on a one-component plan;
 * `release <plugin-id>` runs the fixed release route on one component;
 * `batches <budget-MiB> <units.jsonl>` prints the planned hub batches; `census <release|prototype> <out.json>` captures every release pass's unit graph without compiling and measures every repository unit against the policy. */
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const control = (() => { const controller = new AbortController(), deadline = Date.now() + 2 * 3600_000; return { signal: controller.signal, remainingMilliseconds: () => deadline - Date.now(), publish: () => {}, yieldContinuation: async () => {} }; })();

/** 🕰️ The pre-fix `ReleaseScript` argument set against a one-component plan. */
async function legacy(pluginId: string): Promise<void> {
  const plan: NativeBuildPlanV1 = componentPassPlanV1(repoRoot, new Set([pluginId]), "release");
  try { await (runNativeBuildDriverV1 as (options: unknown) => Promise<unknown>)({ plan, compilerConfiguration: join(repoRoot, ".cargo/config.toml"), cwd: repoRoot, cacheRoot: repoCacheDirectory(repoRoot), shared: cargoDirectories(repoRoot), generation: "warm", leaseDirectory: join(repoRoot, ".cache", "framework", "leases"), environment: process.env, control }); console.log("[DEBUG] legacy call returned"); }
  catch (error) { console.log("[DEBUG] legacy call refused: " + (error instanceof Error ? error.message : String(error))); }
}

/** 🚀️ The post-fix release route on exactly `pluginId`: the root release pipeline through `buildReleasePipelineV1` under an explicit network grant. */
async function release(pluginId: string, offline: boolean): Promise<void> {
  const { buildReleasePipelineV1 } = await import("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts");
  const pipeline = JSON.parse(require("node:fs").readFileSync(join(repoRoot, "📋️project.json"), "utf8")).targets.release.metadata.semio.release;
  try { const outcome = await buildReleasePipelineV1(repoRoot, pipeline, { components: new Set([pluginId]), network: { offline }, control }); console.log("[DEBUG] release outcome " + JSON.stringify({ staged: outcome.staged, failed: outcome.failed, report: outcome.report })); }
  catch (error) { console.log("[DEBUG] release refused: " + (error instanceof Error ? error.message : String(error)).slice(0, 1500)); }
}

/** 📦️ Prints the batches the driver would plan for every release pass under `budgetMiB` of free storage: each pass's real unit graph (`--unit-graph`, nothing
 * compiled, in the issued release compiler store) against the recorded p95 output footprints in `<history>` (a `units.jsonl`), unmeasured units estimated
 * by kind, the reserve kept free. */
async function batches(budgetMiB: number, historyPath: string): Promise<void> {
  const { componentBatchesV1, NATIVE_BUILD_DRIVER_POLICY_V1, nativeBuildUnitGraphArgsV1 } = await import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts");
  const { ADMISSION_POLICY_V1, readUnitHistoryV1 } = await import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts");
  const { captureOwnedCommandV1 } = await import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
  const { withRepositoryCompilerStoreV1 } = await import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts");
  const history = await readUnitHistoryV1(resolve(historyPath)), plan = componentPassPlanV1(repoRoot, undefined, "release");
  await withRepositoryCompilerStoreV1(repoRoot, "release", control, { ...process.env, SEMIO_BUILD_MODE: "release" }, async (store) => {
    for (const pass of plan.passes) {
      let graph;
      try { graph = JSON.parse(await captureOwnedCommandV1("cargo", nativeBuildUnitGraphArgsV1(pass), repoRoot, { control, env: store.environment, output: { maximumBytes: 536870912, maximumLines: 1 } })); }
      catch (error) { console.log(`[DEBUG] ${pass.id} ${relative(repoRoot, pass.manifest)}: no unit graph (${String(error instanceof Error ? error.message : error).split("\n").find((line) => line.startsWith("error")) ?? "failed"})`); continue; }
      for (const batch of componentBatchesV1(pass, graph, history, budgetMiB * 1024 ** 2 - NATIVE_BUILD_DRIVER_POLICY_V1.storageReserveBytes, { quantile: ADMISSION_POLICY_V1.quantile, bounds: NATIVE_BUILD_DRIVER_POLICY_V1.unitFootprintBoundsBytes })) console.log(`[DEBUG] ${batch.pass.id} ${relative(repoRoot, pass.manifest)}: ${batch.pass.packages.length} component(s), ${(batch.bytes / 1024 ** 2).toFixed(0)} MiB${batch.measured ? "" : " (unmeasured units estimated by kind)"}`);
    }
  });
}

/** 📏️ Measures every repository unit of the workspace and play release closures against the configured Source boundary. */
async function census(mode: "release" | "prototype", output: string): Promise<void> {
  const policy = repositoryStrictStorageV1(repoRoot).sourceBoundary, environment = { ...process.env, SEMIO_BUILD_MODE: mode }, port = actualCargoSourceAdmissionPortV1(environment, repoRoot);
  const play = new Set(playRuntimeComponentIds()), plan = componentPassPlanV1(repoRoot, undefined, mode), rows = new Map<string, any>(), refusals: any[] = [], passes: any[] = [];
  for (const pass of plan.passes) {
    const started = Date.now();
    let graph;
    try { graph = await captureCargoCompilationGraphV1({ workspaceRoot: repoRoot, cwd: repoRoot, program: "cargo", prefix: [], args: nativeBuildPassArgsV1(pass), environment, maximumOutputBytes: 16777216 }, control, actualCargoCompilerPlanningPortV1()); }
    catch (error) { passes.push({ id: pass.id, manifest: relative(repoRoot, pass.manifest), error: String(error instanceof Error ? error.message : error).slice(0, 600) }); console.log(`[DEBUG] ${pass.id} graph failed`); continue; }
    try { await admitCargoSourceBoundariesV1(JSON.parse(JSON.stringify(graph)), repoRoot, policy, control, port); refusals.push({ pass: pass.id, refusal: null }); }
    catch (error) { refusals.push({ pass: pass.id, refusal: error instanceof Error ? error.message : String(error) }); }
    const components = pass.packages.map((row) => row.name), inPlay = pass.packages.filter((row) => play.has(Bun.TOML.parse(require("node:fs").readFileSync(row.manifest, "utf8")).package.metadata.component.package.replace(/^semio:/u, ""))).map((row) => row.name);
    const kinds = new Map<string, number>(); for (const unit of graph.units) kinds.set(unit.provenance.kind, (kinds.get(unit.provenance.kind) ?? 0) + 1);
    passes.push({ id: pass.id, manifest: relative(repoRoot, pass.manifest), components: components.length, playComponents: inPlay.length, units: graph.units.length, provenance: Object.fromEntries(kinds), ms: Date.now() - started });
    for (const unit of graph.units) {
      if (unit.provenance.kind !== "workspace") continue;
      const key = [unit.packageName, unit.kind, relative(repoRoot, unit.entry), unit.platform ?? "host", JSON.stringify(unit.features)].join("\0");
      const known = rows.get(key);
      if (known) { known.passes.add(pass.id); if (inPlay.length) known.play = true; continue; }
      const limits = unit.boundary ? { maxSourceBytes: Math.min(policy.maxSourceBytes, unit.boundary.maxSourceBytes), maxSourceFiles: Math.min(policy.maxSourceFiles, unit.boundary.maxSourceFiles), maxActiveLines: Math.min(policy.maxActiveLines, unit.boundary.maxActiveLines) } : policy;
      const distance = relative(repoRoot, unit.manifest), root = distance === ".." || distance.startsWith(".." + sep) || isAbsolute(distance) ? dirname(unit.manifest) : repoRoot;
      const row: any = { package: unit.packageName, kind: unit.kind, platform: unit.platform ?? "host", manifest: relative(repoRoot, unit.manifest), entry: relative(repoRoot, unit.entry), boundaryDeclared: Boolean(unit.boundary), limits, passes: new Set([pass.id]), play: inPlay.length > 0 };
      try {
        const cfg = await port.cfg(unit, control), inputs = await readRustSourceClosureV1([unit.entry], root, cfg, (path) => port.read(path, control), { manifestDirectory: dirname(unit.manifest), generated: () => undefined });
        row.files = inputs.length; row.bytes = inputs.reduce((sum, input) => sum + input.bytes, 0); row.lines = inputs.reduce((sum, input) => sum + input.activeLines, 0);
        row.largest = [...inputs].sort((left, right) => right.activeLines - left.activeLines).slice(0, 3).map((input) => ({ path: relative(repoRoot, resolve(root, input.path)), lines: input.activeLines, bytes: input.bytes }));
        row.over = row.files > limits.maxSourceFiles || row.bytes > limits.maxSourceBytes || row.lines > limits.maxActiveLines;
      } catch (error) { row.error = String(error instanceof Error ? error.message : error).slice(0, 400); row.over = true; }
      rows.set(key, row);
    }
    console.log(`[DEBUG] ${pass.id} ${relative(repoRoot, pass.manifest)} units=${graph.units.length} rows=${rows.size} ${Date.now() - started}ms`);
  }
  mkdirSync(dirname(output), { recursive: true });
  writeFileSync(output, JSON.stringify({ mode, policy, passes, refusals, units: [...rows.values()].map((row) => ({ ...row, passes: [...row.passes].sort() })) }, null, 1) + "\n");
  console.log(`[DEBUG] census wrote ${rows.size} unit rows, ${[...rows.values()].filter((row) => row.over).length} over the ceiling`);
}

const [command, ...rest] = process.argv.slice(2);
if (command === "legacy" && rest.length === 1) await legacy(rest[0]!);
else if (command === "release" && rest.length === 2 && ["offline", "online"].includes(rest[1]!)) await release(rest[0]!, rest[1] === "offline");
else if (command === "batches" && rest.length === 2 && Number(rest[0]) > 0) await batches(Number(rest[0]), rest[1]!);
else if (command === "census" && (rest[0] === "release" || rest[0] === "prototype") && rest[1]) await census(rest[0], resolve(rest[1]));
else throw Error("usage: legacy <plugin-id> | release <plugin-id> <offline|online> | batches <budget-MiB> <units.jsonl> | census <release|prototype> <out.json>");
