import { readFileSync } from "node:fs";
import { join } from "node:path";
import { compileGisScopeExport } from "../../🧬️schema/🟦️.ts";

const GIS_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json";

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

