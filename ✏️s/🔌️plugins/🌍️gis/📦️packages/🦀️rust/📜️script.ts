#!/usr/bin/env bun
/** 🌍️ `@semio-tech/gis-plugin` router: `bun ./📜️script.ts test`. */
import { isAbsolute, join, relative, resolve } from "node:path";
import { mkdirSync, mkdtempSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import {
  describePluginComponent,
  produceFreshComponentV1,
  type FreshBuildControlV1,
} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await proveGisNativeCodecReceipts(this.repoRoot);
    await runCargoTestBudgeted(["semio-s-plugin-gis"], this.repoRoot, rest);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-gis", join(this.root, "..", "..")));
  }
}

const GIS_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json";
const GIS_MAP_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json";

/** 🧬️ Compiles one PascalCase `$defs` export of a scope-owned draft-07 schema module.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json */
async function compileGisScopeExport(repoRoot: string, moduleRel: string, exportId: string) {
  const module = JSON.parse(readFileSync(join(repoRoot, moduleRel), "utf8"));
  const Ajv = (await import("ajv")).default;
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  ajv.addKeyword("x-semio-state").addFormat("double", true);
  ajv.addSchema(module);
  return ajv.compile({ $ref: `${module.$id}#/$defs/${exportId}` });
}

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

/** 🌐️ Validates the literal bounded proposal independently of the native package. */
export async function proveGisControlledProposal(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧫️fixtures/💡️inference-control");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisInferenceControl");
  if (!validate(fixture)) throw new Error(`invalid GIS controlled corpus: ${JSON.stringify(validate.errors)}`);
  const points = [[fixture.snapshot.positions[0].data.lon, fixture.snapshot.positions[0].data.lat], ...fixture.snapshot.routes[0].data.points];
  const x = points.map((point: number[]) => point[0]), y = points.map((point: number[]) => point[1]);
  const [west, east, south, north] = [Math.min(...x), Math.max(...x), Math.min(...y), Math.max(...y)];
  const bounds = { lonMin: west, lonMax: east, latMin: south, latMax: north };
  if (JSON.stringify(bounds) !== JSON.stringify(fixture.expected.bounds)) throw new Error("GIS independent bounds mismatch");
  const proposalId = `inference-${fixture.proposalJobId}`;
  const proposal = { CreateRegion: { index: fixture.snapshot.regions.length, item: { id: proposalId, data: { id: proposalId, kind: "inference-bounds", ring: [[west, south], [east, south], [east, north], [west, north], [west, south]] } } } };
  if (JSON.stringify(proposal) !== JSON.stringify(fixture.proposal)) throw new Error("GIS independent proposal mismatch");
  for (const row of fixture.proposalRejections) {
    const state = structuredClone(fixture.snapshot), candidate = structuredClone(fixture.expected);
    let jobId = fixture.proposalJobId;
    switch (row.case) {
      case "wrong-job": jobId = "not-a-job"; break;
      case "duplicate-id": state.regions.push({ id: `inference-${jobId}`, data: null }); candidate.regionCount = state.regions.length; break;
      case "stale-count": candidate.positionCount++; break;
      case "no-bounds": candidate.bounds = null; break;
      case "non-finite": candidate.bounds.lonMin = NaN; break;
      case "out-of-range": candidate.bounds.lonMin = -181; break;
      case "reversed": candidate.bounds.latMin = 49; break;
      default: throw new Error("unhandled proposal rejection");
    }
    const b = candidate.bounds;
    const error = !/^[a-f0-9]{32}$/u.test(jobId) ? "Identity"
      : candidate.positionCount !== state.positions.length || candidate.routeCount !== state.routes.length || candidate.regionCount !== state.regions.length || state.regions.some((region: any) => region.id === `inference-${jobId}`) ? "Stale"
      : !b || ![b.lonMin, b.lonMax, b.latMin, b.latMax].every(Number.isFinite) || b.lonMin < -180 || b.lonMax > 180 || b.latMin < -90 || b.latMax > 90 || b.lonMin > b.lonMax || b.latMin > b.latMax ? "Bounds" : null;
    if (error !== row.error) throw new Error(`GIS proposal rejection mismatch ${row.case}`);
  }
  for (const row of fixture.interruptions) {
    const first = fixture.checkpoints.indexOf(row.at);
    if (first < 0 || first + 1 !== row.calls) throw new Error("GIS interruption trace permits later work");
  }
  console.log("gis-controlled-proposal-oracle: literal=1 bounds=1 interruption=3 rejection=7 ajv=1; no hub approval authority");
}

