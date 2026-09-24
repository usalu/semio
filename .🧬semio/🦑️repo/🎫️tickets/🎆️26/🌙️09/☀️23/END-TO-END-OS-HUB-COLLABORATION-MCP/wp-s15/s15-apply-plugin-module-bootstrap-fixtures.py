"""🧩️ S15: the hub script's generation-stage, publication and rotation-source proofs and the stdio+GIS rotation
carry the trusted plugin module bundle (schema v3). Run after s15-apply-plugin-module-bootstrap.py."""
import json, sys
HUB = "/Users/ueli/Documents/semio/🌎️hub"
P = f"{HUB}/📦️packages/🦀️rust/📜️script.ts"
s = open(P, encoding="utf-8").read()

def rep(old, new):
    global s
    n = s.count(old)
    if n != 1:
        sys.exit(f"anchor count {n}: {old[:160]!r}")
    s = s.replace(old, new)

rep('''/** 🧫️ Constructs only neutral descriptor projections for physical publication-fence laws. */
function trustedBootstrapStageFixturePackages(fixture: any, actor: any, component: Uint8Array): { packages: any[]; descriptors: Map<string, Buffer> } {''', '''/** 🧩️ One never-executed fixture plugin module: its entry, both descriptor forms (the packed one the package's
 * own descriptor) and one vendored import — the TS twin of the hub's `fixture_plugin_module_files`. */
type TrustedBootstrapFixturePluginModuleV1 = Readonly<{ record: TrustedBootstrapPluginModuleRecordV1; manifest: Uint8Array; blobs: ReadonlyMap<string, Uint8Array> }>;

function trustedBootstrapFixturePluginModule(identity: Readonly<{ pluginId: string; packageId: string; version: string }>, componentSha256: string, descriptor: Uint8Array): TrustedBootstrapFixturePluginModuleV1 {
  const utf8 = (text: string) => Buffer.from(text, "utf8");
  const contents: [string, Uint8Array][] = [
    [`${identity.pluginId}/${MODULE_BRIDGE_FILE}`, utf8("export async function createActorApi() {}\\n")],
    [`${identity.pluginId}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE}`, utf8(JSON.stringify({ manifest: { pluginId: identity.pluginId } }))],
    [`${identity.pluginId}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE}`, descriptor],
    [`${PREVIEW2_VENDOR_RELATIVE}/io.js`, utf8("export const streams = {};\\n")],
  ];
  const blobs = new Map<string, Uint8Array>();
  const files = contents
    .sort(([left], [right]) => utf8OrderV1(left, right))
    .map(([path, bytes]) => {
      const sha256 = createHash("sha256").update(bytes).digest("hex");
      blobs.set(sha256, bytes);
      return { path, byteLength: bytes.byteLength, sha256, blake3: blake3Hex(bytes) };
    });
  const descriptorByteSha256 = createHash("sha256").update(descriptor).digest("hex");
  const manifest = encodeTrustedPluginModuleBundleV1({ schema: TRUSTED_PLUGIN_MODULE_SCHEMA, ...identity, sourceComponentSha256: componentSha256, sourceDescriptorByteSha256: descriptorByteSha256, moduleDirectory: identity.pluginId, entry: `${identity.pluginId}/${MODULE_BRIDGE_FILE}`, files });
  decodeTrustedPluginModuleBundleV1(manifest, { ...identity, componentSha256, descriptorByteSha256 });
  return Object.freeze({ record: Object.freeze({ path: trustedBootstrapPluginModuleManifestPath(identity.pluginId), byteLength: manifest.byteLength, sha256: createHash("sha256").update(manifest).digest("hex"), blake3: blake3Hex(manifest) }), manifest, blobs });
}

/** 🧩️ Writes one fixture plugin module beneath a generation root, replacing an earlier one's manifest. */
function trustedBootstrapWriteFixturePluginModule(root: string, module: TrustedBootstrapFixturePluginModuleV1): void {
  mkdirSync(join(root, "plugin-modules"), { recursive: true });
  mkdirSync(dirname(join(root, module.record.path)), { recursive: true });
  writeFileSync(join(root, module.record.path), module.manifest);
  for (const [sha256, bytes] of module.blobs) writeFileSync(join(root, trustedBootstrapPluginModuleBlobPath(sha256)), bytes);
}

/** 🧫️ Constructs only neutral descriptor projections for physical publication-fence laws. */
function trustedBootstrapStageFixturePackages(fixture: any, actor: any, component: Uint8Array): { packages: any[]; descriptors: Map<string, Buffer>; modules: Map<string, TrustedBootstrapFixturePluginModuleV1> } {''')

