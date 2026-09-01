/** 🔎️ Ticket-local MCP diagnostics through the repository's configured stdio entry point. */

//#region 🔌️Client
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { dirname, relative, resolve } from "node:path";
import { createReadStream, existsSync, readFileSync, statSync } from "node:fs";
import { execFileSync, spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
//#endregion 🔌️Client

//#region 🧪️ComponentProbe
function semanticComponent(value: unknown, decodeScene: (bytes: Uint8Array) => unknown): unknown {
  if (typeof value === "bigint") return value.toString();
  if (value instanceof Uint8Array) return Array.from(value);
  if (Array.isArray(value)) return value.map((item) => semanticComponent(item, decodeScene));
  if (typeof value === "string" && /^[\[{]/.test(value.trimStart())) {
    try { return semanticComponent(JSON.parse(value), decodeScene); } catch { return value; }
  }
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(Object.keys(value).sort().map((key) => {
      const item = Reflect.get(value, key);
      return [key, semanticComponent(key === "doc" && Array.isArray(item?.bytes) ? decodeScene(Uint8Array.from(item.bytes)) : item, decodeScene)];
    }));
  }
  return value;
}

function probeFlowSnapshot(): void {
  const source = readFileSync(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs"), "utf8");
  const start = source.indexOf("struct FlowSnapshotRetirement {");
  const end = source.indexOf("struct FlowOwnedFixtureRetirementFactory;", start);
  if (start < 0 || end < 0) throw new Error("Flow snapshot source boundaries changed");
  const production = source.slice(start, end);
  const harness = `
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
#[derive(Debug, PartialEq)]
enum SnapshotRetirementStep { Complete, Pending { released_items: usize, released_bytes: usize }, Blocked }
trait ErasedSnapshotRetirement {
  fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String>;
  fn terminal_is_empty(&self) -> bool;
}
trait SnapshotRetirementFactory<T> { fn retire(&self, snapshot: Arc<T>) -> Box<dyn ErasedSnapshotRetirement>; }
struct FlowFixture(Arc<AtomicUsize>);
impl Drop for FlowFixture { fn drop(&mut self) { self.0.fetch_add(1, Ordering::SeqCst); } }
struct FlowFixtureRetirement(Option<FlowFixture>);
impl FlowFixtureRetirement { fn new(value: FlowFixture) -> Self { Self(Some(value)) } }
impl ErasedSnapshotRetirement for FlowFixtureRetirement {
  fn close_step(&mut self, items: usize, _bytes: usize) -> Result<SnapshotRetirementStep, String> {
    if items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
    self.0 = None;
    Ok(SnapshotRetirementStep::Complete)
  }
  fn terminal_is_empty(&self) -> bool { self.0.is_none() }
}
${production}
#[test]
fn shared_readers_release_one_final_fixture_owner() {
  let drops = Arc::new(AtomicUsize::new(0));
  let snapshot = Arc::new(FlowFixture(drops.clone()));
  let mut readers = [
    std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(snapshot.clone())),
    std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(snapshot))
  ];
  for _ in 0..4096 {
    for reader in &mut readers { if !reader.terminal_is_empty() { reader.close_step(1, 256).unwrap(); } }
    if readers.iter().all(|reader| reader.terminal_is_empty()) {
      for reader in &mut readers { unsafe { std::mem::ManuallyDrop::drop(reader); } }
      assert_eq!(drops.load(Ordering::SeqCst), 1);
      return;
    }
  }
  panic!("shared snapshot readers retained each other's final-owner claim");
}
#[test]
fn zero_item_grant_does_not_consume_a_snapshot_claim() {
  let drops = Arc::new(AtomicUsize::new(0));
  let mut reader = std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(Arc::new(FlowFixture(drops.clone()))));
  assert_eq!(reader.close_step(0, 256).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
  assert!(!reader.terminal_is_empty());
  for _ in 0..4096 {
    if reader.close_step(1, 256).unwrap() == SnapshotRetirementStep::Complete {
      unsafe { std::mem::ManuallyDrop::drop(&mut reader); }
      assert_eq!(drops.load(Ordering::SeqCst), 1);
      return;
    }
  }
  panic!("snapshot did not retire after a zero-item grant");
}
`;
  const binary = resolve(dirname(fileURLToPath(import.meta.url)), process.platform === "win32" ? "🧪️flow-snapshot-isolated.exe" : "🧪️flow-snapshot-isolated");
  const compiled = spawnSync("rustc", ["--edition=2021", "--test", "-o", binary, "-"], { input: harness, encoding: "utf8", timeout: 30_000 });
  process.stdout.write(compiled.stdout ?? "");
  process.stderr.write(compiled.stderr ?? "");
  if (compiled.error) throw compiled.error;
  if (compiled.status !== 0) throw new Error(`Isolated production-method compile failed: ${compiled.status}`);
  const tested = spawnSync(binary, ["--nocapture"], { encoding: "utf8", timeout: 10_000 });
  process.stdout.write(tested.stdout ?? "");
  process.stderr.write(tested.stderr ?? "");
  if (tested.error) throw tested.error;
  process.exitCode = tested.status ?? 1;
}

async function descriptorSource(): Promise<void> {
  const pluginId = process.argv[3] ?? "demonstrator";
  const modulePath = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules", pluginId, `semio_s_plugin_${pluginId}_component.js`);
  const moduleUrl = pathToFileURL(modulePath).href;
  const { rewriteJcoAsyncResultLifting } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts")).href);
  const source = rewriteJcoAsyncResultLifting(readFileSync(modulePath, "utf8"))
    .replace(/from (['"])(\.[^'"]+)\1/g, (_match, _quote, specifier) => `from ${JSON.stringify(new URL(specifier, moduleUrl).href)}`)
    .replaceAll("import.meta.url", JSON.stringify(moduleUrl));
  await new Promise<void>((resolve, reject) => process.stdout.write(JSON.stringify(source), (error) => error ? reject(error) : resolve()));
}

async function probeJco(): Promise<void> {
  const { parse, transpile } = await import("@bytecodealliance/jco");
  const fixture = JSON.parse(readFileSync(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧪️fixtures/🔣️async-results.json"), "utf8"));
  for (const test of fixture.cases) {
    const wat = fixture.componentTemplate.replace("{{type}}", test.type).replace("{{params}}", test.params.join(" "));
    const result = await transpile(await parse(wat), { name: test.id, noTypescript: true });
    const source = new TextDecoder().decode(result.files[`${test.id}.js`]);
    console.log(`[DEBUG] ${test.id}: ${source.match(/taskReturn\.bind\([\s\S]*?useDirectParams: (true|false)/)?.[1]}`);
  }
}

async function probeDescriptor(): Promise<void> {
  const pluginId = process.argv[3] ?? "demonstrator";
  const modulePath = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules", pluginId, `semio_s_plugin_${pluginId}.js`);
  const { createActorApi } = await import(pathToFileURL(modulePath).href);
  const api = await createActorApi(`${pluginId}#descriptor-diagnostic`, 1n);
  console.log(`[DEBUG] Reading ${pluginId} descriptor`);
  const started = performance.now();
  let describe = api.describe;
  if (process.argv.includes("--corrected")) {
    const source = JSON.parse(execFileSync("bun", [process.argv[1]!, "descriptor-source", pluginId], { cwd: root, encoding: "utf8", maxBuffer: 8 * 1024 * 1024 }));
    const component = await import(`data:text/javascript;base64,${Buffer.from(source).toString("base64")}`);
    describe = () => component.describe.describe();
  }
  const descriptor = await describe();
  const { decodePackValue } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🟦️component.ts")).href);
  console.log(JSON.stringify({ pluginId, milliseconds: performance.now() - started, bytes: descriptor.length, descriptor: decodePackValue(descriptor) }, (_, value) => typeof value === "bigint" ? value.toString() : value));
}

async function probeDescriptorPack(): Promise<void> {
  const { createHash } = await import("node:crypto");
  const { decodePackValue, encodePackValue } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🟦️component.ts")).href);
  const bytes = readFileSync(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/demonstrator/🛂️descriptor.semio"));
  const descriptor = decodePackValue(bytes) as any;
  const roundTrip = encodePackValue(descriptor);
  const expected = descriptor.hashes.descriptorSha256;
  descriptor.hashes.descriptorSha256 = "";
  const actual = createHash("sha256").update(encodePackValue(descriptor)).digest("hex");
  console.log(`[DEBUG] descriptor byte round-trip=${bytes.equals(Buffer.from(roundTrip))} hash matches=${expected === actual} expected=${expected} actual=${actual}`);
  if (!bytes.equals(Buffer.from(roundTrip)) || expected !== actual) throw new Error("Descriptor encoding oracle mismatch");
}

async function probePublication(): Promise<void> {
  const { isDeepStrictEqual } = await import("node:util");
  const { demonstratorRuntimeModuleLayout, demonstratorRuntimeBuildVariants } = await import(pathToFileURL(resolve(root, "♻️mit-bestand/🧺️demonstrator/📜️script.ts")).href);
  const { PLAYGROUND_BUILD_TARGETS } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts")).href);
  const { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts")).href);
  const { decodePackValue, encodePackValue } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🟦️component.ts")).href);
  const { createHash } = await import("node:crypto");
  const roots = ["generator", ...demonstratorRuntimeBuildVariants("generator")].map((variant: string) => PLAYGROUND_BUILD_TARGETS.find((entry: any) => entry.variant === variant)?.pluginId);
  if (roots.some((id: unknown) => typeof id !== "string")) throw new Error("Unknown demonstrator runtime variant");
  const layout = demonstratorRuntimeModuleLayout(roots);
  const catalog = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
  for (const id of ["_vendor", "_shard"]) {
    if (!existsSync(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules", id))) throw new Error("Missing shared module directory " + id);
  }
  const modules = [...layout.pluginModuleDirNames.filter((id: string) => !id.startsWith("_")).map((id: string) => ({ id, directory: "🔌️plugin-modules" })), ...layout.extensionModuleDirNames.map((id: string) => ({ id, directory: "🔌️extension-modules" }))];
  const since = process.argv.find((argument) => argument.startsWith("--since="))?.slice(8);
  const minimumTime = since ? Date.parse(since) : 0;
  if (!Number.isFinite(minimumTime)) throw new Error("Invalid publication timestamp");
  const manifests: any[] = [];
  let failures = 0;
  for (const { id, directory } of modules) {
    try {
      const target = catalog.find((entry: any) => entry.pluginId === id);
      if (!target) throw new Error("Missing catalog target");
      const output = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev", directory, id);
      const base = target.wasmOut.replace(/\.wasm$/, "");
      const required = ["🔣️descriptor.json", "🛂️descriptor.semio", base + ".js", base + "_component.js", base + "_component.core.wasm"];
      for (const name of required) {
        const path = resolve(output, name);
        if (!existsSync(path)) throw new Error("Missing " + name);
        if (statSync(path).mtimeMs < minimumTime) throw new Error("Stale " + name);
      }
      const descriptor = JSON.parse(readFileSync(resolve(output, required[0]), "utf8"));
      if (descriptor.manifest?.pluginId !== id) throw new Error("Descriptor identity mismatch");
      const pack = readFileSync(resolve(output, required[1]));
      if (!isDeepStrictEqual(decodePackValue(pack), descriptor)) throw new Error("Pack and JSON descriptors differ");
      const expectedHash = descriptor.hashes.descriptorSha256;
      descriptor.hashes.descriptorSha256 = "";
      if (createHash("sha256").update(encodePackValue(descriptor)).digest("hex") !== expectedHash) throw new Error("Descriptor self-hash mismatch");
      if (process.argv.includes("--verify-hashes")) {
        const hash = createHash("sha256");
        for await (const chunk of createReadStream(resolve(output, required[4]))) hash.update(chunk);
        if (hash.digest("hex") !== descriptor.hashes.coreWasmSha256) throw new Error("Core component hash mismatch");
      }
      manifests.push({ pluginId: id, manifest: descriptor.manifest });
      console.log("[DEBUG] Publication verified: " + id);
    } catch (error) {
      failures++;
      console.error("[DEBUG] Publication rejected: " + id + ": " + String(error));
    }
  }
  const { buildContributionsJson } = await import(pathToFileURL(resolve(root, "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts")).href);
  const contributions = buildContributionsJson(manifests);
  console.log("[DEBUG] Verified publication manifests=" + manifests.length + "/" + modules.length + " contributionBytes=" + Buffer.byteLength(contributions));
  if (failures) throw new Error("Publication incomplete: " + failures + " modules rejected");
}

/** 🧾️ Every probed source is an individual `resolve(root, "…")` call — a computed spread over
 * a name array (as this once was for the nine flow extensions) is opaque to the taxonomy engine's
 * reference scanner: it cannot statically prove what a template-literal interpolation evaluates to,
 * so it can neither verify nor rewrite the reference when the named file moves. Spelling out every
 * path literally keeps each one an exact, provable, rewritable coordinate at zero runtime cost. */
async function probeSources(): Promise<void> {
  const { createHash } = await import("node:crypto");
  const absolutes = [
    resolve(root, "Cargo.toml"),
    resolve(root, "Cargo.lock"),
    resolve(root, ".cargo/config.toml"),
    resolve(root, "🧰️framework/🔨️modules/🧵️job/🦀️component.rs"),
    resolve(root, "🧰️framework/🔨️modules/⏳️async/⏱️clock/🦀️.rs"),
    resolve(root, "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts"),
    resolve(root, "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts"),
    resolve(root, "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts"),
    resolve(root, "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️component.rs"),
    resolve(root, "🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/🧪️fixture.json"),
    resolve(root, "🧰️framework/🔨️modules/🎭️actor/🪪️activation/📨️inbound/🧪️schema.json"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧪️fixtures/🔣️host-activation.json"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧪️fixtures/🔣️host-activation.schema.json"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️catalogue/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs"),
    resolve(root, "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx"),
    resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts"),
  ];
  const sources = absolutes.map((absolute) => {
    const path = relative(root, absolute);
    const bytes = readFileSync(absolute);
    return { path, bytes: bytes.length, modified: statSync(absolute).mtime.toISOString(), sha256: createHash("sha256").update(bytes).digest("hex") };
  });
  console.log(JSON.stringify({ captured: new Date().toISOString(), sources }, null, 2));
}

async function probePlugin(): Promise<void> {
  const pluginId = process.argv[3] ?? "demonstrator";
  const appId = process.argv[4] ?? "s.puzzle.puzzle3d@1/*#editor";
  const modulePath = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules", pluginId, `semio_s_plugin_${pluginId}.js`);
  const { createActorApi } = await import(pathToFileURL(modulePath).href);
  const { decodePackValue, decodeScenePackValue, encodePackValue, encodeAppCommand, decodeAppFrame } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/💻️os/🟦️component.ts")).href);
  const { createShardCommandIngressPages } = await import(pathToFileURL(resolve(root, "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts")).href);
  const descriptor = JSON.parse(readFileSync(resolve(dirname(modulePath), "🔣️descriptor.json"), "utf8"));
  const appIds = appId === "all" ? ["s.puzzle.puzzle3d@1/*#editor", "s.cad.cad@1/*#editor", "s.sourcing.curate@1/*#editor", "s.process.process3d@1/*#editor", "s.gis.gismap@1/*#editor"] : [appId];
  for (const [index, id] of appIds.entries()) {
    const api = await createActorApi(`${pluginId}#diagnostic-${index}`, BigInt(index + 1));
    console.log(`[DEBUG] Component loaded; opening ${id}`);
    try {
      const definition = descriptor.manifest.apps.find((candidate: { id: string }) => candidate.id === id);
      if (!definition?.windowKinds.length) throw new Error(`The staged descriptor must declare ${id} and its windows`);
      const result = await api.poll([{ kind: "instance-open", payload: { instance: 1, appId: id, actor: "local", config: [], assets: [], capabilities: [], quotas: [] } }], undefined, { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
      console.log(JSON.stringify(result, (_, value) => typeof value === "bigint" ? value.toString() : value));
      const example = process.argv.find((argument) => argument.startsWith("--example="))?.slice(10);
      const action = process.argv.find((argument) => argument.startsWith("--action="))?.slice(9);
      const drainTurns = Number(process.argv.find((argument) => argument.startsWith("--drain-turns="))?.slice(14) ?? "0");
      let pendingAcks: any[] = [];
      const pageMagic = Buffer.from("semio.typed-operation-page.v1\0");
      const ackMagic = Buffer.from("semio.typed-operation-ack.v1\0");
      const typedPage = (effect: any): { acknowledgement: any; lane: number; payload: Buffer } | undefined => {
        if (effect.tag !== "send-message" || effect.val.target.tag !== "shell") return undefined;
        const bytes = Buffer.from(effect.val.payload);
        if (!bytes.subarray(0, pageMagic.length).equals(pageMagic)) return undefined;
        const header = bytes.subarray(pageMagic.length);
        if (header.length < 30 || header.readUInt32LE(0) !== 1 || effect.val.target.val !== "1" || header[25]! > 11 || header.readUInt32LE(26) > 4_096 || header.length !== 30 + header.readUInt32LE(26)) throw new Error("Invalid typed publication authority");
        return { acknowledgement: { kind: "message", payload: { source: { tag: "shell", val: "1" }, payload: Array.from(Buffer.concat([ackMagic, header.subarray(0, 25)])) } }, lane: header[25]!, payload: header.subarray(30) };
      };
      const acknowledgements = (outcome: any): any[] => [
        ...outcome.uiPatches.map((patch: any) => ({ kind: "patch-ack", payload: { surface: patch.surface, revision: patch.revision } })),
        ...outcome.effects.flatMap((effect: any) => { const page = typedPage(effect); return page ? [page.acknowledgement] : []; }),
      ];
      const components = new Map<string, unknown>();
      const retainComponents = (outcome: any): void => {
        for (const patch of outcome.uiPatches) {
          const prefix = `${patch.surface.instance}:${patch.surface.surface}:`;
          for (const operation of patch.ops) {
            if (operation.tag === "upsert") {
              const node = decodePackValue(Uint8Array.from(operation.val.node)) as any;
              if (String(node.component?.value ?? "").startsWith("Unknown body:")) throw new Error(`Unrecognized body ${patch.surface.surface}`);
              components.set(prefix + node.id, node.component);
            } else if (operation.tag === "set-component") {
              components.set(prefix + operation.val.node, decodePackValue(Uint8Array.from(operation.val.component)));
            } else if (operation.tag === "remove") components.delete(prefix + operation.val);
          }
        }
      };
      const componentSnapshot = (): string => JSON.stringify(semanticComponent([...components].sort(([left], [right]) => left.localeCompare(right)), decodeScenePackValue));
      const printOutcome = (value: unknown) => console.log(JSON.stringify(value, (key, item) => typeof item === "bigint" ? item.toString() : item instanceof Uint8Array ? key === "node" || key === "component" ? decodePackValue(item) : Array.from(item) : key === "doc" && Array.isArray(item?.bytes) ? { snapshot: decodeScenePackValue(Uint8Array.from(item.bytes)) } : item));
      let sequence = 0;
      const sendCommand = async (value: Record<string, unknown>): Promise<any[]> => {
        sequence += 1;
        const command = encodeAppCommand(Object.fromEntries(Object.entries(value).map(([key, body]) => [key, { ...(body as object), seq: sequence }])) as any);
        const pages = createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: BigInt(sequence), command });
        const effects: any[] = [];
        let outcome: any;
        let ingressComplete = false;
        let publicationFault: string | undefined;
        for (let turn = 0; turn < 4096; turn++) {
          await new Promise<void>((resolve) => setImmediate(resolve));
          outcome = await api.poll(pendingAcks, pages[turn], { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
          if (turn % 128 === 0) console.log(`[DEBUG] Command ${sequence} turn=${turn} status=${JSON.stringify(outcome.status)} ingress=${JSON.stringify(outcome.commandIngress, (_key, value) => typeof value === "bigint" ? value.toString() : value)}`);
          retainComponents(outcome);
          pendingAcks = acknowledgements(outcome);
          if (outcome.uiPatches.length) printOutcome({ stage: `command-${sequence}`, turn, ...outcome });
          if (outcome.effects.length) {
            for (const effect of outcome.effects) {
              const page = typedPage(effect);
              if (page) {
                console.log(`[DEBUG] Command ${sequence} publication lane=${page.lane} bytes=${page.payload.length}`);
                if (page.lane === 11) publicationFault = page.payload.toString("utf8");
                continue;
              }
              if (effect.tag === "send-message" && effect.val.target.tag === "shell") {
                const frame = decodeAppFrame(Uint8Array.from(effect.val.payload));
                if ("Error" in frame) throw new Error(JSON.stringify(decodePackValue(Uint8Array.from(frame.Error.fault))));
                console.log(`[DEBUG] Command frame ${sequence}: ${JSON.stringify(frame, (_key, item) => typeof item === "bigint" ? item.toString() : item)}`);
              } else effects.push(effect);
            }
          }
          if (outcome.status.tag === "faulted" || outcome.commandIngress.tag === "fault" || outcome.commandIngress.tag === "backpressure") {
            throw new Error(JSON.stringify({ status: outcome.status, ingress: outcome.commandIngress }, (_key, item) => typeof item === "bigint" ? item.toString() : item instanceof Uint8Array ? decodePackValue(item) : item));
          }
          ingressComplete ||= outcome.commandIngress.tag === "command-complete";
          if (ingressComplete && outcome.status.tag === "idle" && !pendingAcks.length) {
            if (publicationFault) throw new Error(publicationFault);
            console.log(`[DEBUG] Command ${sequence} completed after ${turn + 1} turns with ${effects.length} host effects`);
            return effects;
          }
        }
        throw new Error(`Command ${sequence} did not settle in 4096 turns`);
      };
      const invoke = async (actionId: string, args: Record<string, unknown>): Promise<void> => {
        const windowId = definition.windowKinds[0].id;
        const modeId = definition.defaultModeId ?? definition.modes[0].id;
        console.log(`[DEBUG] Invoking ${actionId}: ${JSON.stringify(args)}`);
        const invocation = { address: { pluginId, appId: id, modeId, windowKindId: windowId, windowInstanceId: windowId, actionId }, arguments: { ...args, windowId } };
        const effects = await sendCommand({ Command: { command: encodePackValue(invocation), view_state: encodePackValue({ activeModeId: modeId, activeWindowKindId: windowId, windowId }) } });
        for (const effect of effects) {
          if (effect.tag === "load-document") await sendCommand({ LoadDocument: { pack: effect.val.pack, spr: effect.val.spr } });
          else console.log(`[DEBUG] Unhandled effect ${effect.tag}`);
        }
      };
      const panelBodies = (panels: any[]): string[] => panels.flatMap((panel) => [...(panel.bodyKey ? [panel.bodyKey] : []), ...panelBodies(panel.children ?? [])]);
      const surfaces = [...new Set(process.argv.includes("--all-surfaces")
        ? [...definition.windowKinds.map((window: any) => window.bodyKey), ...panelBodies(definition.panelTabs ?? [])].filter((body): body is string => typeof body === "string" && body.length > 0)
        : process.argv.slice(5).filter((argument) => !argument.startsWith("--")))];
      const refreshSurfaces = async (stage: string, requireAll: boolean): Promise<void> => {
      let events = [...pendingAcks, ...surfaces.map((surface) => ({ kind: "surface-visible", payload: { surface: { instance: 1, surface } } }))];
      pendingAcks = [];
      const published = new Set<string>();
      let settled = surfaces.length === 0;
      let publicationFault: string | undefined;
      for (let turn = 0; surfaces.length && turn < 4096; turn++) {
        await new Promise<void>((resolve) => setImmediate(resolve));
        const outcome = await api.poll(events.length ? events : [{ kind: "wake" }], undefined, { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
        retainComponents(outcome);
        if (outcome.uiPatches.length || outcome.effects.length || turn % 128 === 0) printOutcome({ stage, turn, ...outcome });
        if (outcome.status.tag === "faulted") throw new Error(`Surface turn faulted for ${id}`);
        for (const effect of outcome.effects) {
          const page = typedPage(effect);
          if (page) {
            if (page.lane === 11) publicationFault = page.payload.toString("utf8");
          } else if (effect.tag === "send-message" && effect.val.target.tag === "shell") {
            const frame = decodeAppFrame(Uint8Array.from(effect.val.payload));
            if ("Error" in frame) throw new Error(JSON.stringify(decodePackValue(Uint8Array.from(frame.Error.fault))));
          }
        }
        outcome.uiPatches.forEach((patch) => {
          published.add(patch.surface.surface);
        });
        events = acknowledgements(outcome);
        if (outcome.status.tag === "idle" && !events.length && turn >= drainTurns) { settled = true; break; }
      }
      if (publicationFault) throw new Error(publicationFault);
      if (!settled) throw new Error(`Surface refresh did not settle in 4096 turns for ${id}`);
      if (requireAll && surfaces.some((surface) => !published.has(surface))) throw new Error(`Missing surfaces: ${surfaces.filter((surface) => !published.has(surface)).join(", ")}`);
      if (surfaces.length) console.log(`[DEBUG] Published ${published.size} requested surfaces for ${id}`);
      };
      await refreshSurfaces("initial", true);
      if (example !== undefined) {
        await invoke("setActiveExample", { exampleId: example });
        await refreshSurfaces("example", false);
      }
      if (action) {
        const beforeAction = componentSnapshot();
        const args = Object.fromEntries(process.argv.filter((argument) => argument.startsWith("--string=")).map((argument) => {
          const value = argument.slice(9);
          const separator = value.indexOf("=");
          if (separator < 1) throw new Error("String arguments require --string=key=value");
          return [value.slice(0, separator), value.slice(separator + 1)];
        }));
        await invoke(action, args);
        await refreshSurfaces("after-action", false);
        if (process.argv.includes("--require-change") && beforeAction === componentSnapshot()) throw new Error(`Action ${action} left every retained component unchanged`);
        console.log(`[DEBUG] Action ${action} changed component content=${beforeAction !== componentSnapshot()}`);
      }
      if (process.argv.includes("--close")) {
        let closed = false;
        for (let turn = 0; turn < 4096; turn++) {
          await new Promise<void>((resolve) => setImmediate(resolve));
          const outcome = await api.poll([...pendingAcks, ...(turn ? [{ kind: "wake" }] : [{ kind: "instance-close", payload: { instance: 1 } }])], undefined, { fuel: 50_000_000, wallMs: 100, maxEffects: 64, maxPatchBytes: 1 << 20 });
          pendingAcks = acknowledgements(outcome);
          if (outcome.status.tag === "faulted") throw new Error(`Close fault: ${JSON.stringify(decodePackValue(outcome.status.val))}`);
          if (outcome.status.tag === "idle" && !pendingAcks.length) { closed = true; console.log(`[DEBUG] Closed ${id} after ${turn + 1} turns`); break; }
          if (turn % 128 === 0) console.log(`[DEBUG] Closing ${id}: turn ${turn}`);
        }
        if (!closed) throw new Error(`Close did not become idle for ${id}`);
      }
    } catch (error) {
      console.error(error);
      process.exitCode = 1;
    }
  }
}
//#endregion 🧪️ComponentProbe

//#region 🔎️Inspect
async function inspectRepo(): Promise<void> {
const client = new Client({ name: "demonstrator-ticket", version: "1.0.0" }, { capabilities: {} });
const transport = new StdioClientTransport({
  command: "bun",
  args: ["./📜️script.ts", "dev", "mcp", "stdio", "codex"],
  cwd: root,
  env: { ...process.env, SEMIO_BUILD_BUDGET_MS: "60000" },
  stderr: "inherit",
});
try {
  await client.connect(transport);
  console.log(JSON.stringify({ kind: "goals", result: await client.readResource({ uri: "repo://goals" }) }));
  const listed = await client.listTools();
  console.log(JSON.stringify({ kind: "ticket-tools", tools: listed.tools.filter((tool) => /ticket_(open|reopen|close|read|list)/.test(tool.name)) }));
} catch (error) {
  console.error(String(error));
  process.exitCode = 1;
} finally {
  await client.close();
}
}

try {
  if (process.argv[2] === "probe-plugin") await probePlugin();
  else if (process.argv[2] === "probe-flow-snapshot") probeFlowSnapshot();
  else if (process.argv[2] === "probe-descriptor") await probeDescriptor();
  else if (process.argv[2] === "probe-descriptor-pack") await probeDescriptorPack();
  else if (process.argv[2] === "probe-publication") await probePublication();
  else if (process.argv[2] === "probe-sources") await probeSources();
  else if (process.argv[2] === "probe-jco") await probeJco();
  else if (process.argv[2] === "descriptor-source") await descriptorSource();
  else await inspectRepo();
} catch (error) {
  console.error(error);
  process.exitCode = 1;
}
//#endregion 🔎️Inspect
