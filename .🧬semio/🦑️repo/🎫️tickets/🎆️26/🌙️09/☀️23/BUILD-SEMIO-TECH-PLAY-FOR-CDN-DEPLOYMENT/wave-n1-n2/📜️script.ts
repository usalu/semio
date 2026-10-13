/** 🧪️ Wave N1/N2 ticket tools: `probe [variant]` prints the coarse release inference without constructing the Nx graph; `demo <plugin-id…>` runs one cold
 * build driver over a few components (descriptor emitter and browser support as jobs, every component streamed through describe, transpile, `wasm-opt`,
 * probe and publish, committed pairs written) in a private generation under `⚡️cache/n1` and prints how the stages overlapped the compilation; `rerun
 * <generation> <plugin-id…>` runs the same driver warm over that generation's Cargo stores with its stage store as the warm one; `contracts` runs the
 * repository cache contracts (the Nx contract fixtures) under a script envelope; `script <📜️script.ts> <argument…>` runs one owner script command under a
 * ticket-owned script envelope, as Nx would. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repo = resolve(import.meta.dir, "../../../../../../../.."), library = join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), cache = join(repo, ".🧬semio/🦑️repo/⚡️cache/n1");

/** 🔬️ The release orchestrators' prerequisites and one variant's closure preparation, straight from the Nx plugin's pure inference. */
async function probe(variant: string): Promise<void> {
  const { cacheInternals, libraryBootstrap } = await import(join(library, "🟨️.mjs"));
  await libraryBootstrap;
  const configFiles = cacheInternals.walkCargoToml(repo), contracts = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8")).generatorContracts, roots = cacheInternals.componentCrateRoots(configFiles, repo);
  console.log(`[DEBUG] component crates ${roots.length}`);
  for (const [file, target] of [["📋️project.json", "release"], ["🏢️semio-tech/🎡️play/📋️project.json", "build"]] as const) {
    const json = JSON.parse(readFileSync(join(repo, file), "utf8")), project = { name: json.name, targets: { [target]: structuredClone(json.targets[target]) } };
    cacheInternals.withReleasePipelinePreparation(project, repo, contracts, roots);
    console.log(`[DEBUG] ${json.name}:${target} dependsOn ${JSON.stringify(project.targets[target].dependsOn)}`);
  }
  const targets = cacheInternals.playgroundPreparationTargets(configFiles, repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript", contracts);
  for (const name of [`prepare-${variant}-react-dev`, `prepare-${variant}-native-release`, `build-${variant}-react-release`]) console.log(`[DEBUG] ${name} ${JSON.stringify({ cache: targets[name]?.cache, outputs: targets[name]?.outputs, dependsOn: targets[name]?.dependsOn })}`);
}

/** 🚀️ One driver run over `pluginIds` with the play pipeline's emitter and support jobs, then the stage-versus-compile overlap table: cold in a fresh
 * generation, or warm over `reuse`'s Cargo stores. */
async function demo(pluginIds: readonly string[], reuse?: string): Promise<void> {
  const [{ runReleaseDriverV1 }, { cargoDirectories, releasePipelinePlanV1 }, { componentDriverStagesV1 }, { createScriptProcessEnvelope, scriptProcessEnvironment }] = await Promise.all([
    import(join(repo, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts")),
    import(join(library, "⚡️caching/🦀️cargo/🟦️.ts")),
    import(join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts")),
    import(join(repo, "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts")),
  ]);
  const play = JSON.parse(readFileSync(join(repo, "🏢️semio-tech/🎡️play/📋️project.json"), "utf8")).targets.build.metadata.semio.release;
  const pipeline = { prerequisites: [], stageAfter: play.stageAfter, jobs: play.jobs.filter((job: { id: string }) => ["describe-tool", "support"].includes(job.id)).map((job: { after: string[] }) => ({ ...job, after: [] })) };
  const { plan, jobEnvironment } = releasePipelinePlanV1(repo, pipeline, new Set(pluginIds));
  const environment = scriptProcessEnvironment(createScriptProcessEnvelope({ version: 1, owner: "ticket:wave-n1-n2:demo", maximumElapsedMilliseconds: 0 }, {}, Date.now()), process.env);
  const controller = new AbortController(), stop = (): void => controller.abort(Error("demo stopped"));
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  const outcome = await runReleaseDriverV1({ plan, cwd: repo, cacheRoot: cache, shared: reuse ? { target: join(reuse, "t"), build: join(reuse, "b") } : cargoDirectories(repo), generation: reuse ? "warm" : "fresh", leaseDirectory: join(repo, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), environment, signal: controller.signal, stages: componentDriverStagesV1(repo, "write"), jobEnvironment, progressMs: 15_000 });
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8")), first = Math.min(...units.units.map((unit: any) => unit.startMs)), last = Math.max(...units.units.map((unit: any) => unit.endMs));
  console.log(`[DEBUG] demo ${JSON.stringify({ staged: outcome.staged, units: outcome.units, report: outcome.report, compileStart: 0, compileEnd: Math.round((last - first) / 1000) })}`);
  for (const stage of [...units.stages].sort((left: any, right: any) => left.startMs - right.startMs)) console.log(`[DEBUG] stage ${stage.component} ${stage.stage} start ${Math.round((stage.startMs - first) / 1000)} s end ${Math.round((stage.endMs - first) / 1000)} s ${stage.endMs < last ? "inside the compile window" : "after the last unit"}`);
}

/** ⚖️ The repository cache contracts, exactly as `repo-lib:test` runs them, under a ticket-owned script envelope. */
async function contracts(): Promise<void> {
  const { createScriptProcessEnvelope, scriptProcessEnvironment } = await import(join(repo, "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
  Object.assign(process.env, scriptProcessEnvironment(createScriptProcessEnvelope({ version: 1, owner: "ticket:wave-n1-n2:contracts", maximumElapsedMilliseconds: 0 }, {}, Date.now()), process.env));
  const { testCacheContracts } = await import(join(library, "⚡️caching/📜️script.ts"));
  await testCacheContracts();
  console.log("[DEBUG] cache contracts passed");
}

/** 🏃️ One owner script command in the foreground under a ticket-owned script envelope; its exit status becomes this process's. */
async function script(path: string, args: readonly string[]): Promise<void> {
  const { createScriptProcessEnvelope, scriptProcessEnvironment } = await import(join(repo, "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
  const environment = scriptProcessEnvironment(createScriptProcessEnvelope({ version: 1, owner: "ticket:wave-n1-n2:script", maximumElapsedMilliseconds: 0 }, {}, Date.now()), process.env);
  const child = Bun.spawn([process.execPath, resolve(path), ...args], { cwd: resolve(path, ".."), env: environment, stdout: "inherit", stderr: "inherit" });
  process.exitCode = await child.exited;
}

const [command, ...rest] = process.argv.slice(2);
if (command === "probe") await probe(rest[0] ?? "raster");
else if (command === "demo" && rest.length) await demo(rest);
else if (command === "contracts" && !rest.length) await contracts();
else if (command === "script" && rest.length) await script(rest[0]!, rest.slice(1));
else if (command === "rerun" && rest.length > 1) await demo(rest.slice(1), resolve(rest[0]!));
else throw new Error("probe [variant] | demo <plugin-id…> | rerun <generation> <plugin-id…> | contracts | script <📜️script.ts> <argument…>");