/** 🧩️ Proves the neutral stable-child, typed parent+drawing+value CreateRegion group contract. */
export async function proveGisMapCreateRegionGroup(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🧩️map-create-region-group");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_MAP_SCHEMA_MODULE, "GisMapCreateRegionGroup");
  if (!validate(fixture)) throw new Error(`invalid GIS Map group corpus: ${JSON.stringify(validate.errors)}`);
  const points = [
    ...fixture.base.positions.map((feature: any) => [feature.data.lon, feature.data.lat]),
    ...fixture.base.routes.flatMap((feature: any) => feature.data.points),
    ...fixture.base.regions.flatMap((feature: any) => feature.data.ring ?? feature.data.points ?? []),
  ];
  const xs = points.map((point: number[]) => point[0]), ys = points.map((point: number[]) => point[1]);
  const id = `inference-${fixture.jobId}`;
  const ring = [[Math.min(...xs), Math.min(...ys)], [Math.max(...xs), Math.min(...ys)], [Math.max(...xs), Math.max(...ys)], [Math.min(...xs), Math.max(...ys)], [Math.min(...xs), Math.min(...ys)]];
  const region = { id, data: { id, kind: "inference-bounds", ring } };
  const admitted = (candidate: any): boolean => validate(candidate)
    && candidate.expected.region.id === `inference-${candidate.jobId}`
    && candidate.expected.region.data.id === candidate.expected.region.id
    && candidate.expected.drawing.index === candidate.base.positions.length + candidate.base.routes.length + candidate.base.regions.length
    && candidate.expected.value.index === candidate.base.regions.length
    && !candidate.expected.imageTouched
    && JSON.stringify(candidate.expected.touchedChildren) === JSON.stringify(["drawing", "value"]);
  if (!admitted(fixture) || JSON.stringify(region) !== JSON.stringify(fixture.expected.region)) throw new Error("GIS Map group contract mismatch");
  const source = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"), "utf8");
  const owner = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs"), "utf8");
  const schema = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"), "utf8");
  for (const symbol of ["GisMapCreateRegionGroupWorkV1", "create_region_group_work", "SemioDrawingMutation", "SemioValueMutation", "drawing_inverse", "value_inverse"]) {
    if (!source.includes(symbol)) throw new Error(`GIS Map typed group owner missing ${symbol}`);
  }
  for (const fragment of ["after_children.starts_with(before_children)", "projected_drawing != after_drawing", "projected_value != after_value", "bytes > 65_536", '("id".into(), dsl::DslValue::String(id.clone()))']) {
    if (!source.includes(fragment)) throw new Error(`GIS Map typed group invariant missing ${fragment}`);
  }
  if (!schema.includes('value.get("points").or_else(|| value.get("ring"))')) throw new Error("GIS Map ring geometry is not projected");
  if (!owner.includes('let child_id = "gismap-drawing".to_string()') || !owner.includes('let child_id = "gismap-value".to_string()') || owner.includes("content_hash:016x")) throw new Error("GIS Map child identities are not stable");
  for (const hostile of fixture.hostile) {
    const candidate = structuredClone(fixture);
    if (hostile === "drawing-id") candidate.base.drawingChildId = "forged";
    if (hostile === "value-id") candidate.base.valueChildId = "forged";
    if (hostile === "missing-drawing") candidate.expected.touchedChildren.shift();
    if (hostile === "missing-value") candidate.expected.touchedChildren.pop();
    if (hostile === "image-touch") candidate.expected.imageTouched = true;
    if (hostile === "wrong-index") candidate.expected.drawing.index++;
    if (hostile === "wrong-job") candidate.jobId = "not-a-job";
    if (hostile === "oversize") candidate.expected.maximumBytes++;
    if (admitted(candidate)) throw new Error(`GIS Map hostile group admitted ${hostile}`);
  }
  const validateMembership = await compileGisScopeExport(repoRoot, GIS_MAP_SCHEMA_MODULE, "GisMapCreateRegionGroupMembership");
  for (const row of fixture.membershipCases) {
    const membership = { drawingChildId: row.deriveChildren ? "gismap-drawing" : row.drawingChildId, valueChildId: row.deriveChildren ? "gismap-value" : row.valueChildId, imageChildId: row.imageChildId };
    const accepted = membership.drawingChildId === "gismap-drawing" && membership.valueChildId === "gismap-value" && membership.imageChildId === null;
    if (validateMembership(membership) !== row.accepted || accepted !== row.accepted) throw new Error(`GIS Map membership parity failed: ${row.name}`);
  }
  const nativeTests = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  if (!nativeTests.includes('fixture["membershipCases"]')) throw new Error("GIS Map native law does not consume membership cases");
  console.log(`gis-map-create-region-group-check: checks=${14 + fixture.hostile.length + fixture.membershipCases.length} clean; atomic durable publication not claimed`);
}

