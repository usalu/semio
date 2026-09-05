#!/usr/bin/env bun
/** 📏️ `@semio-tech/norm-plugin` router: `bun ./📜️script.ts test`. */
import Ajv from "ajv";
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

type MutationLeafTaxonomyRow = {
  readonly aggregateVariant: string;
  readonly artifact: string;
  readonly entity: string;
  readonly kind: string;
  readonly module: string;
  readonly physicalLayout: "direct" | "split";
  readonly record: string;
  readonly source: string;
  readonly standard: string;
  readonly subset: string;
  readonly type: string;
  readonly verb: string;
};

type MutationLeafTaxonomy = {
  readonly contractId: "semio.norm.mutation-leaf-taxonomy/v1";
  readonly rows: readonly MutationLeafTaxonomyRow[];
  readonly schemaVersion: 1;
};

function filesBelow(root: string): string[] {
  return readdirSync(root, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? filesBelow(join(root, entry.name)) : [join(root, entry.name)]);
}

function taxonomy(root: string): MutationLeafTaxonomy {
  const artifactsRoot = join(root, "..", "..", "🗿️artifacts");
  const rows: MutationLeafTaxonomyRow[] = [];
  for (const source of filesBelow(artifactsRoot)) {
    const normalizedSource = source.replaceAll("\\", "/");
    if (!normalizedSource.endsWith("/🦀️.rs") || !normalizedSource.includes("/🧬️schema/🧬️mutations/")) continue;
    const rust = readFileSync(source, "utf8");
    if (!rust.includes("impl protocol::MutationKind<")) continue;
    const type = rust.match(/pub struct\s+([A-Za-z0-9_]+)/)?.[1];
    const semantics = rust.match(/const SEMANTICS:\s*protocol::SemanticDescriptor\s*=\s*protocol::SemanticDescriptor\s*\{\s*verb:\s*"([^"]+)",\s*entity:\s*"([^"]+)",\s*kind:\s*"([^"]+)",\s*record:\s*"([^"]+)"\s*\}/s);
    if (!type || !semantics) throw new Error(`mutation leaf metadata is unreadable: ${source}`);
    const segments = normalizedSource.split("/");
    const artifactIndex = segments.indexOf("🗿️artifacts");
    const standardIndex = segments.indexOf("🏅️standards", artifactIndex + 1);
    const subsetIndex = segments.indexOf("🪆️subsets", standardIndex + 1);
    const mutationsIndex = segments.indexOf("🧬️mutations", subsetIndex + 1);
    if ([artifactIndex, standardIndex, subsetIndex, mutationsIndex].some((index) => index < 0)) throw new Error(`mutation leaf taxonomy path is incomplete: ${source}`);
    const physicalLayout = normalizedSource.endsWith("/🦠️mutation/🦀️.rs") ? "split" : "direct";
    const owner = physicalLayout === "split" ? dirname(dirname(source)) : dirname(source);
    const descriptor = JSON.parse(readFileSync(join(owner, "🔣️.json"), "utf8"));
    const expectedOwner = relative(join(root, "..", "..", "..", "..", ".."), owner).replaceAll("\\", "/");
    if (descriptor.owner !== expectedOwner || descriptor.semanticKind !== semantics[3] || descriptor.aggregateVariant !== type) throw new Error(`mutation leaf descriptor identity differs from its source: ${source}`);
    if (typeof descriptor.payloadSchema !== "string" || !existsSync(join(owner, descriptor.payloadSchema))) throw new Error(`mutation payload schema is missing: ${source}: ${descriptor.payloadSchema}`);
    const module = segments[mutationsIndex + 1];
    if (!module) throw new Error(`mutation leaf module is missing: ${source}`);
    const localTypes = new Set([...rust.matchAll(/pub struct\s+([A-Za-z0-9_]+)/g)].map((match) => match[1]));
    for (const binding of rust.matchAll(/^use\s+crate::.*::mutations::.*::([A-Za-z0-9_]+);\s*$/gm)) {
      if (localTypes.has(binding[1])) throw new Error(`mutation leaf imports its own payload ${binding[1]}: ${source}`);
    }
    if (!rust.includes("dsl::MutationLeaf") || !rust.includes("#[mutation_leaf(contract = ::protocol)]")) throw new Error(`mutation leaf contract is missing: ${source}`);
    rows.push({
      aggregateVariant: type,
      artifact: segments[artifactIndex + 1]!,
      entity: semantics[2]!,
      kind: semantics[3]!,
      module,
      physicalLayout,
      record: semantics[4]!,
      source: relative(artifactsRoot, source).replaceAll("\\", "/"),
      standard: segments[standardIndex + 1]!,
      subset: segments[subsetIndex + 1]!,
      type,
      verb: semantics[1]!,
    });
  }
  rows.sort((left, right) => Buffer.from(left.source).compare(Buffer.from(right.source)));
  return { contractId: "semio.norm.mutation-leaf-taxonomy/v1", rows, schemaVersion: 1 };
}

