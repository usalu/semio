#!/usr/bin/env bun
/** 🌍️ `@semio-tech/gis-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

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

/** 🪢 Independently validates literal GIS codec identity and protocol-byte receipts. */
export async function proveGisNativeCodecReceipts(repoRoot: string): Promise<void> {
  const owner = join(repoRoot, "✏️s/🔌️plugins/🌍️gis");
  const root = join(owner, "📇️native-codecs");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid GIS receipt corpus: ${JSON.stringify(validate.errors)}`);
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
  const root = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧪️fixtures/💡️inference-control");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
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
  const root = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🧩️map-create-region-group");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
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
  console.log(`gis-map-create-region-group-check: checks=${14 + fixture.hostile.length} clean; atomic durable publication not claimed`);
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
      groups: [{ package: "semio-s-plugin-gis", target: { kind: "lib" }, cargoArgs: ["--no-default-features"], laws: ["artifacts::gismap::standards::v1::subsets::any::schema::inferences::tests::map_create_region_group_work_stabilizes_parent_drawing_value_without_image"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: 3_600_000,
      listBudgetMs: 60_000,
      lawBudgetMs: 60_000,
      progress(event) { console.log(`gis-map-create-region-group ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-map-create-region-group-receipt: ${JSON.stringify(receipt)}`);
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

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript).register("native-codec-check", NativeCodecCheckScript).register("map-create-region-group-check", MapCreateRegionGroupCheckScript).register("map-create-region-group-native-check", MapCreateRegionGroupNativeCheckScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
