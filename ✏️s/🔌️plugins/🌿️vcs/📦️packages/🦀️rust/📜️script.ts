#!/usr/bin/env bun
/** 🌿️ `@semio-tech/vcs-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-vcs"], this.repoRoot, rest);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-vcs", join(this.root, "..", "..")));
  }
}

/** 🧬️ Compiles one PascalCase `$defs` export of the `s.vcs` scope-owned draft-07 schema module.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json */
async function compileVcsScopeExport(repoRoot: string, exportId: string) {
  const module = JSON.parse(readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌿️vcs/🧬️schema/🔣️.json"), "utf8"));
  const { default: Ajv } = await import("ajv");
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  ajv.addSchema(module);
  return ajv.compile({ $ref: `${module.$id}#/$defs/${exportId}` });
}

/** 🪤 Independently validates the literal VCS codec identity and its protocol-byte receipt. */
export async function proveVcsNativeCodecReceipts(repoRoot: string): Promise<void> {
  const owner = join(repoRoot, "✏️s/🔌️plugins/🌿️vcs");
  const root = join(owner, "📇️native-codecs");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await compileVcsScopeExport(repoRoot, "VcsNativeCodecs");
  if (!validate(fixture)) throw new Error(`invalid VCS receipt corpus: ${JSON.stringify(validate.errors)}`);
  const documentIdRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🌱️artifact-document-id-v1");
  const documentIds = JSON.parse(readFileSync(join(documentIdRoot, "🔣️.json"), "utf8"));
  const registryModule = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json"), "utf8"));
  const { default: RegistryAjv } = await import("ajv");
  const registryAjv = new RegistryAjv({ strict: true, allErrors: true });
  registryAjv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  registryAjv.addSchema(registryModule);
  const validateDocumentIds = registryAjv.compile({ $ref: `${registryModule.$id}#/$defs/ArtifactDocumentIdV1` });
  if (!validateDocumentIds(documentIds)) throw new Error(`invalid artifact document-id corpus: ${JSON.stringify(validateDocumentIds.errors)}`);
  for (const row of documentIds.cases) if (/^artifact-(?!0{32}$)[0-9a-f]{32}$/u.test(row.documentId) !== row.accepted) throw new Error(`artifact document-id oracle mismatch ${row.id}`);
  const manifest = Bun.TOML.parse(readFileSync(join(owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as any;
  if (manifest.package.metadata.component.package !== fixture.packageId) throw new Error("VCS Cargo package identity differs from receipt owner");
  const workspace = Bun.TOML.parse(readFileSync(join(repoRoot, "Cargo.toml"), "utf8")) as any;
  const version = manifest.package.version?.workspace === true ? workspace.workspace.package.version : manifest.package.version;
  if (version !== fixture.packageVersion) throw new Error("VCS compiled package version differs from receipt owner");
  for (const row of fixture.receipts) {
    const bytes = readFileSync(join(owner, row.protocolPath));
    const nodeHash = createHash("sha256").update(bytes).digest("hex");
    const webHash = Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
    if (bytes.length !== row.protocolBytes || nodeHash !== row.protocolSha256 || webHash !== nodeHash || /^0{64}$/u.test(nodeHash)) throw new Error(`VCS protocol receipt mismatch ${row.factoryId}`);
    if (row.kind !== `s.${fixture.pluginId}.${row.extension}` || row.capability !== `${row.kind}.codec.document` || row.factoryId !== `${fixture.pluginId}.${row.extension}.v1` || row.schema !== `${fixture.pluginId}.${row.extension}`) throw new Error("VCS receipt canonical owner mismatch");
  }
  const expected = new Map(fixture.receipts.map((row: any) => [row.factoryId, JSON.stringify(row)]));
  const admitted = (candidate: any): boolean => candidate.pluginId === "vcs" && candidate.packageId === "semio:vcs" && candidate.packageVersion === version && candidate.receipts.length === 1
    && new Set(candidate.receipts.map((row: any) => row.factoryId)).size === 1
    && candidate.receipts.every((row: any) => expected.get(row.factoryId) === JSON.stringify(row));
  if (!admitted(fixture)) throw new Error("literal VCS closure was denied");
  for (const hostile of fixture.hostile) {
    const candidate = structuredClone(fixture);
    switch (hostile) {
      case "missing": candidate.receipts.pop(); break;
      case "duplicate": candidate.receipts.push(structuredClone(candidate.receipts[0])); break;
      case "foreign-package": candidate.packageId = "semio:stdio"; break;
      case "wrong-version": candidate.packageVersion = "0.2.0"; break;
      case "bare-kind": candidate.receipts[0].kind = "vcs.vcs"; break;
      case "legacy-kind": candidate.receipts[0].kind = "vcs.document"; break;
      case "legacy-schema": candidate.receipts[0].schema = "vcs.document"; break;
      case "wrong-extension": candidate.receipts[0].extension = "vcsdocument"; break;
      case "zero-hash": candidate.receipts[0].protocolSha256 = "00".repeat(32); break;
      default: throw new Error(`unknown VCS hostile closure ${hostile}`);
    }
    if (admitted(candidate)) throw new Error(`VCS hostile closure admitted ${hostile}`);
  }
  const source = readFileSync(join(root, "🦀️.rs"), "utf8");
  if (!source.includes(`include_bytes!("../${fixture.receipts[0].protocolPath}")`) || source.includes("vcs.document")) throw new Error("VCS receipt module does not pin its exact protocol bytes without the retired kind");
  const appBase = join(owner, "🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
  const config = readFileSync(join(appBase, "🎚️config/🦀️.rs"), "utf8");
  const presence = readFileSync(join(appBase, "👥️presence/🦀️.rs"), "utf8");
  const history = readFileSync(join(appBase, "🎭️modes/✏️edit/🪟️windows/📜️history/🦀️.rs"), "utf8");
  if (config.match(/schema_version: 1,/gu)?.length !== 1 || !config.includes('semantic_kind: "snapshot"') || !config.includes("MutationOutcomeClass::Warning") || !config.includes("fn descriptor(&self)")) throw new Error("VCS config mutation metadata is incomplete");
  if (presence.match(/schema_version: 1,/gu)?.length !== 1 || !presence.includes('semantic_kind: "noop"') || !presence.includes("MutationInvertibility::SelfInvertible") || !presence.includes("fn descriptor(&self)")) throw new Error("VCS presence mutation metadata is incomplete");
  if (!readFileSync(join(owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8").includes("semio-framework-ui-scene =") || !history.includes("use semio_framework_ui_scene::GraphTimelineScene;") || history.includes("semio_framework_ui_scene::GraphTimelineScene {")) throw new Error("VCS timeline scene does not use its declared first-party scene dependency");
  console.log(`vcs-native-codec-oracle: receipts=${fixture.receipts.length} hostile=${fixture.hostile.length} ajv+node+webcrypto=1 dependency-coherence=3; no catalog activation or VCS execution claim`);
}

/** 🪤 Exact native VCS receipt proof; it does not activate a hub catalog or link a provider. */
class NativeCodecCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveVcsNativeCodecReceipts(this.repoRoot);
    if (process.argv.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-s-plugin-vcs", target: { kind: "test", name: "native_codecs" }, laws: ["vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution", "vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind"] }],
      progress(event) { console.log(`vcs-native-codecs ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`vcs-native-codec-receipt: ${JSON.stringify(receipt)}`);
    console.log("vcs-native-codec-check: exact=1 literal-codecs=1; no hub catalog activation, provider link or client mount");
  }
}

class NativeOpenableIdentityCheckScript extends BundleScript {
  async run(): Promise<void> {
    const owner = join(this.root, "..", "..");
    const fixtureRoot = join(owner, "🧫️fixtures/🪪️native-openable-identity/🧬️v1");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const module = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ allErrors: true, strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    ajv.addSchema(module);
    const validate = ajv.compile({ $ref: `${module.$id}#/$defs/VcsNativeOpenableIdentity` });
    if (!validate(fixture)) throw new Error(`VCS identity fixture is invalid: ${ajv.errorsText(validate.errors)}`);
    const identities = new Set<string>();
    for (const hostile of fixture.hostileCases) {
      if (identities.has(hostile.name)) throw new Error("duplicate hostile identity case");
      identities.add(hostile.name);
      const candidate = structuredClone(fixture);
      if (hostile.value === null) delete candidate.authority[hostile.field];
      else candidate.authority[hostile.field] = hostile.value;
      if (validate(candidate)) throw new Error(`VCS hostile identity admitted: ${hostile.name}`);
    }
    const protocol = readFileSync(join(owner, "🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/📡️.protocol.semio"));
    const webDigest = Buffer.from(await crypto.subtle.digest("SHA-256", protocol)).toString("hex");
    if (webDigest !== fixture.snapshotProtocolSha256 || webDigest !== createHash("sha256").update(protocol).digest("hex")) throw new Error("VCS protocol identity differs from its pinned neutral vector");
    console.log(`vcs-native-openable-identity-oracle: positive=1 hostile-denied=${identities.size} protocol-webcrypto=1`);
    const root = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const artifact = readFileSync(join(owner, "🗿️artifacts/🌿️vcs/🦀️.rs"), "utf8");
    const manifest = readFileSync(join(this.root, "Cargo.toml"), "utf8");
    if (!root.includes('.package_id("semio:vcs")') || !manifest.includes('package = "semio:vcs"') || artifact.includes('"vcs.document"')) throw new Error("VCS guest package/artifact authority is not canonical");
    if (process.argv.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-s-plugin-vcs", target: { kind: "test", name: "native_openable_identity" }, laws: ["vcs_guest_descriptor_has_one_canonical_native_openable_identity"] }],
      progress(event) { console.log(`vcs-native-openable-identity ${event.stage}: artifacts=${event.artifactDir}`); },
    });
    console.log(`vcs-native-openable-identity-laws: ${JSON.stringify(receipts)}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript).register("native-openable-identity-check", NativeOpenableIdentityCheckScript).register("native-codec-check", NativeCodecCheckScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