function validateUniqueTaxonomy(value: MutationLeafTaxonomy): boolean {
  const sources = new Set<string>();
  const variants = new Set<string>();
  for (const row of value.rows) {
    const variant = `${row.artifact}\0${row.standard}\0${row.subset}\0${row.aggregateVariant}`;
    if (sources.has(row.source) || variants.has(variant)) return false;
    sources.add(row.source);
    variants.add(variant);
  }
  return true;
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-norm"], this.repoRoot, rest);
  }
}

function configBinaryFixtureOracle(hex: string): number | null | undefined {
  const bytes = Buffer.from(hex, "hex");
  let cursor = 0;
  const integer = (): bigint | undefined => {
    let value = 0n;
    for (let index = 0; index < 10; index++) {
      const byte = bytes[cursor++];
      if (byte === undefined || (index === 9 && byte > 1)) return undefined;
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if (byte < 128) return index > 0 && byte === 0 ? undefined : value;
    }
    return undefined;
  };
  if (bytes[cursor++] !== 1 || integer() !== 0n || integer() !== 0n) return undefined;
  const count = integer();
  if (count === 0n) return cursor === bytes.length ? null : undefined;
  if (count !== 1n || integer() !== 0n || bytes[cursor++] !== 4) return undefined;
  const index = integer();
  return index !== undefined && index <= 4294967295n && cursor === bytes.length ? Number(index) : undefined;
}