rep('''      browserActor: pluginId === "gis" ? { ...actor, path: "packages/gis/browser/closed-actor.mjs", sourceComponentSha256: source.component.sha256, sourceDescriptorByteSha256: source.descriptor.sha256 } : { kind: "none" },
    };
  });
  return { packages, descriptors };
}''', '''      browserActor: pluginId === "gis" ? { ...actor, path: "packages/gis/browser/closed-actor.mjs", sourceComponentSha256: source.component.sha256, sourceDescriptorByteSha256: source.descriptor.sha256 } : { kind: "none" },
    };
  });
  const modules = new Map(packages.map((record) => [record.pluginId as string, trustedBootstrapFixturePluginModule(record, record.component.sha256, descriptors.get(record.pluginId)!)]));
  for (const record of packages) record.pluginModule = modules.get(record.pluginId)!.record;
  return { packages, descriptors, modules };
}''')

# generation-stage proof
rep('''  const { packages, descriptors } = trustedBootstrapStageFixturePackages(fixture, actorFixture.closed, component);
  const receipts = new Map(packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)]));''', '''  const { packages, descriptors } = trustedBootstrapStageFixturePackages(fixture, actorFixture.closed, component);
  const receipts = new Map(packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)]));
  const moduleBlob = (stage: string, plugin: string, suffix: string) => {
    const manifest = JSON.parse(readFileSync(join(stage, "packages", plugin, "plugin-module.json"), "utf8"));
    return join(stage, "plugin-modules", manifest.files.find((file: any) => file.path.endsWith(suffix)).sha256);
  };''')
rep('''    "actor-symlink", "missing-protocol", "unsupported-protocol", "receipt-protocol", "missing-dependency", "receipt-dependency", "bundle-dependency",
    "wrong-package", "nonexact-dependency", "duplicate-dependency",
  ]);''', '''    "actor-symlink", "missing-protocol", "unsupported-protocol", "receipt-protocol", "missing-dependency", "receipt-dependency", "bundle-dependency",
    "wrong-package", "nonexact-dependency", "duplicate-dependency", "plugin-module-file", "plugin-module-manifest", "plugin-module-missing-file",
    "plugin-module-extra-file", "post-read-plugin-module-file",
  ]);''')
rep('''    const caseReceipts = structuredClone(receipts);
    const casePackages = structuredClone(packages);
    const stage = join(evidence, test.change);''', '''    const caseReceipts = structuredClone(receipts);
    const casePackages = structuredClone(packages);
    const caseDescriptors = new Map(descriptors);
    const stage = join(evidence, test.change);''')
rep('''      const bytes = Buffer.from(encodePackValue(changed));
      const digest = await identity(bytes);
      writeFileSync(join(stage, "packages/gis/descriptor.semio"), bytes);''', '''      const bytes = Buffer.from(encodePackValue(changed));
      const digest = await identity(bytes);
      caseDescriptors.set("gis", bytes);
      writeFileSync(join(stage, "packages/gis/descriptor.semio"), bytes);''')
rep('''      writeFileSync(join(stage, "packages/stdio/descriptor.semio"), changed);
      Object.assign(caseReceipts.get("stdio")!, { descriptor: await identity(changed) });
      casePackages[1].descriptor = await identity(changed);''', '''      writeFileSync(join(stage, "packages/stdio/descriptor.semio"), changed);
      caseDescriptors.set("stdio", changed);
      Object.assign(caseReceipts.get("stdio")!, { descriptor: await identity(changed) });
      casePackages[1].descriptor = await identity(changed);''')
