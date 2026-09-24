"""🧩️ S15: the hub bootstrap materializes each package's trusted plugin module bundle (schema v3) and the
generation fence, candidate staging and rotation read carry it. Anchored assert-once replacements."""
import sys
P = "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts"
s = open(P, encoding="utf-8").read()

def rep(old, new):
    global s
    n = s.count(old)
    if n != 1:
        sys.exit(f"anchor count {n}: {old[:160]!r}")
    s = s.replace(old, new)

rep('''import { buildClosedBrowserActorArtifactV1, type ClosedBrowserActorArtifactV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";''',
'''import { buildClosedBrowserActorArtifactV1, type ClosedBrowserActorArtifactV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";
import { ensureGuestSlimTypstFontsAt, ensurePreview2ShimVendorAt, hostShimSource, PLUGIN_HOST_SHIM_FILE, PREVIEW2_VENDOR_RELATIVE, pluginComponentBridgeSource, transpilePluginComponentAsync } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { MODULE_BRIDGE_FILE, moduleDirectoryName } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { decodeTrustedPluginModuleBundleV1, encodeTrustedPluginModuleBundleV1, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE, TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES, TRUSTED_PLUGIN_MODULE_SCHEMA, utf8OrderV1, type TrustedPluginModuleBundleV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts";''')

