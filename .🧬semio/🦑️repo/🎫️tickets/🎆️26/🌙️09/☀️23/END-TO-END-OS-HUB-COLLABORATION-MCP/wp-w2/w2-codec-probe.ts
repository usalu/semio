/** 🧬️ W2: the trusted-catalog codec probe for every selectable package, per declared kind, against the current wasm-dev
 * component and the committed descriptor (`🔣️.json`), using the same `semio-framework-plugin-describe codecs` call the
 * bootstrap makes. Prints one row per kind (PASS hash / FAIL reason, seconds) plus the committed descriptor's own
 * openability facts (editor app bound to the kind, first window kind).
 * usage: bun w2-codec-probe.ts <emitter> <plugin…|all> */
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const ALL = "stdio,gis,animate,architect,block,cad,dag,demonstrator,draw,energy,fem,flow,forms,imperative,layout,lowpoly,mathematical,norm,note,playbook,procedural,process,puzzle,raster,reasoning,remodel,sequence,shooting,sourcing,space,trinity,vcs,wfc,writer".split(",");
const emitter = process.argv[2]!;
const selection = process.argv[3] === "all" ? ALL : process.argv.slice(3);
const target = join(repo, ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev");
const scratch = mkdtempSync(join(repo, ".tmp-ticket/wp-w2/generated/codec-probe-"));
const plugins = readdirSync(join(repo, "✏️s/🔌️plugins"));
let failed = 0;
for (const pluginId of selection) {
  const dir = plugins.find((entry) => entry.replace(/^\P{L}+/u, "") === pluginId);
  if (!dir) { console.log(`MISS ${pluginId}`); failed += 1; continue; }
  const descriptor = JSON.parse(readFileSync(join(repo, "✏️s/🔌️plugins", dir, "🔣️.json"), "utf8"));
  const apps = descriptor.manifest.apps ?? [];
  const seen = new Set<string>();
  const kinds = [...(descriptor.manifest.artifactKinds ?? []), ...apps.flatMap((app: any) => app.artifactKinds ?? [])].filter((kind: any) => !seen.has(kind.id) && seen.add(kind.id));
  const wasm = join(target, `semio_s_plugin_${pluginId}.wasm`);
  console.log(`== ${pluginId} kinds=${kinds.length} deps=${JSON.stringify((descriptor.manifest.dependencies ?? []).map((dep: any) => dep.pluginId ?? dep.id ?? dep))} wasm=${existsSync(wasm) ? `${(statSync(wasm).size / 1048576).toFixed(1)}MB ${statSync(wasm).mtime.toISOString()}` : "missing"}`);
  if (!existsSync(wasm) || kinds.length === 0) { failed += 1; continue; }
  for (const kind of kinds) {
    const editor = apps.find((app: any) => app.role === "editor" && app.dialect?.artifactKind === kind.id);
    const opens = editor ? `editor=${editor.id} window=${(editor.windowKinds ?? [])[0]?.id ?? "NONE"}` : "no-editor";
    const out = join(scratch, `${pluginId}-${kind.id}.json`);
    const started = Date.now();
    const probe = spawnSync(emitter, ["codecs", wasm, "--kinds", `${kind.id}=${kind.schema}`, "--out", out], { encoding: "utf8", timeout: 600_000 });
    const seconds = ((Date.now() - started) / 1000).toFixed(1);
    if (probe.status === 0) {
      const row = JSON.parse(readFileSync(out, "utf8")).rows?.[0];
      console.log(`PASS ${pluginId} ${kind.id}=${kind.schema} hash=${row?.packSchemaHash} ${opens} ${seconds}s`);
    } else {
      failed += 1;
      console.log(`FAIL ${pluginId} ${kind.id}=${kind.schema} ${opens} ${seconds}s status=${probe.status} signal=${probe.signal}\n    ${(probe.stderr || probe.stdout).trim().split("\n").slice(-2).join(" | ").slice(0, 400)}`);
    }
  }
}
rmSync(scratch, { recursive: true, force: true });
console.log(`DONE failed=${failed}`);