rep('''    const bundle = Buffer.from(JSON.stringify({ neutral: true, packages: casePackages }) + "\\n");
    writeFileSync(join(stage, "trusted-catalog.json"), test.change === "bundle" ? '{"neutral":false}' : bundle);''', '''    for (const [index, plugin] of ["gis", "stdio"].entries()) {
      if (!existsSync(join(stage, "packages", plugin))) continue;
      const module = trustedBootstrapFixturePluginModule(casePackages[index], casePackages[index].component.sha256, caseDescriptors.get(plugin)!);
      trustedBootstrapWriteFixturePluginModule(stage, module);
      casePackages[index].pluginModule = module.record;
      caseReceipts.set(plugin, { ...caseReceipts.get(plugin)!, pluginModule: module.record });
    }
    if (test.change === "plugin-module-file") writeFileSync(moduleBlob(stage, "gis", "bridge.js"), "export {};\\n");
    if (test.change === "plugin-module-manifest") writeFileSync(join(stage, "packages/stdio/plugin-module.json"), "{}\\n");
    if (test.change === "plugin-module-missing-file") rmSync(moduleBlob(stage, "stdio", "io.js"));
    if (test.change === "plugin-module-extra-file") writeFileSync(join(stage, "plugin-modules", "ab".repeat(32)), "unlisted");
    const pluginModuleFile = moduleBlob(stage, "gis", ".json");
    const bundle = Buffer.from(JSON.stringify({ neutral: true, packages: casePackages }) + "\\n");
    writeFileSync(join(stage, "trusted-catalog.json"), test.change === "bundle" ? '{"neutral":false}' : bundle);''')
rep('''      if (test.change === "post-read-actor" && phase === "identity" && !replaced) {
        writeFileSync(actorPath, "xbc");
        replaced = true;
      }''', '''      if (test.change === "post-read-actor" && phase === "identity" && !replaced) {
        writeFileSync(actorPath, "xbc");
        replaced = true;
      }
      if (test.change === "post-read-plugin-module-file" && phase === "identity" && !replaced) {
        writeFileSync(pluginModuleFile, "{}");
        replaced = true;
      }''')

# publication proof
rep('''  const { packages, descriptors } = trustedBootstrapStageFixturePackages(stageFixture, actorFixture.closed, component);
  for (const record of packages) {
    assert.equal(record.component.sha256, await digest(component));
    assert.equal(record.descriptor.sha256, await digest(descriptors.get(record.pluginId)!));
  }
  const bundle = Buffer.from(JSON.stringify({ schemaVersion: 2, profiles: [{ id: fixture.profileId, generationId: fixture.generationId }], packages }) + "\\n");''', '''  const { packages, descriptors, modules } = trustedBootstrapStageFixturePackages(stageFixture, actorFixture.closed, component);
  for (const record of packages) {
    assert.equal(record.component.sha256, await digest(component));
    assert.equal(record.descriptor.sha256, await digest(descriptors.get(record.pluginId)!));
    assert.equal(record.pluginModule.sha256, await digest(modules.get(record.pluginId)!.manifest));
  }
  const bundle = Buffer.from(JSON.stringify({ schemaVersion: 3, profiles: [{ id: fixture.profileId, generationId: fixture.generationId }], packages }) + "\\n");''')
rep('''    for (const plugin of ["gis", "stdio"]) {
      writeFileSync(join(generationRoot, "packages", plugin, "component.wasm"), component);
      writeFileSync(join(generationRoot, "packages", plugin, "descriptor.semio"), descriptors.get(plugin)!);
    }
    const actorPath = join(generationRoot, "packages/gis/browser/closed-actor.mjs"), bundlePath = join(generationRoot, "trusted-catalog.json");''', '''    for (const plugin of ["gis", "stdio"]) {
      writeFileSync(join(generationRoot, "packages", plugin, "component.wasm"), component);
      writeFileSync(join(generationRoot, "packages", plugin, "descriptor.semio"), descriptors.get(plugin)!);
      trustedBootstrapWriteFixturePluginModule(generationRoot, modules.get(plugin)!);
    }
    const actorPath = join(generationRoot, "packages/gis/browser/closed-actor.mjs"), bundlePath = join(generationRoot, "trusted-catalog.json");''')
