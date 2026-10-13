/** 🧪️ Wave L1 ticket tools: `demo <plugin-id…>` runs one cold release driver over a few component crates in a private generation under
 * `⚡️cache/l1` (no post-link stages: the run shows each `🧩️component` crate link and stage into its `dist/components`) and prints the staged
 * deliverables; `resume <generation> <plugin-id…>` continues a deadline-stopped cold run over that generation's own Cargo stores; `script <📜️script.ts> <argument…>` runs one owner script command under a ticket-owned script envelope, as Nx would. */
import { existsSync, statSync } from "node:fs";
import { join, resolve } from "node:path";

const repo = resolve(import.meta.dir, "../../../../../../../.."), library = join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), cache = join(repo, ".🧬semio/🦑️repo/⚡️cache/l1");

/** 🚀️ One cold driver run over `pluginIds`: Cargo links each component crate and the driver stages it beside its manifest. */
async function demo(pluginIds: readonly string[], reuse?: string): Promise<void> {
  const [{ runReleaseDriverV1 }, { cargoDirectories, releasePipelinePlanV1 }, { createScriptProcessEnvelope, scriptProcessEnvironment }] = await Promise.all([
    import(join(repo, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts")),
    import(join(library, "⚡️caching/🦀️cargo/🟦️.ts")),
    import(join(repo, "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts")),
  ]);
  const { plan, jobEnvironment } = releasePipelinePlanV1(repo, { prerequisites: [], jobs: [] }, new Set(pluginIds));
  const environment = scriptProcessEnvironment(createScriptProcessEnvelope({ version: 1, owner: "ticket:wave-l1:demo", maximumElapsedMilliseconds: 0 }, {}, Date.now()), process.env);
  const controller = new AbortController(), stop = (): void => controller.abort(Error("demo stopped"));
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  const outcome = await runReleaseDriverV1({ plan, cwd: repo, cacheRoot: cache, shared: reuse ? { target: join(reuse, "t"), build: join(reuse, "b") } : cargoDirectories(repo), generation: reuse ? "warm" : "fresh", leaseDirectory: join(repo, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), environment, signal: controller.signal, jobEnvironment, progressMs: 15_000 });
  console.log(`[DEBUG] demo ${JSON.stringify({ generation: outcome.generation.root, staged: outcome.staged, failed: outcome.failed, units: outcome.units, report: outcome.report })}`);
  for (const pass of plan.passes) for (const row of pass.packages) {
    const wasm = join(row.manifest, "..", pass.output, `${row.name.replaceAll("-", "_")}.wasm`);
    console.log(`[DEBUG] staged ${row.name} ${existsSync(wasm) ? `${statSync(wasm).size} B` : "absent"} ${wasm.slice(repo.length + 1)}`);
  }
}

/** 🏃️ One owner script command in the foreground under a ticket-owned script envelope; its exit status becomes this process's. */
async function script(path: string, args: readonly string[]): Promise<void> {
  const { createScriptProcessEnvelope, scriptProcessEnvironment } = await import(join(repo, "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"));
  const environment = scriptProcessEnvironment(createScriptProcessEnvelope({ version: 1, owner: "ticket:wave-l1:script", maximumElapsedMilliseconds: 0 }, {}, Date.now()), process.env);
  const child = Bun.spawn([process.execPath, resolve(path), ...args], { cwd: resolve(path, ".."), env: environment, stdout: "inherit", stderr: "inherit" });
  process.exitCode = await child.exited;
}

const [command, ...rest] = process.argv.slice(2);
if (command === "demo" && rest.length) await demo(rest);
else if (command === "resume" && rest.length > 1) await demo(rest.slice(1), resolve(rest[0]!));
else if (command === "script" && rest.length) await script(rest[0]!, rest.slice(1));
else throw new Error("demo <plugin-id…> | resume <generation> <plugin-id…> | script <📜️script.ts> <argument…>");