class ConfigMutationSourceScript extends BundleScript {
  run(): void {
    const configRoot = join(this.root, "..", "..", "🎚️config");
    const declarations = filesBelow(configRoot).filter((path) => path.endsWith(".rs")).flatMap((path) => [...readFileSync(path, "utf8").matchAll(/pub struct NormConfig\b/g)].map(() => path));
    if (declarations.length !== 1 || declarations[0] !== join(configRoot, "🧬️schema", "🦀️.rs")) throw new Error("NormConfig must have one schema-owned Rust declaration");
    const schema = JSON.parse(readFileSync(join(configRoot, "🧬️schema", "🧬️mutations", "☑️change-selected-check-index", "🧬️.schema.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(configRoot, "🧪️tests", "🔣️.json"), "utf8"));
    const aggregate = JSON.parse(readFileSync(join(configRoot, "🧬️schema", "🧬️mutations", "🔣️.schema.json"), "utf8"));
    const ajv = new Ajv({ allErrors: true, strict: true });
    const fixtureSchema = JSON.parse(readFileSync(join(configRoot, "🧪️tests", "🧬️.schema.json"), "utf8"));
    const validateFixture = ajv.compile(fixtureSchema);
    if (!validateFixture(fixture)) throw new Error(`config fixture schema failed: ${JSON.stringify(validateFixture.errors)}`);
    const payloadRef = aggregate.properties.ChangeSelectedCheckIndex.$ref;
    if (decodeURI(payloadRef) !== "☑️change-selected-check-index/🧬️.schema.json") throw new Error("config aggregate schema does not reference its owned payload");
    ajv.addSchema(schema, payloadRef);
    const validate = ajv.compile(schema);
    const validateMutation = ajv.compile(aggregate);
    for (const test of fixture.cases) {
      if (!validate(test.payload) || !validateMutation({ ChangeSelectedCheckIndex: test.payload })) throw new Error(`config fixture ${test.id} failed AJV: ${JSON.stringify(validate.errors ?? validateMutation.errors)}`);
      if ((test.payload.index ?? null) !== test.after || (test.before === test.after) !== test.warning) throw new Error(`config fixture ${test.id} has inconsistent results`);
    }
    for (const payload of fixture.invalid) if (validate(payload) || validateMutation({ ChangeSelectedCheckIndex: payload })) throw new Error("AJV accepted a hostile config mutation payload");
    for (const mutation of [{}, { Snapshot: {} }, { SetSelectedCheckIndex: { index: 1 } }, { ChangeSelectedCheckIndex: {}, unknown: true }]) if (validateMutation(mutation)) throw new Error("AJV accepted an undeclared config mutation");
    for (const rows of [fixture.text, fixture.binary]) if (new Set(rows.map((row: { id: string }) => row.id)).size !== rows.length) throw new Error("config wire fixture has duplicate vector identities");
    for (const row of fixture.text) {
      const match = /^\s*change-selected-check-index(?:\s+index\s*=\s*([0-9]+))?\s*$/.exec(row.wire);
      const after = match ? (match[1] === undefined ? null : Number(match[1])) : undefined;
      const accepted = after !== undefined && validate({ index: after });
      if (accepted !== row.accepted || (accepted && after !== row.after)) throw new Error(`independent text oracle disagrees with ${row.id}`);
    }
    for (const row of fixture.binary) {
      const after = configBinaryFixtureOracle(row.hex);
      const accepted = after !== undefined && validate({ index: after });
      if (accepted !== row.accepted || (accepted && after !== row.after)) throw new Error(`independent byte oracle disagrees with ${row.id}`);
    }
    const operationRoot = join(configRoot, "🧬️schema", "🧬️mutations");
    if (!readFileSync(join(operationRoot, "📝️text", "🦀️.rs"), "utf8").includes("dsl::parse_exact(")) throw new Error("config text must use the shared exact record boundary");
    if (!readFileSync(join(operationRoot, "💾️binary", "🦀️.rs"), "utf8").includes("dsl::variants_binary::decode_op(bytes)")) throw new Error("config binary must use the shared closed canonical operation boundary");
    console.log(`Norm config schema oracle passed: ${fixture.cases.length} cases, ${fixture.invalid.length} hostile payloads, 4 undeclared wire forms, ${fixture.text.length} text vectors, ${fixture.binary.length} binary vectors`);
  }
}

class ConfigMutationTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-norm"], this.repoRoot, ["--test", "config_mutation", ...rest]);
  }
}

type SurfaceRow = { variant: string; role: "editor" | "viewer"; appId: string; bodyKeys: string[] };
type SurfaceFixture = { contractId: "semio.norm.surface-render/v1"; rows: SurfaceRow[] };

class SurfaceRenderSourceScript extends BundleScript {
  run(): void {
    const testRoot = join(this.root, "..", "..", "🖥️app-surface", "🧪️tests");
    const schema = JSON.parse(readFileSync(join(testRoot, "🧬️.schema.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(testRoot, "🔣️.json"), "utf8")) as SurfaceFixture;
    const pluginRoot = readFileSync(join(this.root, "..", "..", "🦀️.rs"), "utf8");
    if (!pluginRoot.includes('.package_id("semio:norm")')) throw new Error("norm plugin does not declare its exact component package identity");
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    const manifest = Bun.TOML.parse(readFileSync(join(this.root, "Cargo.toml"), "utf8")) as { package: { metadata: { semio: { playground: { variant: string }[] } } } };
    const variants = new Set(manifest.package.metadata.semio.playground.map((entry) => entry.variant));
    const artifactRoot = join(this.root, "..", "..", "🗿️artifacts");
    const incompleteIdentity = new RegExp(`\\bs\\.(${[...variants].join("|")})\\b`);
    for (const path of filesBelow(artifactRoot).filter((path) => /\.(rs|ts|json)$/.test(path))) {
      if (incompleteIdentity.test(readFileSync(path, "utf8"))) throw new Error(`norm artifact uses an identity without its plugin namespace: ${path}`);
    }
    const admitted = (value: SurfaceFixture): boolean => {
      if (!validate(value)) return false;
      const identities = new Set<string>();
      for (const row of value.rows) {
        const expected = (row.role === "editor" ? ["inputs", "results", "document", "catalogue", "inspection"].map((key) => `norm.${row.variant}.play.${key}`) : ["framework.window.table"]).concat("framework.body.history");
        if (!variants.has(row.variant) || row.appId !== `s.norm.${row.variant}@1/*#${row.role}` || identities.has(row.appId) || JSON.stringify([...row.bodyKeys].sort()) !== JSON.stringify(expected.sort())) return false;
        identities.add(row.appId);
      }
      return identities.size === variants.size * 2;
    };
    if (!admitted(fixture)) throw new Error("norm surface inventory disagrees with the neutral schema or owned playground variants");
    const hostile = Array.from({ length: 5 }, () => structuredClone(fixture));
    hostile[0]!.rows[1] = hostile[0]!.rows[0]!;
    hostile[1]!.rows[0]!.bodyKeys.pop();
    hostile[2]!.rows[0]!.bodyKeys = ["framework.window.table"];
    hostile[3]!.rows[0]!.bodyKeys[0] = "unknown.body";
    hostile[4]!.rows[0]!.appId = "s.norm.unknown@1/*#editor";
    for (const candidate of hostile) if (admitted(candidate)) throw new Error("norm surface inventory admitted a hostile vector");
    console.log(`[DEBUG] Norm surface inventory: ${variants.size} variants, ${fixture.rows.length} apps, ${fixture.rows.reduce((count, row) => count + row.bodyKeys.length, 0)} bodies, AJV and ${hostile.length} hostile vectors passed`);
  }
}

class SurfaceRenderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-norm"], this.repoRoot, ["--test", "surface_render", ...rest]);
  }
}

class CheckScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-norm", join(this.root, "..", "..")));
  }
}

