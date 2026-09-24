/** 🧬️ W1: the trusted-catalog codec probe, run per package against its current wasm-dev component and its
 * committed descriptor's declared kinds (same `semio-framework-plugin-describe codecs` call the bootstrap makes).
 * usage: bun w1-codec-probe.ts <plugin…>   (prints one PASS/FAIL row per package) */
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const emitter = join(repo, ".🧬semio/🦑️repo/⚡️cache/cargo/target/debug/semio-framework-plugin-describe");
const target = join(repo, ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev");
const scratch = mkdtempSync(join(repo, ".tmp-ticket/wp-w1/generated/codec-probe-"));
const plugins = readdirSync(join(repo, "✏️s/🔌️plugins"));
for (const pluginId of process.argv.slice(2)) {
  const dir = plugins.find((entry) => entry.replace(/^\P{L}+/u, "") === pluginId);
  if (!dir) { console.log(`MISS ${pluginId}`); continue; }
  const descriptor = JSON.parse(readFileSync(join(repo, "✏️s/🔌️plugins", dir, "🔣️.json"), "utf8"));
  const apps = descriptor.manifest.apps ?? [];
  const seen = new Set<string>();
  const kinds = [...(descriptor.manifest.artifactKinds ?? []), ...apps.flatMap((app: any) => app.artifactKinds ?? [])].filter((kind: any) => !seen.has(kind.id) && seen.add(kind.id));
  const wasm = join(target, `semio_s_plugin_${pluginId}.wasm`);
  if (!existsSync(wasm) || kinds.length === 0) { console.log(`SKIP ${pluginId} wasm=${existsSync(wasm)} kinds=${kinds.length}`); continue; }
  const failing: string[] = [];
  for (const kind of kinds) {
    const out = join(scratch, `${pluginId}-${kind.id}.json`);
    const probe = spawnSync(emitter, ["codecs", wasm, "--kinds", `${kind.id}=${kind.schema}`, "--out", out], { encoding: "utf8", timeout: 300_000 });
    if (probe.status !== 0) failing.push(`${kind.id}: ${(probe.stderr || probe.stdout).trim().split("\n").pop()?.replace(/.*guest fault /, "").slice(0, 160)}`);
  }
  console.log(`${failing.length ? "FAIL" : "PASS"} ${pluginId} kinds=${kinds.length} wasm=${statSync(wasm).mtime.toISOString()}${failing.map((row) => `\n    ${row}`).join("")}`);
}
rmSync(scratch, { recursive: true, force: true });
