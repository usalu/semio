import sys

p = sys.argv[1]
s = open(p, encoding="utf-8", newline="").read()
crlf = "\r\n" in s
s = s.replace("\r\n", "\n")


def rep(a, b):
    global s
    assert s.count(a) == 1, (a, s.count(a))
    s = s.replace(a, b)


rep('import { realpathSync } from "node:fs";', 'import { readFileSync, realpathSync } from "node:fs";')
rep('import { type Asset, type Environment, type EnvironmentPreset } from "../../../🧬️schema/🟦️.ts";',
    'import { type Asset, type AuthoredRepresentation, type Environment, type EnvironmentPreset, SCALE_DENOMINATORS } from "../../../🧬️schema/🟦️.ts";')
rep("import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from",
    "import { DEFAULT_CATALOG_ROOT, type LoadedAsset, loadCatalogFromFiles, loadEnvironments } from")
rep('''import { type Cast, type CastIndex, type CastManifest, parseCastManifest } from "./🧬️schema/🟦️.ts";

const DEFAULT_SCALE = 50;
const DEFAULT_FPS = 15;
const MARGIN_MM''', '''import { type Cast, type CastIndex, type CastManifest, type CastProvenance, type CastScenario, type Revision, parseCastManifest } from "./🧬️schema/🟦️.ts";

const MARGIN_MM''')
rep('''const round = (value: number, digits = 6): number => Math.round(value * 10 ** digits) / 10 ** digits;
''', '''const round = (value: number, digits = 6): number => Math.round(value * 10 ** digits) / 10 ** digits;

/** 🔖️ The revision of a catalog file: sha256 of its text with CRLF turned into LF, so a checkout with and one without `core.autocrlf` agree. */
export const revisionOfText = (text: string): Revision => `sha256:${createHash("sha256").update(text.replace(/\\r\\n/gu, "\\n"), "utf8").digest("hex")}`;

/** 🧾️ Canonical JSON: sorted keys, no whitespace, non-ASCII kept, numbers as ECMAScript prints them; the Python loader writes the same text, so both compute one revision for one preset. */
export function canonicalJson(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(",")}]`;
  if (typeof value === "object" && value !== null) return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalJson((value as Record<string, unknown>)[key])}`).join(",")}}`;
  return JSON.stringify(value);
}

/** 🔖️ The revision of an environment preset: sha256 of its canonical JSON. */
export const presetRevision = (preset: EnvironmentPreset): Revision => `sha256:${createHash("sha256").update(canonicalJson(preset), "utf8").digest("hex")}`;
''')
rep("frameSchedule(clip, cast.fps ?? DEFAULT_FPS)", "frameSchedule(clip, cast.fps!)")
rep("frameSchedule({ durationS: end, loop: false }, cast.fps ?? DEFAULT_FPS)", "frameSchedule({ durationS: end, loop: false }, cast.fps!)")
rep('ease: key.ease ?? "linear" }', "ease: key.ease }")
rep('scale: { denominator: cast.scale ?? DEFAULT_SCALE } } as const;', "scale: { denominator: cast.scale } } as const;")
rep("textlessDepictionOptions(cast.view, cast.scale ?? DEFAULT_SCALE, cast.viewpoint, cast.theme)", "textlessDepictionOptions(cast.view, cast.scale, cast.viewpoint, cast.theme)")
rep('''  const representation = asset.representations[`${cast.scale ?? DEFAULT_SCALE}` as keyof Asset["representations"]];
  const section = sectionOf(representation?.status === "authored" ? representation : undefined, cast.viewpoint);''',
    '''  const section = sectionOf(authoredRepresentation(id, asset, cast), cast.viewpoint);''')