rep('''type TrustedBootstrapGenerationReceiptV1 = Readonly<{
  identity: TrustedBootstrapIdentityV1;
  dependencies: readonly TrustedBootstrapIdentityV1[];
  component: Readonly<{ byteLength: number; sha256: string }>;
  descriptor: Readonly<{ byteLength: number; sha256: string }>;
  executionProtocol: Readonly<DocumentExecutionProtocolV1>;
  browserActor: TrustedBootstrapBrowserActorV1;
}>;''', '''type TrustedBootstrapGenerationReceiptV1 = Readonly<{
  identity: TrustedBootstrapIdentityV1;
  dependencies: readonly TrustedBootstrapIdentityV1[];
  component: Readonly<{ byteLength: number; sha256: string }>;
  descriptor: Readonly<{ byteLength: number; sha256: string }>;
  executionProtocol: Readonly<DocumentExecutionProtocolV1>;
  browserActor: TrustedBootstrapBrowserActorV1;
  pluginModule: TrustedBootstrapPluginModuleRecordV1;
}>;

/** 🧩️ Where a package record finds its plugin module manifest: `TrustedBundlePluginModuleV1`. */
type TrustedBootstrapPluginModuleRecordV1 = Readonly<{ path: string; byteLength: number; sha256: string; blake3: string }>;

/** 🧩️ Admits one `TrustedBundlePluginModuleV1` record at its own package's fixed manifest path. */
function trustedBootstrapPluginModuleRecordV1(candidate: unknown, pluginId: string): TrustedBootstrapPluginModuleRecordV1 {
  const row = documentOpenNeutralObject(candidate, ["path", "byteLength", "sha256", "blake3"]);
  if (row.path !== trustedBootstrapPluginModuleManifestPath(pluginId) || !Number.isSafeInteger(row.byteLength) || (row.byteLength as number) <= 0 || (row.byteLength as number) > TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES || !/^[0-9a-f]{64}$/u.test(String(row.sha256)) || !/^[0-9a-f]{64}$/u.test(String(row.blake3)))
    throw new Error("trusted plugin module record is not its package's exact manifest");
  return Object.freeze({ path: row.path as string, byteLength: row.byteLength as number, sha256: row.sha256 as string, blake3: row.blake3 as string });
}

/** 🧩️ The generation-relative manifest path of one package's plugin module. */
function trustedBootstrapPluginModuleManifestPath(pluginId: string): string {
  return `packages/${pluginId}/plugin-module.json`;
}

/** 🗃️ The generation-relative content-addressed path of one plugin module file. */
function trustedBootstrapPluginModuleBlobPath(sha256: string): string {
  return `plugin-modules/${sha256}`;
}

/** 🧬️ Frames one plugin module record into the generation exactly as `trusted_profile_generation` does. */
function trustedBootstrapPluginModuleEncoding(record: TrustedBootstrapPluginModuleRecordV1): Buffer {
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(record.byteLength));
  return Buffer.concat([trustedBootstrapField(record.path), length, trustedBootstrapField(Buffer.from(record.sha256, "hex")), trustedBootstrapField(Buffer.from(record.blake3, "hex"))]);
}

/** 🧩️ Reads one generation's plugin module manifest, bound to its record and its package's digests. */
function trustedBootstrapReadPluginModule(root: string, receipt: TrustedBootstrapGenerationReceiptV1, check: () => void): TrustedPluginModuleBundleV1 {
  const bytes = readStableBuildFile(join(root, receipt.pluginModule.path), TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, { remaining: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES }, check);
  if (bytes.byteLength !== receipt.pluginModule.byteLength || createHash("sha256").update(bytes).digest("hex") !== receipt.pluginModule.sha256 || blake3Hex(bytes) !== receipt.pluginModule.blake3) throw new Error("generation plugin module manifest differs from its record");
  return decodeTrustedPluginModuleBundleV1(bytes, { ...receipt.identity, componentSha256: receipt.component.sha256, descriptorByteSha256: receipt.descriptor.sha256 });
}

/** 🛑️ An abort signal for one child process that follows the bootstrap's own cancellation and deadline. */
function trustedBootstrapAbortSignal(control: FreshBuildControlV1): { readonly signal: AbortSignal; close(): void } {
  const abort = new AbortController();
  const poll = setInterval(() => {
    if (control.cancelled() || control.remainingMs() <= 0) abort.abort(new Error("trusted catalog build cancelled"));
  }, 250);
  return { signal: abort.signal, close: () => clearInterval(poll) };
}

/**
 * 🧩️ Materializes one package's browser plugin module from its own fresh component and staged descriptor —
 * host shim, jco component module, core Wasm (never re-optimized, so it stays the exact module the
 * descriptor's `coreWasmSha256` names), bridge, both descriptor forms and the vendored imports — and stages
 * it content-addressed in the generation: every file once under `plugin-modules/<sha256>` (shared across
 * packages), the canonical manifest at `packages/<plugin>/plugin-module.json`.
 */
async function trustedBootstrapPluginModuleV1(repoRoot: string, request: Readonly<{ pluginId: string; outputName: string }>, stage: string, stageRoot: string, work: string, receipt: FreshComponentReceiptV1, control: FreshBuildControlV1, check: () => void): Promise<TrustedBootstrapPluginModuleRecordV1> {
  const moduleRoot = join(work, "plugin-module");
  const directory = moduleDirectoryName(request.pluginId);
  const moduleDir = join(moduleRoot, directory);
  const vendor = join(moduleRoot, PREVIEW2_VENDOR_RELATIVE);
  mkdirSync(moduleDir, { recursive: true, mode: 0o700 });
  ensurePreview2ShimVendorAt(vendor, repoRoot);
  if (!ensureGuestSlimTypstFontsAt(moduleRoot, repoRoot)) throw new Error("trusted plugin module needs the staged guestslim typst font seed");
  const descriptor = trustedBootstrapReadRegular(join(stage, "descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, `fresh ${request.pluginId} descriptor for its plugin module`, check);
  if (descriptor.byteLength !== receipt.descriptor.byteLength || createHash("sha256").update(descriptor).digest("hex") !== receipt.descriptor.sha256) throw new Error("fresh descriptor differs from its receipt before plugin module materialization");
  const componentBase = `${request.outputName.slice(0, -".wasm".length)}_component`;
  writeFileSync(join(moduleDir, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  const abort = trustedBootstrapAbortSignal(control);
  try {
    await transpilePluginComponentAsync(join(stage, "component.wasm"), moduleDir, componentBase, { repoRoot, preview2VendorDir: vendor, signal: abort.signal, optimize: false });
  } finally {
    abort.close();
  }
  check();
  writeFileSync(join(moduleDir, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, request.outputName));
  writeFileSync(join(moduleDir, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE), `${JSON.stringify(packValueToExactJson(decodePackValue(descriptor)), null, 2)}\\n`);
  writeFileSync(join(moduleDir, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE), descriptor);
  rmSync(join(moduleDir, `${componentBase}.d.ts`), { force: true });
  rmSync(join(moduleDir, "interfaces"), { recursive: true, force: true });
  const core = readStableBuildFile(join(moduleDir, `${componentBase}.core.wasm`), TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES, { remaining: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES }, check);
  if (createHash("sha256").update(core).digest("hex") !== receipt.coreSha256) throw new Error(`trusted ${request.pluginId} plugin module core differs from the core its descriptor names`);
  const walk = (relative: string): string[] =>
    readdirSync(join(moduleRoot, relative), { withFileTypes: true }).flatMap((entry) => {
      const path = relative ? `${relative}/${entry.name}` : entry.name;
      if (entry.isDirectory()) return walk(path);
      if (!entry.isFile() || entry.name.startsWith(".")) throw new Error(`trusted plugin module carries a non-file ${path}`);
      return [path];
    });
  const blobs = join(stageRoot, "plugin-modules");
  mkdirSync(blobs, { recursive: true, mode: 0o700 });
  const files = walk("")
    .sort(utf8OrderV1)
    .map((path) => {
      check();
      const bytes = readStableBuildFile(join(moduleRoot, path), TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES, { remaining: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES }, check);
      const file = { path, byteLength: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex"), blake3: blake3Hex(bytes) };
      if (!existsSync(join(blobs, file.sha256))) trustedBootstrapWriteNew(join(blobs, file.sha256), bytes, check);
      return file;
    });
  const bundle = {
    schema: TRUSTED_PLUGIN_MODULE_SCHEMA,
    pluginId: request.pluginId,
    packageId: receipt.packageId,
    version: receipt.version,
    sourceComponentSha256: receipt.component.sha256,
    sourceDescriptorByteSha256: receipt.descriptor.sha256,
    moduleDirectory: directory,
    entry: `${directory}/${MODULE_BRIDGE_FILE}`,
    files,
  };
  const manifest = encodeTrustedPluginModuleBundleV1(bundle);
  decodeTrustedPluginModuleBundleV1(manifest, { pluginId: request.pluginId, packageId: receipt.packageId, version: receipt.version, componentSha256: receipt.component.sha256, descriptorByteSha256: receipt.descriptor.sha256 });
  trustedBootstrapWriteNew(join(stage, "plugin-module.json"), manifest, check);
  trustedBootstrapFsyncDirectory(blobs);
  rmSync(moduleRoot, { recursive: true, force: true });
  return Object.freeze({ path: trustedBootstrapPluginModuleManifestPath(request.pluginId), byteLength: manifest.byteLength, sha256: createHash("sha256").update(manifest).digest("hex"), blake3: blake3Hex(manifest) });
}''')

