import { jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import { buildSync } from "esbuild";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { discoverPluginBuildTargets, generatePluginRegistry, generatePluginRegistryReport, parseComponentSourceOwnerV1, parseCompiledComponentOwnerV1, parseRegistryChannelDiagnosticsV1 } from "../🟦️.ts";
import { REGISTRY_HOST_APP_CHANNEL_VERSION } from "../../🧬️schema/🟦️.ts";
import contract from "../🧬️schema/🔣️.json";
import corpus from "./🧫️fixtures/🔣️.json";
import { identityInstallationSchema as identity } from "../../../../../../../🔨️modules/🪪️identity/🧪️tests/🧫️fixtures/🟦️.ts";
import { parseComponentSourceRowV1, parseCompiledComponentRowV1, parseDeployedRegistryEntryV1, parsePluginBuildTargetV1 } from "../🧬️schema/🟦️.ts";

const parsers = { source: parseComponentSourceRowV1, target: parsePluginBuildTargetV1, compiled: parseCompiledComponentRowV1, deployed: parseDeployedRegistryEntryV1 };
const names = { source: "ComponentSourceOwnerV1", target: "PluginBuildTargetV1", compiled: "CompiledComponentOwnerV1", deployed: "DeployedRegistryEntryV1" };
test("source, build target, compiled and deployed contracts agree with independent AJV", () => {
  const ajv = new Ajv({ strict: false }).addSchema(identity).addSchema(contract);
  
  for (const row of corpus.cases) {
    const stage = row.stage as keyof typeof parsers;
    const oracle = ajv.compile({ $ref: `${contract.$id}#/$defs/${names[stage]}` });
    expect(oracle(row.value), row.name).toBe(row.valid);
    if (row.valid) {
      const parsed = parsers[stage](row.value);
      expect(parsed, row.name).toMatchObject(row.value);
      expect(row.value, row.name).toMatchObject(parsed);
    }
    else expect(() => parsers[stage](row.value), row.name).toThrow();
  }
});


test("independent Node executes every authored stage vector", () => {
  const program = `import * as api from ${JSON.stringify(resolve(import.meta.dirname, "../🧬️schema/🟦️.ts"))}; const rows=${JSON.stringify(corpus.cases)}; const parsers={source:api.parseComponentSourceRowV1,target:api.parsePluginBuildTargetV1,compiled:api.parseCompiledComponentRowV1,deployed:api.parseDeployedRegistryEntryV1}; console.log(JSON.stringify(rows.map(row=>{try{parsers[row.stage](row.value);return true}catch{return false}})));`;
  const result = buildSync({ stdin: { contents: program, resolveDir: import.meta.dirname }, bundle: true, platform: "node", format: "esm", write: false });
  const node = spawnSync("node", ["--input-type=module"], { input: result.outputFiles[0]!.text, encoding: "utf8" });
  expect(node.status, node.stderr).toBe(0);
  expect(JSON.parse(node.stdout)).toEqual(corpus.cases.map(row => row.valid));
});

test("real source and descriptor admission preserves compiled-only ownership", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Component owner law requires ticket artifacts");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "component-owner-"));
  try {
    const cargo = join(root, "owner/📦️packages/🦀️rust/Cargo.toml");
    mkdirSync(dirname(cargo), { recursive: true });
    writeFileSync(cargo, corpus.source.cargo);
    const toml = parseToml(corpus.source.cargo) as { package: { metadata: { component: { package: string }; semio: { "component-kind": string } } } };
    const source = parseComponentSourceOwnerV1(cargo, root);
    expect(source.packageId).toBe(toml.package.metadata.component.package);
    expect(toml.package.metadata.semio["component-kind"]).toBe(source.role);
    expect(source.directoryName).toBeUndefined();
    for (const field of ["capabilities", "contributes", "activationEvents", "extensionPoints", "hashes"]) expect(Object.hasOwn(source, field)).toBe(false);
    expect(() => parseCompiledComponentOwnerV1(cargo, root)).toThrow();
    const descriptor = join(root, "owner/🔣️.json");
    writeFileSync(descriptor, JSON.stringify(corpus.source.descriptor));
    const compiled = parseCompiledComponentOwnerV1(cargo, root);
    expect(compiled).toEqual({ ...source, ...corpus.source.compiled });
    expect(() => parseDeployedRegistryEntryV1(compiled)).toThrow();
    writeFileSync(cargo, corpus.source.cargo + `deployment-directory=${JSON.stringify(corpus.source.directoryName)}\n`);
    expect(corpus.source.directoryName).toBe(parseDeployedRegistryEntryV1(parseCompiledComponentOwnerV1(cargo, root)).directoryName);
    writeFileSync(descriptor, JSON.stringify({ ...corpus.source.descriptor, packageId: "semio:other" }));
    expect(() => parseCompiledComponentOwnerV1(cargo, root)).toThrow(/identity/);
    writeFileSync(descriptor, JSON.stringify({ ...corpus.source.descriptor, executionProtocol: { appChannelVersion: 19 } }));
    expect(() => parseCompiledComponentOwnerV1(cargo, root)).toThrow(/registry descriptor/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

const staleFixtureOwners = (root: string) => {
  const owner = (id: string, channel: number, dependsOn: readonly string[] = []) => {
    const manifestPath = `${id}/📦️packages/🦀️rust/Cargo.toml`;
    mkdirSync(join(root, dirname(manifestPath)), { recursive: true });
    const cargo = corpus.source.cargo.replace('name = "portable-component"', `name = "${id}-component"`).replace('package = "semio:portable"', `package = "semio:${id}"`);
    writeFileSync(join(root, manifestPath), `${cargo}${dependsOn.length ? `depends-on = ${JSON.stringify(dependsOn)}\n` : ""}deployment-directory=${JSON.stringify(`🧪️${id}`)}\n`);
    writeFileSync(join(root, id, "🔣️.json"), JSON.stringify({ ...corpus.source.descriptor, packageId: `semio:${id}`, manifest: { ...corpus.source.descriptor.manifest, pluginId: id }, executionProtocol: { appChannelVersion: channel } }));
    return { lang: "🦀️rust", manifestPath };
  };
  return [owner("fresh", REGISTRY_HOST_APP_CHANNEL_VERSION), owner("stale", REGISTRY_HOST_APP_CHANNEL_VERSION - 1), owner("dependent", REGISTRY_HOST_APP_CHANNEL_VERSION, ["stale"])] as never;
};

test("dev generation withholds stale-channel plugins and their dependents while the refusing gate names them", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Stale-channel registry law requires ticket artifacts");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "stale-channel-"));
  try {
    const packages = staleFixtureOwners(root);
    const report = generatePluginRegistryReport(root, { packages, staleChannel: "exclude" });
    expect(report.entries.map((entry) => entry.pluginId)).toEqual(["fresh"]);
    expect(report.diagnostics).toEqual([
      { code: "stale-channel-dependency", pluginId: "dependent", cratePath: "dependent/📦️packages/🦀️rust", dependency: "stale" },
      { code: "stale-channel", pluginId: "stale", cratePath: "stale/📦️packages/🦀️rust", descriptorChannel: REGISTRY_HOST_APP_CHANNEL_VERSION - 1, hostChannel: REGISTRY_HOST_APP_CHANNEL_VERSION },
    ]);
    expect(parseRegistryChannelDiagnosticsV1(`${JSON.stringify(report.diagnostics, null, 2)}\n`)).toEqual(report.diagnostics);
    expect(() => generatePluginRegistry(root, { packages })).toThrow(/stale-channel descriptors refused .*stale \(stale\/📦️packages\/🦀️rust\)/);
    expect(() => generatePluginRegistryReport(root, { packages, staleChannel: "refuse" })).toThrow(/stale-channel descriptors refused/);
    expect(() => parseRegistryChannelDiagnosticsV1(JSON.stringify([{ ...report.diagnostics[1], extra: true }]))).toThrow(/undeclared shape/);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("a withheld stale-channel row is still a build target while deployment keeps refusing it", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Stale-channel build-target law requires ticket artifacts");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "stale-build-target-"));
  const { resolvePluginBuildTargets } = await import("../../../🏗️build/📋️plan/🟦️.ts");
  const previousOnly = process.env.SEMIO_PLUGIN_ONLY;
  delete process.env.SEMIO_PLUGIN_ONLY;
  try {
    const packages = staleFixtureOwners(root);
    const deployed = generatePluginRegistryReport(root, { packages, staleChannel: "exclude" }).entries.map((entry) => entry.pluginId);
    const targets = discoverPluginBuildTargets(root, { packages });
    expect(deployed).toEqual(["fresh"]);
    expect(targets.map((target) => target.pluginId)).toEqual(["dependent", "fresh", "stale"]);
    for (const target of targets) expect(String(target.directoryName), target.pluginId).toBe(`🧪️${target.pluginId}`);
    const catalog = { entries: targets, playgrounds: [] };
    expect(resolvePluginBuildTargets(undefined, catalog).map((target) => target.pluginId)).toEqual(["dependent", "fresh", "stale"]);
    expect(resolvePluginBuildTargets("dependent", catalog).map((target) => target.pluginId)).toEqual(["dependent", "stale"]);
    expect(resolvePluginBuildTargets("stale", catalog).map((target) => target.pluginId)).toEqual(["stale"]);
    expect(() => resolvePluginBuildTargets("absent", catalog)).toThrow(/no program build targets/);
    process.env.SEMIO_PLUGIN_ONLY = "stale";
    expect(resolvePluginBuildTargets(undefined, catalog).map((target) => target.pluginId)).toEqual(["stale"]);
    process.env.SEMIO_PLUGIN_ONLY = "absent";
    expect(() => resolvePluginBuildTargets(undefined, catalog)).toThrow(/matched no plugin crates/);
    expect(() => generatePluginRegistry(root, { packages })).toThrow(/stale-channel descriptors refused/);
  } finally {
    if (previousOnly === undefined) delete process.env.SEMIO_PLUGIN_ONLY; else process.env.SEMIO_PLUGIN_ONLY = previousOnly;
    rmSync(root, { recursive: true, force: true });
  }
});

test("JSON-schema equality preserves array order and ignores object member order", () => {
  const ajv = new Ajv({ strict: false });
  for (const row of corpus.comparisons) {
    expect(ajv.compile({ const: row.left })(row.right), row.name).toBe(row.equal);
    expect(jsonSchemaSubsetValueEquals(row.left, row.right), row.name).toBe(row.equal);
    const schema = { type: "array", uniqueItems: true };
    expect(validateJsonSchemaSubset(schema, [row.left, row.right]).length === 0).toBe(ajv.compile(schema)([row.left, row.right]));
  }
});


test("source playground catalogs retain example membership while compiled channels are withheld", async () => {
  const { generateWithheldPlaygroundRegistry } = await import("../../🎮️playground/🔎️discovery/🟦️.ts");
  const { registryCatalogInputView } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
  const root = mkdtempSync(join(output, "source-playground-")), cratePath = "owner/📦️packages/🦀️rust";
  const cargo = '[package]\nname="owner"\n[[package.metadata.semio.playground]]\nvariant="owner"\napp="owner.drawing@1/*#editor"\nports={react=6000,wgpu=6100}\nengines=["engine/📦️packages/🦀️rust"]\n[package.metadata.semio.sources]\nartifacts=["../../🗿️artifacts"]\n';
  mkdirSync(join(root,cratePath),{recursive:true});writeFileSync(join(root,cratePath,"Cargo.toml"),cargo);
  const examples = ["🎬️demo","🎬️demo-session"];
  for(const example of examples){const folder=join(root,"owner/🗿️artifacts/🖍️drawing/📚️examples",example);mkdirSync(folder,{recursive:true});writeFileSync(join(folder,"🦀️.rs"),"");}
  const [row]=generateWithheldPlaygroundRegistry(root,[{code:"stale-channel",pluginId:"owner",cratePath,descriptorChannel:REGISTRY_HOST_APP_CHANNEL_VERSION-1,hostChannel:REGISTRY_HOST_APP_CHANNEL_VERSION}],registryCatalogInputView(root,(await import("../🟦️.ts")).TAXONOMY));
  const parsed = parseToml(cargo) as { package: { metadata: { semio: { playground: { engines: string[]; ports: {react:number;wgpu:number} }[] } } } };
  expect(row!.examples).toEqual(examples);expect(row!.engines).toEqual(parsed.package.metadata.semio.playground[0].engines);expect(row!.ports).toEqual(parsed.package.metadata.semio.playground[0].ports);
});


test("pure session file DATA carries its supplied catalog independently of the global projection", async () => {
  const {playgroundSessionPublicationV1,buildPlaygroundSession}=await import("../../🎮️playground/🧭️session/🟦️.ts");
  const { readGeneratedCatalogProjection } = await import("../../📖️catalog-view/🟦️.ts");
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "session-catalog-"));
  const entry = corpus.cases.find(row => row.stage === "deployed" && row.valid)!.value;
  const projection = { entries: [entry], playgrounds: [{ variant: "portable", pluginId: entry.pluginId, aliases: [], app: "portable.drawing@1/*#editor", ports: { react: 6000, wgpu: 6100 }, examples: [], assets: [], engines: [], cratePath: entry.cratePath }] } as never;
  const output=join(root,"portable");for(const row of playgroundSessionPublicationV1("portable",projection)){const path=join(output,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.text);}const session=buildPlaygroundSession("portable",projection);
  const parsed = readGeneratedCatalogProjection(join(root,"portable"));
  const oracle = new Ajv({ strict: false }).addSchema(identity).addSchema(contract).compile({ $ref: contract.$id + "#/$defs/DeployedRegistryEntryV1" });
  expect(oracle(parsed.entries[0])).toBe(true);
  expect(parsed).toEqual(projection);
  expect(session.plugins.map(row => row.pluginId)).toEqual([entry.pluginId]);
});

test("descriptor emitter product request is selected-mode DATA without a checkout binary probe",async()=>{
 const {descriptorEmitterRequestV1}=await import("../../../🖨️describe/🛠️emitter/📋️contract/🟦️.ts");for(const mode of["prototype","release"]as const){const request=descriptorEmitterRequestV1(join(process.cwd(),".fixture-products"),{version:1,mode,platform:process.platform});expect(request.path).toContain("/browser/"+mode+"/tools/descriptor-emitter/");expect(request.maximumFiles).toBe(2);}
});


test("source deployment names remain available before compiled catalog admission", async () => {
  const {generateComponentSourceRegistry} = await import("../🟦️.ts");
  const {emitTypeScript} = await import("../../📽️projection/🟦️.ts");
  const {registryModuleDirectories} = await import("../../📖️catalog-view/🟦️.ts");
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"source-directories-")), manifestPath="owner/📦️packages/🦀️rust/Cargo.toml";
  mkdirSync(dirname(join(root,manifestPath)),{recursive:true});
  const cargo=corpus.source.cargo+'deployment-directory='+JSON.stringify(corpus.source.directoryName)+'\n';
  writeFileSync(join(root,manifestPath),cargo);
  const rows=generateComponentSourceRegistry(root,{packages:[{lang:"🦀️rust",manifestPath}] as never});
  const inventory=registryModuleDirectories(rows.flatMap(row => row.directoryName === undefined ? [] : [{pluginId:row.pluginId,directoryName:row.directoryName}]));
  const source=emitTypeScript([],inventory);
  const bundle=buildSync({stdin:{contents:source,loader:"ts",resolveDir:resolve(import.meta.dir,"../../🤖️generated/🧩️plugins")},bundle:true,platform:"node",format:"esm",write:false});
  const node=spawnSync("node",["--input-type=module"],{input:bundle.outputFiles[0]!.text+'\nconsole.log(JSON.stringify({modules:COMPONENT_MODULE_DIRECTORIES,plugins:PLUGIN_BUILD_TARGETS}))',encoding:"utf8"});
  expect(node.status,node.stderr).toBe(0);
  const result=JSON.parse(node.stdout), parsed=parseToml(cargo) as any;
  expect(result.modules).toEqual([{pluginId:"portable",directoryName:parsed.package.metadata.semio["deployment-directory"]}]);
  expect(result.plugins).toEqual([]);
});
