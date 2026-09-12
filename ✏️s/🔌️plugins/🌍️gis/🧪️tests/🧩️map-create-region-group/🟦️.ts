import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { compileGisScopeExport } from "../../🧬️schema/🟦️.ts";

const GIS_MAP_ARTIFACT_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json";
const GIS_MAP_CONTROL_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🧪️tests/🧩️map-create-region-group/🧬️schema/🔣️.json";
const GIS_MAP_SCHEMA_DEPENDENCIES = [
  "🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json",
  "🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📍️feature/🔣️.json",
] as const;

/** 🧩️ Proves the neutral stable-child, typed parent+drawing+value CreateRegion group contract. */
export async function proveGisMapCreateRegionGroup(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🧩️map-create-region-group");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validateArtifact = await compileGisScopeExport(repoRoot, GIS_MAP_ARTIFACT_SCHEMA_MODULE, null, GIS_MAP_SCHEMA_DEPENDENCIES);
  const target = { artifactId: "artifact-11111111111111111111111111111111", dialect: { artifactKind: "s.stdio.semio", standard: "1", subset: "*" } };
  const artifact = { positions: fixture.base.positions, routes: fixture.base.routes, regions: fixture.base.regions, drawing: { childId: fixture.base.drawingChildId, target }, image: null, value: { childId: fixture.base.valueChildId, target } };
  if (!validateArtifact(artifact)) throw new Error(`invalid GIS Map artifact projection: ${JSON.stringify(validateArtifact.errors)}`);
  const validate = await compileGisScopeExport(repoRoot, GIS_MAP_CONTROL_SCHEMA_MODULE, "GisMapCreateRegionGroup");
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
  const validateMembership = await compileGisScopeExport(repoRoot, GIS_MAP_CONTROL_SCHEMA_MODULE, "GisMapCreateRegionGroupMembership");
  for (const row of fixture.membershipCases) {
    const membership = { drawingChildId: row.deriveChildren ? "gismap-drawing" : row.drawingChildId, valueChildId: row.deriveChildren ? "gismap-value" : row.valueChildId, imageChildId: row.imageChildId };
    const accepted = membership.drawingChildId === "gismap-drawing" && membership.valueChildId === "gismap-value" && membership.imageChildId === null;
    if (validateMembership(membership) !== row.accepted || accepted !== row.accepted) throw new Error(`GIS Map membership parity failed: ${row.name}`);
  }
  const nativeTests = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  if (!nativeTests.includes('fixture["membershipCases"]')) throw new Error("GIS Map native law does not consume membership cases");
  console.log(`gis-map-create-region-group-check: checks=${14 + fixture.hostile.length + fixture.membershipCases.length} clean; atomic durable publication not claimed`);
}


export class MapCreateRegionGroupCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisMapCreateRegionGroup(this.repoRoot);
  }
}

export class MapCreateRegionGroupNativeCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisMapCreateRegionGroup(this.repoRoot);
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      groups: [{ package: "semio-s-artifact-gis-gismap", target: { kind: "lib" }, cargoArgs: ["--no-default-features"], laws: ["standards::v1::subsets::any::schema::inferences::component::tests::map_create_region_group_work_stabilizes_parent_drawing_value_without_image"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 60_000,
      progress(event) { console.log(`gis-map-create-region-group ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-map-create-region-group-receipt: ${JSON.stringify(receipt)}`);
  }
}