/** 🏗️ Proves the exact fixed-three GIS assembly schema, typed factory ports, and private Store bind. */
export async function proveGisDurableThreeStoreAssembly(repoRoot: string): Promise<void> {
  const fixtureRoot = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🗄️durable-three-store-assembly");
  const fixtureBytes = readFileSync(join(fixtureRoot, "🔣️.json"));
  const fixture = JSON.parse(fixtureBytes.toString("utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisDurableThreeStoreAssembly");
  if (!validate(fixture)) throw new Error(`invalid GIS durable three-Store assembly corpus: ${JSON.stringify(validate.errors)}`);
  const identity = [
    fixture.schema,
    fixture.shape,
    fixture.operation,
    fixture.actor,
    `parent:${fixture.members[0].document}`,
    `drawing:${fixture.members[1].document}:${fixture.members[1].owner}`,
    `value:${fixture.members[2].document}:${fixture.members[2].owner}`,
  ].join("|");
  const nodeHash = createHash("sha256").update(identity).digest("hex");
  const webHash = Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(identity))).toString("hex");
  if (identity !== fixture.canonicalIdentity || nodeHash !== fixture.canonicalIdentitySha256 || webHash !== nodeHash) throw new Error("GIS durable assembly identity oracle mismatch");
  if (JSON.stringify(fixture.members.map((member: any) => member.role)) !== JSON.stringify(["parent", "drawing", "value"])) throw new Error("GIS durable assembly role order changed");
  if (JSON.stringify(fixture.preparationOrder) !== JSON.stringify(["parent", "drawing", "value"])) throw new Error("GIS durable assembly preparation order changed");
  if (fixture.invariants.callerSuppliesGroupId || fixture.invariants.exposesPreparedSeal || fixture.invariants.publishBeforeAllPrepared || fixture.invariants.terminalReturnsStores !== 3 || fixture.invariants.journalBegins !== 1) {
    throw new Error("GIS durable assembly ownership invariants changed");
  }
  if (fixture.cancellationCases.some((row: any) => row.journalBegins !== 0 || row.terminalStores !== 3) || fixture.rejectionCases.some((row: any) => row.terminalStores !== 3)) {
    throw new Error("GIS durable assembly terminal owner trace changed");
  }
  const storeSource = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs"), "utf8");
  const assembly = storeSource.slice(storeSource.indexOf("pub struct DurableOwnedThreeStoreMapAssemblyV1"), storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreBoundV1"));
  for (const marker of ["DurableOwnedMapMemberAdmissionV1", "AdmittingParent", "AdmittingDrawing", "AdmittingValue", "PreparingParent", "PreparingDrawing", "PreparingValue", "take_assembly_prepared", "bind_store_owned", "mount_map", "take_mounted_host", "take_terminal_owners"]) {
    if (!assembly.includes(marker)) throw new Error(`Store durable assembly missing ${marker}`);
  }
  if (assembly.includes("begin_member_apply_one") || assembly.includes("group_id:") || !assembly.includes("begin_apply_one(")) throw new Error("Store durable assembly admits a caller group or catalog factory shortcut");
  const journal = storeSource.slice(storeSource.indexOf("DurableOwnedThreeStoreCommitPhaseV1::StartingJournal =>"), storeSource.indexOf("DurableOwnedThreeStoreCommitPhaseV1::Journal =>"));
  if (!journal.includes("sink.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?") || (journal.match(/\.begin_commit\(/gu) ?? []).length !== 1) throw new Error("Store durable journal does not begin exactly once");
  const gisSource = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), "utf8");
  for (const builder of ["gis_map_parent_one_item_preparation_factory", "gis_map_drawing_one_item_preparation_factory", "gis_map_value_one_item_preparation_factory"]) {
    if (!gisSource.includes(builder)) throw new Error(`GIS exact preparation port missing ${builder}`);
  }
  console.log(`gis-durable-three-store-assembly-oracle: AJV=1 SHA256=node+webcrypto roles=3 cancellation=${fixture.cancellationCases.length} rejection=${fixture.rejectionCases.length}; no WAL recovery or Hub publication claim`);
}

