/** 🔭️ W4 (session 15): pre-checks the chain's NEXT gates for hosted packages before the chain reaches them — emits each selected package's
 * descriptor from its current `dist/component-dev` deliverable into a scratch dir (jco core extraction + the already-built
 * `semio-framework-plugin-describe` emitter, no cargo, no owner-root writes), holds it to the catalog manifest/descriptor allow-lists and
 * then runs the hub's own `trustedBootstrapSelectionFindingsV1` over the `all` selection (fresh descriptors for the emitted packages,
 * committed owner-root descriptors for the rest).
 *   bun wp-w4/w4-hosted-precheck.ts <scratch dir> <pluginId,…>   (pluginIds to emit fresh; e.g. the 9 stdio families + demonstrator) */
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { CATALOG_DESCRIPTOR_TOP_LEVEL, CATALOG_MANIFEST_FIELDS } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts";
import { trustedBootstrapSelectionFindingsV1 } from "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const [scratch, fresh] = [process.argv[2]!, new Set(process.argv[3]!.split(","))];
const registry = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"), "utf8")) as any[];
const jco = join(repo, "node_modules/@bytecodealliance/jco/dist/jco.js");
const emitter = join(repo, ".🧬semio/🦑️repo/⚡️cache/cargo/target/debug/semio-framework-plugin-describe");
const all = (process.env.W4_PRECHECK_ORDER ?? "stdio,stdio-image,stdio-media,stdio-cad,stdio-bim,stdio-mesh,stdio-pdf,stdio-office,stdio-semio,stdio-binary,gis,animate,architect,block,cad,dag,draw,energy,fem,flow,forms,imperative,layout,lowpoly,mathematical,norm,note,playbook,procedural,process,puzzle,raster,reasoning,remodel,sequence,shooting,sourcing,space,trinity,vcs,wfc,writer,demonstrator").split(",");
const problems: string[] = [];
const packages = all.map((pluginId) => {
  const row = registry.find((entry) => entry.pluginId === pluginId && entry.role === "plugin");
  if (!row) { problems.push(`${pluginId}: no plugin row in the generated registry`); return undefined; }
  let descriptor: Record<string, any>;
  if (fresh.has(pluginId) && existsSync(join(scratch, pluginId, "descriptor", "🔣️.json"))) {
    descriptor = JSON.parse(readFileSync(join(scratch, pluginId, "descriptor", "🔣️.json"), "utf8"));
    console.log(`[precheck] reused ${pluginId} from ${join(scratch, pluginId)}`);
  } else if (fresh.has(pluginId)) {
    const out = join(scratch, pluginId);
    mkdirSync(join(out, "core"), { recursive: true });
    mkdirSync(join(out, "descriptor"), { recursive: true });
    const component = join(repo, row.cratePath, "dist", "component-dev", row.wasmOut);
    const base = String(row.wasmOut).replace(/\.wasm$/u, "");
    const started = Date.now();
    execFileSync("node", [jco, "transpile", component, "-o", join(out, "core"), "--name", base, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], { cwd: repo, stdio: ["ignore", "ignore", "inherit"] });
    execFileSync(emitter, ["describe", component, "--core", join(out, "core", `${base}.core.wasm`), "--out", join(out, "descriptor")], { cwd: repo, stdio: ["ignore", "ignore", "inherit"] });
    descriptor = JSON.parse(readFileSync(join(out, "descriptor", "🔣️.json"), "utf8"));
    console.log(`[precheck] emitted ${pluginId} in ${Math.round((Date.now() - started) / 1000)} s: hosted=${(descriptor.manifest?.hostedArtifactKinds ?? []).length} owned=${(descriptor.manifest?.artifactKinds ?? []).length} apps=${(descriptor.manifest?.apps ?? []).length}`);
  } else {
    const path = join(repo, row.cratePath, "..", "..", "🔣️.json");
    if (!existsSync(path)) { problems.push(`${pluginId}: no committed descriptor`); return undefined; }
    descriptor = JSON.parse(readFileSync(path, "utf8"));
  }
  const top = Object.keys(descriptor).filter((key) => !CATALOG_DESCRIPTOR_TOP_LEVEL.has(key));
  const manifest = Object.keys(descriptor.manifest ?? {}).filter((key) => !CATALOG_MANIFEST_FIELDS.has(key));
  if (top.length || manifest.length) problems.push(`${pluginId}: unknown descriptor fields [${top}] manifest fields [${manifest}]`);
  const linked = pluginId === "stdio" || pluginId === "gis" ? `linked-${pluginId}` : null;
  return { pluginId, componentPackageId: `semio:${pluginId}`, outputName: String(row.wasmOut), linkedCodecRegistry: linked, linkedCodecRegistryPresent: linked !== null, descriptor };
}).filter((row) => row !== undefined);
const findings = trustedBootstrapSelectionFindingsV1(`local-${all.length}-packages-0000000000000000-open-v1`, packages as any);
for (const line of [...problems, ...findings]) console.log(`[precheck] FINDING ${line}`);
console.log(`[precheck] ${problems.length + findings.length === 0 ? "CLEAN" : "RED"} packages=${packages.length} fresh=${[...fresh].join(",")} problems=${problems.length} findings=${findings.length}`);