rep('''    browserActor: trustedBootstrapBrowserActorV1(record.browserActor, { componentSha256: record.component.sha256, descriptorByteSha256: record.descriptor.sha256 }, record.browserActor?.kind === "closed-browser-actor" ? "wasm" : "react"),
  });
}''', '''    browserActor: trustedBootstrapBrowserActorV1(record.browserActor, { componentSha256: record.component.sha256, descriptorByteSha256: record.descriptor.sha256 }, record.browserActor?.kind === "closed-browser-actor" ? "wasm" : "react"),
    pluginModule: trustedBootstrapPluginModuleRecordV1(record.pluginModule, identity.pluginId),
  });
}''')

rep('''    const plugins = [...bundleReceipts.keys()].sort();
    const carriesActor = (plugin: string): boolean => bundleReceipts.get(plugin)!.browserActor.kind === "closed-browser-actor";
    const directories: (readonly [string, readonly string[]])[] = [
      [root, ["packages", "trusted-catalog.json"]],
      [join(root, "packages"), plugins],
    ];
    for (const plugin of plugins) {
      directories.push([join(root, "packages", plugin), carriesActor(plugin) ? ["browser", "component.wasm", "descriptor.semio"] : ["component.wasm", "descriptor.semio"]]);
      if (carriesActor(plugin)) directories.push([join(root, "packages", plugin, "browser"), ["closed-actor.mjs"]]);
    }''', '''    const plugins = [...bundleReceipts.keys()].sort();
    const carriesActor = (plugin: string): boolean => bundleReceipts.get(plugin)!.browserActor.kind === "closed-browser-actor";
    const moduleFiles = new Map<string, Readonly<{ byteLength: number; sha256: string }>>();
    for (const plugin of plugins) {
      for (const file of trustedBootstrapReadPluginModule(root, receipts.get(plugin)!, check).files) {
        const known = moduleFiles.get(file.sha256);
        if (known && known.byteLength !== file.byteLength) throw new Error("generation plugin modules disagree about one content address");
        moduleFiles.set(file.sha256, { byteLength: file.byteLength, sha256: file.sha256 });
      }
    }
    const directories: (readonly [string, readonly string[]])[] = [
      [root, ["packages", "plugin-modules", "trusted-catalog.json"]],
      [join(root, "packages"), plugins],
      [join(root, "plugin-modules"), [...moduleFiles.keys()]],
    ];
    for (const plugin of plugins) {
      directories.push([join(root, "packages", plugin), carriesActor(plugin) ? ["browser", "component.wasm", "descriptor.semio", "plugin-module.json"] : ["component.wasm", "descriptor.semio", "plugin-module.json"]]);
      if (carriesActor(plugin)) directories.push([join(root, "packages", plugin, "browser"), ["closed-actor.mjs"]]);
    }''')

