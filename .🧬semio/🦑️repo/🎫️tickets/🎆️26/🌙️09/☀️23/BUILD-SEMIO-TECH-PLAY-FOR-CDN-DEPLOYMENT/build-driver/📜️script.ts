import { join, resolve } from "node:path";
import { runReleaseDriverV1, type ReleasePlanV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { componentWorkspacesV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";

/** 🧪️ Wave A1 demonstration: one cold release-driver run over a subset of hub components in a private generation under `⚡️cache/a1`, staged into `dist/a1-demo`. */
async function subset(profiles: readonly string[], packages: readonly string[], limitMs: number): Promise<void> {
  const repo = resolve(import.meta.dir, "../../../../../../../.."), cache = join(repo, ".🧬semio/🦑️repo/⚡️cache/a1"), hub = componentWorkspacesV1(repo, ["🌎️hub"])[0]!;
  const plan: ReleasePlanV1 = { version: 1, passes: profiles.map((profile) => ({ id: `${profile}-subset`, manifest: hub.manifest, packages: packages.map((name) => ({ name, manifest: hub.manifests[name]! })), target: "wasm32-wasip2", profile, output: "dist/a1-demo" })) };
  const controller = new AbortController(), stop = (): void => controller.abort(Error("demonstration stopped"));
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  const limit = setTimeout(() => controller.abort(Error(`demonstration limit ${limitMs} ms`)), limitMs);
  try {
    const outcome = await runReleaseDriverV1({ plan, cwd: repo, cacheRoot: cache, shared: { target: join(cache, "unused-t"), build: join(cache, "unused-b") }, generation: "fresh", leaseDirectory: join(repo, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), environment: process.env, signal: controller.signal, progressMs: 15_000 });
    console.log(`[DEBUG] a1 demonstration ${JSON.stringify({ staged: outcome.staged, units: outcome.units, report: outcome.report })}`);
  } finally { clearTimeout(limit); }
}

const [command, profiles, limit, ...packages] = process.argv.slice(2);
if (command !== "subset" || !profiles || !limit || !packages.length) throw new Error("subset <profile[,profile]> <limit-ms> <package…>");
await subset(profiles.split(","), packages, Number(limit));