/** 🌉️ Proves the receipt-bound GIS cold-pair, tiled-map patch, and addressed mutation corpus. */
export async function proveGisComponentColdMapPatch(repoRoot: string): Promise<void> {
  const fixtureRoot = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch");
  const fixtureBytes = readFileSync(join(fixtureRoot, "🔣️.json"));
  const fixture = JSON.parse(fixtureBytes.toString("utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisComponentColdMapPatch");
  if (!validate(fixture)) throw new Error(`invalid GIS component cold-map corpus: ${JSON.stringify(validate.errors)}`);
  const nodeHash = createHash("sha256").update(fixtureBytes).digest("hex");
  const webHash = Buffer.from(await crypto.subtle.digest("SHA-256", fixtureBytes)).toString("hex");
  if (nodeHash !== webHash || /^0{64}$/u.test(nodeHash)) throw new Error("GIS component corpus SHA-256 oracle mismatch");
  const owner = structuredClone(fixture);
  const admitted = (candidate: any): boolean =>
    validate(candidate) &&
    candidate.package.pluginId === "gis" &&
    candidate.package.packageId === "semio:gis" &&
    candidate.package.cargoPackage === "semio-s-plugin-gis" &&
    candidate.package.componentFile === "semio_s_plugin_gis.wasm" &&
    candidate.app.id === "s.gis.gismap@1/*#editor" &&
    candidate.app.actionId === "patchPositions" &&
    candidate.surface === "1:gis2d.play.composite" &&
    candidate.authority.activationGeneration > 0 &&
    candidate.authority.transferGeneration > 0 &&
    candidate.receipt.componentSha256 === owner.receipt.componentSha256 &&
    candidate.receipt.descriptorSha256 === owner.receipt.descriptorSha256;
  if (!admitted(fixture)) throw new Error("GIS component cold-map owner was denied");
  for (const hostile of fixture.hostile) {
    const candidate = structuredClone(fixture);
    switch (hostile) {
      case "foreign-package":
        candidate.package.packageId = "semio:stdio";
        break;
      case "stale-lifetime":
        candidate.authority.activationGeneration = 0;
        break;
      case "wrong-descriptor":
        candidate.receipt.descriptorSha256 = "00".repeat(32);
        break;
      case "wrong-action-owner":
        candidate.app.actionId = "replacePositions";
        break;
      case "changed-component-after-receipt":
        candidate.receipt.componentSha256 = "44".repeat(32);
        break;
      default:
        throw new Error(`unhandled GIS component hostile ${hostile}`);
    }
    if (admitted(candidate)) throw new Error(`GIS component hostile admitted ${hostile}`);
  }
  const testSource = readFileSync(join(fixtureRoot, "🦀️.rs"), "utf8");
  for (const marker of [
    "ColdDocumentPairPage",
    "ColdPairIngressStatus::Applied",
    "TiledMapScene",
    "UiTurnPatchTransportLease",
    "ActionInvocation",
    "\"patchPositions\"",
    "Event::PatchAck",
    "SEMIO_GIS_COMPONENT_SHA256",
    "SEMIO_GIS_DESCRIPTOR_SHA256",
  ]) {
    if (!testSource.includes(marker)) throw new Error(`GIS component acceptance omits ${marker}`);
  }
  const manifest = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml"), "utf8");
  if (!manifest.includes('name = "component_cold_map_patch"') || !manifest.includes("semio-framework-plugin-host") || !manifest.includes("semio-framework-ui-scene")) {
    throw new Error("GIS component acceptance is not mounted with its exact first-party host and scene owners");
  }
  console.log(`gis-component-cold-map-patch-source: AJV=1 SHA256=node+webcrypto hostile=${fixture.hostile.length} markers=9; no GIS component build or browser acceptance claim`);
}

function freshGisComponentBuildControl(): { readonly control: FreshBuildControlV1; close(): void } {
  const duration = Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 86_400_000);
  if (!Number.isSafeInteger(duration) || duration < 60_000 || duration > 86_400_000) throw new Error("GIS component build budget must be between one minute and 24 hours");
  const deadline = Date.now() + duration;
  let interrupted = false;
  const interrupt = () => {
    interrupted = true;
  };
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", interrupt);
  return {
    control: Object.freeze({
      diagnosticsRoot: undefined,
      cancelled: () => interrupted,
      remainingMs: () => Math.max(0, deadline - Date.now()),
      checkpoint(stage, completed, total) {
        if (interrupted || Date.now() >= deadline) throw new Error(`GIS component production cancelled during ${stage}`);
        console.log(`gis-component-production: ${stage} ${completed}/${total}`);
      },
    }),
    close() {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", interrupt);
    },
  };
}

class ComponentColdMapPatchCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisComponentColdMapPatch(this.repoRoot);
  }
}

class ComponentColdMapPatchNativeCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisComponentColdMapPatch(this.repoRoot);
    const configuredArtifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    const ticketRoot = resolve(this.repoRoot, ".🧬semio", "🦑️repo", "🎫️tickets");
    const artifactRoot = configuredArtifactRoot ? resolve(configuredArtifactRoot) : "";
    const ticketRelative = artifactRoot ? relative(ticketRoot, artifactRoot) : "";
    if (!artifactRoot || !isAbsolute(artifactRoot) || !ticketRelative || ticketRelative.startsWith("..") || isAbsolute(ticketRelative) || !artifactRoot.split(/[\\/]/u).includes("🗑️generated")) {
      throw new Error("GIS component native acceptance requires ticket-owned SEMIO_TEST_ARTIFACT_DIR");
    }
    mkdirSync(artifactRoot, { recursive: true, mode: 0o700 });
    const runRoot = mkdtempSync(join(artifactRoot, "gis-component-cold-map-patch-"));
    const target = join(runRoot, "producer-target");
    const stage = join(runRoot, "stage");
    const diagnostics = join(runRoot, "producer-diagnostics");
    mkdirSync(target, { mode: 0o700 });
    mkdirSync(stage, { mode: 0o700 });
    mkdirSync(diagnostics, { mode: 0o700 });
    const build = freshGisComponentBuildControl();
    const control = Object.freeze({ ...build.control, diagnosticsRoot: diagnostics });
    try {
      const produced = await produceFreshComponentV1(
        this.repoRoot,
        {
          pluginId: "gis",
          cargoPackage: "semio-s-plugin-gis",
          componentPackageId: "semio:gis",
          outputName: "semio_s_plugin_gis.wasm",
          componentProfile: "wasm-release",
          rootCdylib: true,
        },
        target,
        stage,
        control,
        (lease) =>
          lease.consume(async (component) => {
            const componentSha256 = createHash("sha256").update(component).digest("hex");
            const stagedComponent = readFileSync(join(stage, "component.wasm"));
            const descriptor = readFileSync(join(stage, "descriptor.semio"));
            const descriptorSha256 = createHash("sha256").update(descriptor).digest("hex");
            if (createHash("sha256").update(stagedComponent).digest("hex") !== componentSha256) throw new Error("leased GIS component differs from staged component");
            const receipts = await runExactCargoLaws({
              cwd: this.repoRoot,
              groups: [
                {
                  package: "semio-s-plugin-gis",
                  target: { kind: "test", name: "component_cold_map_patch" },
                  cargoArgs: ["--no-default-features"],
                  laws: [
                    "genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface",
                    "genuine_gis_component_rejects_stale_cold_authority_before_loading",
                  ],
                },
              ],
              artifactDir: join(artifactRoot, "exact"),
              env: {
                ...process.env,
                SEMIO_GIS_COMPONENT_WASM: join(stage, "component.wasm"),
                SEMIO_GIS_COMPONENT_SHA256: componentSha256,
                SEMIO_GIS_DESCRIPTOR_SHA256: descriptorSha256,
              },
              nativeEnv: { RUST_MIN_STACK: process.env.SEMIO_TEST_NATIVE_RUST_MIN_STACK ?? "268435456" },
              buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 86_400_000),
              listBudgetMs: 120_000,
              lawBudgetMs: 300_000,
              cancelled: control.cancelled,
              progress(event) {
                console.log(`gis-component-cold-map-patch ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`);
              },
            });
            return { componentSha256, descriptorSha256, receipts };
          }),
      );
      if (
        produced.receipt.component.sha256 !== produced.derived.componentSha256 ||
        produced.receipt.descriptor.sha256 !== produced.derived.descriptorSha256 ||
        produced.receipt.pluginId !== "gis" ||
        produced.receipt.packageId !== "semio:gis"
      ) {
        throw new Error("GIS component native acceptance differs from its fresh producer receipt");
      }
      for (const receipt of produced.derived.receipts) console.log(`gis-component-cold-map-patch-receipt: ${JSON.stringify(receipt)}`);
      console.log(
        `gis-component-cold-map-patch-native: component=${produced.receipt.component.sha256} descriptor=${produced.receipt.descriptor.sha256} exact=2 stage=${stage}; genuine Wasmtime GIS boundary only, Hub selection and Shell browser acceptance remain separate`,
      );
    } finally {
      build.close();
    }
  }
}

class MapCreateRegionGroupCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisMapCreateRegionGroup(this.repoRoot);
  }
}

class MapCreateRegionGroupNativeCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisMapCreateRegionGroup(this.repoRoot);
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      groups: [{ package: "semio-s-artifact-gis-gismap", target: { kind: "lib" }, cargoArgs: ["--no-default-features"], laws: ["standards::v1::subsets::any::schema::inferences::component::tests::map_create_region_group_work_stabilizes_parent_drawing_value_without_image"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: 3_600_000,
      listBudgetMs: 60_000,
      lawBudgetMs: 60_000,
      progress(event) { console.log(`gis-map-create-region-group ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-map-create-region-group-receipt: ${JSON.stringify(receipt)}`);
  }
}

class DurableThreeStoreAssemblyCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisDurableThreeStoreAssembly(this.repoRoot);
  }
}

class DurableThreeStoreAssemblyNativeCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisDurableThreeStoreAssembly(this.repoRoot);
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      groups: [
        { package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [
          "durable_group::tests::durable_map_three_store_assembly_uses_exact_gis_factories_and_binds_one_decision",
          "durable_group::tests::durable_map_three_store_assembly_late_member_rejection_closes_prior_publications_before_owner_handoff",
          "durable_group::tests::durable_map_three_store_assembly_cancellation_before_journal_restores_all_three_frontiers",
          "durable_group::tests::durable_map_three_store_assembly_uncertain_journal_retains_same_host_until_committed_or_proven_absent",
        ] },
        { package: "semio-s-artifact-gis-gismap", target: { kind: "lib" }, cargoArgs: ["--no-default-features", "--features", "component-app-assembly"], laws: [
          "editor::gis2d::component::tests::gis_map_durable_three_store_factory_builders_are_exact_role_ports",
        ] },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`gis-durable-three-store-assembly ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-durable-three-store-assembly-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🧷 Exact native receipt proof; it does not activate a hub catalog or inference executor. */
class NativeCodecCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisNativeCodecReceipts(this.repoRoot);
    await proveGisControlledProposal(this.repoRoot);
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      groups: [{ package: "semio-s-plugin-gis", target: { kind: "test", name: "native_codecs" }, cargoArgs: ["--no-default-features"], laws: ["gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution", "gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: 3_600_000, listBudgetMs: 60_000, lawBudgetMs: 60_000,
      progress(event) { console.log(`gis-native-codecs ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-native-codec-receipt: ${JSON.stringify(receipt)}`);
    console.log("gis-native-codec-check: exact=2 literal-codecs=2 controlled-proposal=1; no hub catalog activation or approved inference acceptance");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("describe", DescribeScript)
  .register("native-codec-check", NativeCodecCheckScript)
  .register("map-create-region-group-check", MapCreateRegionGroupCheckScript)
  .register("map-create-region-group-native-check", MapCreateRegionGroupNativeCheckScript)
  .register("durable-three-store-assembly-check", DurableThreeStoreAssemblyCheckScript)
  .register("durable-three-store-assembly-native-check", DurableThreeStoreAssemblyNativeCheckScript)
  .register("component-cold-map-patch-check", ComponentColdMapPatchCheckScript)
  .register("component-cold-map-patch-native-check", ComponentColdMapPatchNativeCheckScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
