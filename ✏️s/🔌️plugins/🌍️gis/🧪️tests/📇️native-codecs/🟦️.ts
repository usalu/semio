import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { compileGisScopeExport } from "../../🧬️schema/🟦️.ts";
import { proveGisControlledProposal } from "../💡️inference-control/🟦️.ts";

const GIS_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json";

/** 🪢 Independently validates literal GIS codec identity and protocol-byte receipts. */
export async function proveGisNativeCodecReceipts(repoRoot: string): Promise<void> {
  const owner = join(repoRoot, "✏️s/🔌️plugins/🌍️gis");
  const root = join(owner, "📇️native-codecs");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisNativeCodecs");
  if (!validate(fixture)) throw new Error(`invalid GIS receipt corpus: ${JSON.stringify(validate.errors)}`);
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
  if (manifest.package.metadata.component.package !== fixture.packageId) throw new Error("GIS Cargo package identity differs from receipt owner");
  const workspace = Bun.TOML.parse(readFileSync(join(repoRoot, "Cargo.toml"), "utf8")) as any;
  const version = manifest.package.version?.workspace === true ? workspace.workspace.package.version : manifest.package.version;
  if (version !== fixture.packageVersion) throw new Error("GIS compiled package version differs from receipt owner");
  for (const row of fixture.receipts) {
    const bytes = readFileSync(join(owner, row.protocolPath));
    const nodeHash = createHash("sha256").update(bytes).digest("hex");
    const webHash = Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
    if (bytes.length !== row.protocolBytes || nodeHash !== row.protocolSha256 || webHash !== nodeHash || /^0{64}$/u.test(nodeHash)) throw new Error(`GIS protocol receipt mismatch ${row.factoryId}`);
    const schema = row.extension === "gismap" ? "gis.map" : "gis.terrain";
    if (row.kind !== `s.gis.${row.extension}` || row.capability !== `${row.kind}.codec.document` || row.factoryId !== `gis.${row.extension}.v1` || row.schema !== schema) throw new Error("GIS receipt canonical owner mismatch");
    const fields = row.extension === "gismap"
      ? [[1, "positions", false], [2, "routes", false], [3, "regions", false], [4, "drawing", false], [5, "image", true], [6, "value", false]]
      : [[1, "exaggeration", false], [2, "importedFeaturesJson", false], [3, "mesh", true]];
    if (row.packRecord.keyword !== row.extension || JSON.stringify(row.packRecord.fields.map((field: any) => [field.id, field.key, field.optional])) !== JSON.stringify(fields)) throw new Error("GIS structural pack record mismatch");
  }
  const expected = new Map(fixture.receipts.map((row: any) => [row.factoryId, JSON.stringify(row)]));
  const admitted = (candidate: any): boolean => candidate.pluginId === "gis" && candidate.packageId === "semio:gis" && candidate.packageVersion === version && candidate.receipts.length === 2
    && new Set(candidate.receipts.map((row: any) => row.factoryId)).size === 2
    && candidate.receipts.every((row: any) => expected.get(row.factoryId) === JSON.stringify(row));
  if (!admitted(fixture)) throw new Error("literal GIS closure was denied");
  for (const hostile of fixture.hostile) {
    const candidate = structuredClone(fixture);
    switch (hostile) {
      case "missing": candidate.receipts.pop(); break;
      case "duplicate": candidate.receipts[1] = structuredClone(candidate.receipts[0]); break;
      case "foreign-package": candidate.packageId = "semio:stdio"; break;
      case "wrong-version": candidate.packageVersion = "0.2.0"; break;
      case "bare-kind": candidate.receipts[0].kind = "gis.gismap"; break;
      case "wrong-schema": candidate.receipts[0].schema = "gis.terrain"; break;
      case "wrong-extension": candidate.receipts[0].extension = "gisterrain"; break;
      case "zero-hash": candidate.receipts[0].protocolSha256 = "00".repeat(32); break;
    }
    if (admitted(candidate)) throw new Error(`GIS hostile closure admitted ${hostile}`);
  }
  console.log(`gis-native-codec-oracle: receipts=2 hostile=${fixture.hostile.length} ajv+node+webcrypto=1; no catalog activation or GIS execution claim`);
}

/** 🧷 Exact native receipt proof; it does not activate a hub catalog or inference executor. */
export class NativeCodecCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisNativeCodecReceipts(this.repoRoot);
    await proveGisControlledProposal(this.repoRoot);
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      groups: [{ package: "semio-s-plugin-gis", target: { kind: "test", name: "native_codecs" }, cargoArgs: ["--no-default-features"], laws: ["gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution", "gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listBudgetMs: 60_000, lawBudgetMs: 60_000,
      progress(event) { console.log(`gis-native-codecs ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-native-codec-receipt: ${JSON.stringify(receipt)}`);
    console.log("gis-native-codec-check: exact=2 literal-codecs=2 controlled-proposal=1; no hub catalog activation or approved inference acceptance");
  }
}