rep('''      files.push({ path: join(root, "packages", plugin, "descriptor.semio"), byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });''',
'''      files.push({ path: join(root, "packages", plugin, "descriptor.semio"), byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
      files.push({ path: join(root, receipt.pluginModule.path), byteLength: receipt.pluginModule.byteLength, sha256: receipt.pluginModule.sha256, maximum: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES });''')

rep('''    const admission = { remaining: files.reduce((sum, file) => sum + file.maximum, 0) };
    for (const file of files) {
      if (!Number.isSafeInteger(file.byteLength) || file.byteLength <= 0 || file.byteLength > file.maximum || !/^[0-9a-f]{64}$/u.test(file.sha256)) throw new Error("generation file receipt is not bounded");''',
'''    for (const file of moduleFiles.values()) files.push({ path: join(root, trustedBootstrapPluginModuleBlobPath(file.sha256)), byteLength: file.byteLength, sha256: file.sha256, maximum: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES });
    const admission = { remaining: files.reduce((sum, file) => sum + file.maximum, 0) };
    for (const file of files) {
      if (!Number.isSafeInteger(file.byteLength) || file.byteLength <= 0 || file.byteLength > file.maximum || !/^[0-9a-f]{64}$/u.test(file.sha256)) throw new Error("generation file receipt is not bounded");''')

rep('''    const browserActors = new Map<string, TrustedBootstrapBrowserActorV1>();
    const descriptorJson = new Map<string, Record<string, any>>();''', '''    const browserActors = new Map<string, TrustedBootstrapBrowserActorV1>();
    const pluginModules = new Map<string, TrustedBootstrapPluginModuleRecordV1>();
    const descriptorJson = new Map<string, Record<string, any>>();''')

rep('''        browserActors.set(request.pluginId, actor);
        receipts.set(request.pluginId, receipt);''', '''        browserActors.set(request.pluginId, actor);
        pluginModules.set(request.pluginId, await trustedBootstrapPluginModuleV1(repoRoot, request, stage, stageRoot, target, receipt, control, checkBuild));
        receipts.set(request.pluginId, receipt);''')

rep('''        browserActor: browserActors.get(identity.pluginId)!,
        codecCount: codecs[identity.pluginId]!.length,''', '''        browserActor: browserActors.get(identity.pluginId)!,
        pluginModule: pluginModules.get(identity.pluginId)!,
        codecCount: codecs[identity.pluginId]!.length,''')

rep('''      browserActor: browserActors.get(plugin)!,
      nativeCodecs: codecs[plugin],''', '''      browserActor: browserActors.get(plugin)!,
      pluginModule: pluginModules.get(plugin)!,
      nativeCodecs: codecs[plugin],''')