rep('''    const changes: Readonly<Record<string, string>> = { component: join(generationRoot, "packages/gis/component.wasm"), descriptor: join(generationRoot, "packages/stdio/descriptor.semio"), actor: actorPath, bundle: bundlePath };''', '''    const changes: Readonly<Record<string, string>> = { component: join(generationRoot, "packages/gis/component.wasm"), descriptor: join(generationRoot, "packages/stdio/descriptor.semio"), actor: actorPath, "plugin-module": join(generationRoot, "packages/stdio/plugin-module.json"), bundle: bundlePath };''')
rep('''  assert.deepEqual(fixture.cases, [
    { change: "none", accepted: true }, { change: "component", accepted: false }, { change: "descriptor", accepted: false },
    { change: "actor", accepted: false }, { change: "bundle", accepted: false },
  ]);''', '''  assert.deepEqual(fixture.cases, [
    { change: "none", accepted: true }, { change: "component", accepted: false }, { change: "descriptor", accepted: false },
    { change: "actor", accepted: false }, { change: "plugin-module", accepted: false }, { change: "bundle", accepted: false },
  ]);''')

# rotation-source proof
rep('''    const bundle = { schemaVersion: 2, profiles: [{ id: fixture.profile.id, generationId: fixture.profile.generationId }], packages: [{}, {}] };''', '''    const bundle = { schemaVersion: 3, profiles: [{ id: fixture.profile.id, generationId: fixture.profile.generationId }], packages: [{}, {}] };''')

# stdio+GIS rotation: carry each package's plugin module, re-deriving Stdio's from its rewritten descriptor.
rep('''        trustedBootstrapWriteNew(join(destinationRoot, "component.wasm"), component, checkGeneration);
        trustedBootstrapWriteNew(join(destinationRoot, "descriptor.semio"), descriptor, checkGeneration);
        trustedBootstrapFsyncDirectory(destinationRoot);
      } finally {''', '''        trustedBootstrapWriteNew(join(destinationRoot, "component.wasm"), component, checkGeneration);
        trustedBootstrapWriteNew(join(destinationRoot, "descriptor.semio"), descriptor, checkGeneration);
        record.pluginModule = trustedBootstrapCarryPluginModule(join(generationsRoot, current.generationId), stageRoot, sourceReceipt, record, descriptor, checkGeneration);
        trustedBootstrapFsyncDirectory(destinationRoot);
      } finally {''')
rep('''      const component = trustedBootstrapReadRegular(join(sourceRoot, "component.wasm"), 64 * 1024 * 1024, `${plugin} rotation component`, checkGeneration);
      let descriptor = trustedBootstrapReadRegular(join(sourceRoot, "descriptor.semio"), 4 * 1024 * 1024, `${plugin} rotation descriptor`, checkGeneration);''', '''      const sourceReceipt = trustedBootstrapGenerationReceipt(record);
      const component = trustedBootstrapReadRegular(join(sourceRoot, "component.wasm"), 64 * 1024 * 1024, `${plugin} rotation component`, checkGeneration);
      let descriptor = trustedBootstrapReadRegular(join(sourceRoot, "descriptor.semio"), 4 * 1024 * 1024, `${plugin} rotation descriptor`, checkGeneration);''')
rep('''      browserActor: record.browserActor,
      codecCount: record.nativeCodecs.length,
      targetCount: record.openTargets.length,
    }));
    if (!Array.isArray(profile.openTargets) || profile.openTargets.length !== 1) throw new Error("trusted rotation source profile does not declare exactly one open target");
    const profileSummary = { id: profile.id, selectedClosure: profile.selectedClosure, packages: packageSummary, openTarget: { pluginId: profile.openTargets[0].package.pluginId, ...profile.openTargets[0].target } };''', '''      browserActor: record.browserActor,
      pluginModule: record.pluginModule,
      codecCount: record.nativeCodecs.length,
      targetCount: record.openTargets.length,
    }));
    if (!Array.isArray(profile.openTargets) || profile.openTargets.length !== 1) throw new Error("trusted rotation source profile does not declare exactly one open target");
    const profileSummary = { id: profile.id, selectedClosure: profile.selectedClosure, packages: packageSummary, openTargets: profile.openTargets.map((selection: any) => ({ pluginId: selection.package.pluginId, ...selection.target })) };''')
rep('''    trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes, checkGeneration);
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(stageRoot);
    const generationRoot = join(generationsRoot, generationId);
    if (existsSync(generationRoot)) throw new Error("trusted rotation generation already exists");''', '''    trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes, checkGeneration);
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
    trustedBootstrapFsyncDirectory(stageRoot);
    const generationRoot = join(generationsRoot, generationId);
    if (existsSync(generationRoot)) throw new Error("trusted rotation generation already exists");''')