rep("async function render(id: string, cast: Cast, asset: Asset, presets", "async function render(id: string, cast: Cast, asset: Asset, revision: Revision, presets")
rep('''    heightM: round(heightM),
  };
}''', '''    heightM: round(heightM),
    provenance: provenanceOf(cast, asset, revision, presets, { fps: schedule.fps, durationS: schedule.durationS, loop: schedule.loop, frames: frames.length }),
  };
}''')
rep("function environmentOf(", '''/** 🗺️ The authored representation a cast draws: exactly the authored preset scale, with geometry for the viewpoint and, for the thermal view, thermal regions. Unavailable, derived and empty combinations throw. */
function authoredRepresentation(id: string, asset: Asset, cast: Cast): AuthoredRepresentation {
  const authored = SCALE_DENOMINATORS.filter((denominator) => asset.representations[`${denominator}`]?.status === "authored");
  const representation = asset.representations[`${cast.scale}`];
  if (representation?.status !== "authored") throw new Error(`cast ${id}: asset ${asset.id} has no authored representation at 1:${cast.scale}; authored scales: ${authored.join(", ")}`);
  if (representation.geometry[cast.viewpoint].length === 0) throw new Error(`cast ${id}: asset ${asset.id} has no ${cast.viewpoint} geometry at 1:${cast.scale}`);
  if (cast.view === "thermal" && asset.thermal.regions.length === 0) throw new Error(`cast ${id}: asset ${asset.id} has no thermal regions for the thermal view`);
  return representation;
}

/** 🔖️ The provenance record of a cast: every dimension of the drawing named by the cast contract, with the asset and environment revisions. */
function provenanceOf(cast: Cast, asset: Asset, revision: Revision, presets: ReadonlyMap<string, EnvironmentPreset>, clock: { readonly fps: number; readonly durationS: number; readonly loop: boolean; readonly frames: number }): CastProvenance {
  const revisionOf = (slug: string): Revision => presetRevision(presets.get(slug)!);
  const thermal = { spanK: cast.thermalSpanK ?? null, transport: cast.transport ?? [] };
  const scenario: CastScenario =
    cast.view === "thermal"
      ? cast.timeline
        ? { kind: "thermal", timeline: [...cast.timeline].sort((a, b) => a.t - b.t).map((key) => ({ t: key.t, id: key.environment, ease: key.ease, revision: revisionOf(key.environment) })), ...thermal }
        : { kind: "thermal", environment: { id: cast.environment!, revision: revisionOf(cast.environment!) }, ...thermal }
      : cast.view === "forces" || cast.view === "moments"
        ? { kind: "structural", dynamics: asset.structure.dynamics, gravityMS2: asset.structure.gravityMS2 }
        : { kind: "geometry" };
  return {
    version: 1,
    asset: { id: asset.id, category: asset.category, revision },
    view: { style: cast.view, viewpoint: cast.viewpoint, theme: cast.theme, mirror: cast.mirror === true },
    scale: { denominator: cast.scale },
    state: cast.clip !== undefined ? { kind: "clip", id: cast.clip } : { kind: "pose", id: cast.pose! },
    clock: { ...clock, offsetS: cast.clip !== undefined ? cast.transportAgeS ?? 0 : 0 },
    scenario,
  };
}

function environmentOf(''')
rep("async function exportOne(directory: string, out: string, plan: { readonly id: string; readonly cast: Cast; readonly asset: Asset }, presets",
    "async function exportOne(directory: string, out: string, plan: { readonly id: string; readonly cast: Cast; readonly asset: Asset; readonly revision: Revision }, presets")
rep("render(plan.id, plan.cast, plan.asset, presets,", "render(plan.id, plan.cast, plan.asset, plan.revision, presets,")
rep('''    const asset = catalog.assets.find((candidate) => candidate.id === cast.asset);
    if (!asset) throw new Error(`cast ${id}: asset ${cast.asset} is not in the catalog ${root}`);
    environmentOf(cast, presets);
    return { id, cast, asset, count: framesOf(cast, asset).schedule.count };''',
    '''    const found = catalog.loaded.filter((candidate) => candidate.asset.id === cast.asset);
    if (found.length === 0) throw new Error(`cast ${id}: asset ${cast.asset} is not in the catalog ${root}`);
    if (found.length > 1) throw new Error(`cast ${id}: asset ${cast.asset} is in the catalog ${root} ${found.length} times: ${found.map((candidate) => candidate.path).join(", ")}`);
    const { asset, path, folder } = found[0] as LoadedAsset;
    if (folder !== asset.id) throw new Error(`cast ${id}: asset ${asset.id} lives in the folder ${folder}, the folder must carry the asset id`);
    environmentOf(cast, presets);
    authoredRepresentation(id, asset, cast);
    return { id, cast, asset, revision: revisionOfText(readFileSync(path, "utf8")), count: framesOf(cast, asset).schedule.count };''')
if crlf:
    s = s.replace("\n", "\r\n")
open(p, "w", encoding="utf-8", newline="").write(s)