class MutationLeafTaxonomyGenerateScript extends BundleScript {
  run(): void {
    const value = taxonomy(this.root);
    writeFileSync(join(this.root, "📇️mutation-leaf-taxonomy-v1.json"), `${JSON.stringify(value, null, 2)}\n`);
    console.log(`norm mutation-leaf taxonomy generated: ${value.rows.length} payloads`);
  }
}

class MutationLeafTaxonomyCheckScript extends BundleScript {
  run(): void {
    const actual = taxonomy(this.root);
    const schema = JSON.parse(readFileSync(join(this.root, "🧬️mutation-leaf-taxonomy-v1.schema.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(this.root, "📇️mutation-leaf-taxonomy-v1.json"), "utf8")) as MutationLeafTaxonomy;
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`norm mutation-leaf taxonomy schema failed: ${JSON.stringify(validate.errors)}`);
    if (!validateUniqueTaxonomy(fixture)) throw new Error("norm mutation-leaf taxonomy contains a duplicate source or aggregate variant");
    if (JSON.stringify(actual) !== JSON.stringify(fixture)) throw new Error("norm mutation-leaf taxonomy is stale; run the registered generate target");
    const missing = structuredClone(fixture) as { rows: Record<string, unknown>[] };
    delete missing.rows[0]!.kind;
    if (validate(missing)) throw new Error("AJV accepted a taxonomy row without its semantic kind");
    const wrongLayout = structuredClone(fixture) as { rows: Record<string, unknown>[] };
    wrongLayout.rows[0]!.physicalLayout = "ambient";
    if (validate(wrongLayout)) throw new Error("AJV accepted an ambient mutation-leaf layout");
    const duplicate = structuredClone(fixture) as MutationLeafTaxonomy;
    (duplicate.rows as MutationLeafTaxonomyRow[]).push(duplicate.rows[0]!);
    if (validateUniqueTaxonomy(duplicate)) throw new Error("the neutral uniqueness oracle accepted a duplicate taxonomy row");
    console.log(`norm mutation-leaf taxonomy is fresh: ${fixture.rows.length} payloads, AJV schema and hostile vectors passed`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("surface-render-source", SurfaceRenderSourceScript)
  .register("surface-render-test", SurfaceRenderTestScript)
  .register("test", TestScript)
  .register("check", CheckScript)
  .register("config-mutation-source", ConfigMutationSourceScript)
  .register("config-mutation-test", ConfigMutationTestScript)
  .register("describe", DescribeScript)
  .register("mutation-leaf-taxonomy-generate", MutationLeafTaxonomyGenerateScript)
  .register("mutation-leaf-taxonomy-check", MutationLeafTaxonomyCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