rep('''    const bundle = {
      schemaVersion: 2,
      profiles: [''', '''    const bundle = {
      schemaVersion: 3,
      profiles: [''')

rep('''    pieces.push(
      trustedBootstrapBrowserActorEncoding(selected.browserActor, { componentSha256: selected.componentSha256, descriptorByteSha256: selected.descriptorSha256 }, trustedBootstrapPackageRenderer(profile, selected.pluginId)),
    );
    pieces.push(trustedBootstrapDependencyEncoding(selected.dependencies));''', '''    pieces.push(
      trustedBootstrapBrowserActorEncoding(selected.browserActor, { componentSha256: selected.componentSha256, descriptorByteSha256: selected.descriptorSha256 }, trustedBootstrapPackageRenderer(profile, selected.pluginId)),
    );
    pieces.push(trustedBootstrapPluginModuleEncoding(trustedBootstrapPluginModuleRecordV1(selected.pluginModule, selected.pluginId)));
    pieces.push(trustedBootstrapDependencyEncoding(selected.dependencies));''')

rep('''    if (bundle.schemaVersion !== 2 || bundle.profiles?.length !== 1 || !Array.isArray(bundle.packages) || bundle.packages.length === 0 || bundle.profiles[0]?.id !== current.profileId || bundle.profiles[0]?.generationId !== current.generationId)''',
'''    if (bundle.schemaVersion !== 3 || bundle.profiles?.length !== 1 || !Array.isArray(bundle.packages) || bundle.packages.length === 0 || bundle.profiles[0]?.id !== current.profileId || bundle.profiles[0]?.generationId !== current.generationId)''')

rep('''      const files: { relative: string; maximum: number }[] = [];
      for (const plugin of stagedPlugins) {
        mkdirSync(stagedCarriesActor(plugin) ? join(stageRoot, "packages", plugin, "browser") : join(stageRoot, "packages", plugin), { recursive: true, mode: 0o700 });
        files.push({ relative: `packages/${plugin}/component.wasm`, maximum: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES });
        files.push({ relative: `packages/${plugin}/descriptor.semio`, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
        if (stagedCarriesActor(plugin)) files.push({ relative: `packages/${plugin}/browser/closed-actor.mjs`, maximum: DOCUMENT_BROWSER_ACTOR_MAX_BYTES });
      }''', '''      const files: { relative: string; maximum: number }[] = [];
      mkdirSync(join(stageRoot, "plugin-modules"), { recursive: true, mode: 0o700 });
      for (const plugin of stagedPlugins) {
        mkdirSync(stagedCarriesActor(plugin) ? join(stageRoot, "packages", plugin, "browser") : join(stageRoot, "packages", plugin), { recursive: true, mode: 0o700 });
        files.push({ relative: `packages/${plugin}/component.wasm`, maximum: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES });
        files.push({ relative: `packages/${plugin}/descriptor.semio`, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
        files.push({ relative: receipts.get(plugin)!.pluginModule.path, maximum: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES });
        if (stagedCarriesActor(plugin)) files.push({ relative: `packages/${plugin}/browser/closed-actor.mjs`, maximum: DOCUMENT_BROWSER_ACTOR_MAX_BYTES });
      }
      for (const blob of readdirSync(join(sourceRoot, "plugin-modules")).sort()) files.push({ relative: trustedBootstrapPluginModuleBlobPath(blob), maximum: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES });''')

rep('''      for (const plugin of stagedPlugins) {
        if (stagedCarriesActor(plugin)) trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin, "browser"));
        trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin));
      }''', '''      for (const plugin of stagedPlugins) {
        if (stagedCarriesActor(plugin)) trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin, "browser"));
        trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin));
      }
      trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));''')

rep('''    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(stageRoot);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog bootstrap cancelled before publication");''', '''    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
    trustedBootstrapFsyncDirectory(stageRoot);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog bootstrap cancelled before publication");''')

rep('''      !catalogSource.includes("bundle.schema_version != 2") ||''', '''      !catalogSource.includes("bundle.schema_version != 3") ||''')

open(P, "w", encoding="utf-8").write(s)
print("bootstrap wired")