rep('''/** 🛑️ An abort signal for one child process that follows the bootstrap's own cancellation and deadline. */''', '''/**
 * 🧳️ Carries one package's plugin module from a verified source generation into a new stage: every file is
 * reread and checked against its manifest, and the packed and JSON descriptors are replaced by the package's
 * (possibly rewritten) `descriptor`, so the carried module is derived from exactly the package it ships with.
 */
function trustedBootstrapCarryPluginModule(sourceRoot: string, stageRoot: string, sourceReceipt: TrustedBootstrapGenerationReceiptV1, record: any, descriptor: Uint8Array, check: () => void): TrustedBootstrapPluginModuleRecordV1 {
  const source = trustedBootstrapReadPluginModule(sourceRoot, sourceReceipt, check);
  const replacements = new Map<string, Uint8Array>([
    [`${source.moduleDirectory}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE}`, descriptor],
    [`${source.moduleDirectory}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE}`, Buffer.from(`${JSON.stringify(packValueToExactJson(decodePackValue(descriptor)), null, 2)}\\n`, "utf8")],
  ]);
  const blobs = join(stageRoot, "plugin-modules");
  mkdirSync(blobs, { recursive: true, mode: 0o700 });
  const files = source.files.map((file) => {
    const bytes = replacements.get(file.path) ?? readStableBuildFile(join(sourceRoot, trustedBootstrapPluginModuleBlobPath(file.sha256)), file.byteLength, { remaining: file.byteLength }, check);
    const carried = { path: file.path, byteLength: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex"), blake3: blake3Hex(bytes) };
    if (!replacements.has(file.path) && (carried.sha256 !== file.sha256 || carried.blake3 !== file.blake3)) throw new Error("trusted rotation plugin module file differs from its manifest");
    if (!existsSync(join(blobs, carried.sha256))) trustedBootstrapWriteNew(join(blobs, carried.sha256), bytes, check);
    return carried;
  });
  const descriptorByteSha256 = createHash("sha256").update(descriptor).digest("hex");
  const manifest = encodeTrustedPluginModuleBundleV1({ ...source, sourceComponentSha256: record.component.sha256, sourceDescriptorByteSha256: descriptorByteSha256, files });
  decodeTrustedPluginModuleBundleV1(manifest, { pluginId: record.pluginId, packageId: record.packageId, version: record.version, componentSha256: record.component.sha256, descriptorByteSha256 });
  trustedBootstrapWriteNew(join(stageRoot, trustedBootstrapPluginModuleManifestPath(record.pluginId)), manifest, check);
  return Object.freeze({ path: trustedBootstrapPluginModuleManifestPath(record.pluginId), byteLength: manifest.byteLength, sha256: createHash("sha256").update(manifest).digest("hex"), blake3: blake3Hex(manifest) });
}

/** 🛑️ An abort signal for one child process that follows the bootstrap's own cancellation and deadline. */''')
open(P, "w", encoding="utf-8").write(s)

# Fixture case lists, edited in their own compact one-row-per-case style.
F = f"{HUB}/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures"
def text_edit(path, old, new):
    t = open(path, encoding="utf-8").read()
    if t.count(old) != 1:
        sys.exit(f"{path}: fixture anchor {old!r}")
    open(path, "w", encoding="utf-8").write(t.replace(old, new))
    json.load(open(path, encoding="utf-8"))
text_edit(f"{F}/🧱️generation-stage/🔣️.json", '    { "change": "duplicate-dependency", "accepted": false }\n', '    { "change": "duplicate-dependency", "accepted": false },\n' + ",\n".join(f'    {{ "change": "{change}", "accepted": false }}' for change in ["plugin-module-file", "plugin-module-manifest", "plugin-module-missing-file", "plugin-module-extra-file", "post-read-plugin-module-file"]) + "\n")
text_edit(f"{F}/📤️publication/🔣️.json", '    { "change": "actor", "accepted": false },\n', '    { "change": "actor", "accepted": false },\n    { "change": "plugin-module", "accepted": false },\n')
print("bootstrap fixtures + rotation wired")
