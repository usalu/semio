/** 🧹️ All-asset thermal, transport and view sweep: every catalog asset, every authored scale, all seven views in the three viewpoints, several environments, the default pose, the start, quarter points, middle and end of every clip, and transitions between poses.
 *
 * Hard checks per instant: every view renders; a thermal region drawn in some view has the solved value of the field in every view where it is drawn (and so the same value in all of them); the thermal legends of the three viewpoints are equal;
 * a region not drawn in a viewpoint is named in its legend; a region drawn in no view is a finding unless it is air-borne (its `hidden` note says so in en and de and an emitter anchored to the region traces it: `air-borne (traced by <emitter>)`) or carries a `hidden` note in en and de (D40: `hidden (D40)`); both are printed as `[NOTED]`;
 * emitter markers lie inside the paper; the parcel items equal the parcels of the field.
 *
 * Usage: `bun sweep.ts --out <existing directory> [--shard i/n] [--category <folder substring>] [--asset <slug>] [--environments asset,office-neutral,summer-hot-sun | all]`.
 * The script never creates directories (bun crashes on recursive mkdir with emoji paths): the caller creates `--out`. Shard-safe: assets are dealt round robin to the shards by their index in the sorted catalog and every shard writes its own
 * `sweep-<shard>.jsonl` (one line per asset, appended as it goes, so a crash keeps the finished assets) and prints `[FINDING]`, `[AIRBORNE]` and `[DEBUG]` lines.
 */
import { appendFileSync, existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

function findRoot(): string {
  let dir = import.meta.dirname;
  while (!existsSync(join(dir, ".git"))) {
    const parent = dirname(dir);
    if (parent === dir) throw new Error("repository root not found");
    dir = parent;
  }
  return dir;
}

const ROOT = findRoot();
const L = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage");
const load = async (path: string): Promise<any> => import(`file:///${join(L, path).replaceAll("\\", "/")}`);

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const outDir = flag("--out");
if (outDir === undefined || !existsSync(outDir)) throw new Error("--out must name an existing directory (the caller creates it)");
const [shardIndex, shardCount] = (flag("--shard") ?? "0/1").split("/").map(Number) as [number, number];
const categoryFilter = flag("--category");
const assetFilter = flag("--asset");
const environmentsFlag = flag("--environments") ?? "asset,office-neutral,summer-hot-sun";

const { evaluate } = await load("🔨️modules/🧮️evaluation/🟦️.ts");
const { defaultDepictionOptions, depict } = await load("🔨️modules/🎨️depiction/🟦️.ts");
const { solveThermalAt } = await load("🔨️modules/🌡️thermal/🟦️.ts");
const { solveTransport } = await load("🔨️modules/💨️transport/🟦️.ts");
const { solveStructure } = await load("🔨️modules/🏗️structure/🟦️.ts");
const { VIEWS } = await load("🔨️modules/🔗️contracts/🟦️.ts");
const { resolveDepiction } = await load("🔨️modules/🎨️depiction/🟦️.ts");

type Asset = any;
type Scene = { widthMm: number; heightMm: number; layers: { items: any[] }[]; legends: { interpretation: { en: string } }[] };
type Viewpoint = "elevation" | "front" | "plan";
const VIEWPOINTS: readonly Viewpoint[] = ["elevation", "front", "plan"];
const CLIP_FRACTIONS = [0, 0.25, 0.5, 0.75, 0.999];
const itemsOf = (scene: Scene) => scene.layers.flatMap((l) => l.items);

const presets = (JSON.parse(readFileSync(join(L, "🗂️catalog/🌤️environments/🔣️.json"), "utf8")) as { presets: { id: string; environment: unknown }[] }).presets;
const environmentNames = environmentsFlag === "all" ? ["asset", ...presets.map((p) => p.id)] : environmentsFlag.split(",");

function catalogEntries(): { key: string; asset: Asset }[] {
  const root = join(L, "🗂️catalog");
  const out: { key: string; asset: Asset }[] = [];
  for (const category of readdirSync(root, { withFileTypes: true }).filter((e) => e.isDirectory()))
    for (const folder of readdirSync(join(root, category.name), { withFileTypes: true }).filter((e) => e.isDirectory())) {
      const file = join(root, category.name, folder.name, "🔣️.json");
      if (existsSync(file)) out.push({ key: `${category.name}/${folder.name}`, asset: (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset });
    }
  return out.sort((a, b) => a.key.localeCompare(b.key));
}

const authoredScales = (asset: Asset): number[] =>
  Object.values(asset.representations as Record<string, { status: string; denominator: number }>)
    .filter((r) => r.status === "authored")
    .map((r) => r.denominator)
    .sort((a, b) => a - b);

const regionIds = (shape: { region?: string; regions?: readonly string[] }): readonly string[] => (shape.region !== undefined ? [shape.region] : (shape.regions ?? []));
const legendKey = (scene: any): string => JSON.stringify(scene.legends.map(({ title, quantity, unit, min, max, stops, convention, range, conditions }: any) => ({ title, quantity, unit, min, max, stops, convention, range, conditions })));

function airborne(asset: Asset, regionId: string): string | undefined {
  const region = asset.thermal.regions.find((r: any) => r.id === regionId);
  const note = region?.hidden;
  if (note === undefined || !/air-?borne/iu.test(note.en) || !/luftgetragen/iu.test(note.de)) return undefined;
  return asset.thermal.emitters.find((e: any) => e.region === regionId)?.id;
}

type Instant = { name: string; query: any };

function instantsOf(asset: Asset): Instant[] {
  const list: Instant[] = [{ name: `pose ${asset.presentation.defaultPose}`, query: { kind: "pose", pose: asset.presentation.defaultPose } }];
  for (const clip of asset.clips) for (const f of CLIP_FRACTIONS) list.push({ name: `clip ${clip.id} @${(clip.durationS * f).toFixed(2)}s`, query: { kind: "clip", clip: clip.id, timeS: clip.durationS * f } });
  const poses: string[] = asset.poses.map((p: any) => p.id);
  const pairs: [string, string][] = poses.length >= 2 ? [[poses[0]!, poses[1]!], [poses[poses.length - 1]!, poses[0]!]] : [];
  for (const [from, to] of pairs) for (const f of [0, 0.5, 1]) list.push({ name: `transition ${from}>${to} @${f}`, query: { kind: "transition", from, to, durationS: 1, timeS: f } });
  return list;
}

type Tally = { renders: number };

function unsuitableReason(asset: Asset, denominator: number): string | undefined {
  try {
    resolveDepiction(asset, defaultDepictionOptions("light-solid", "en", denominator, "elevation"));
    return undefined;
  } catch (error) {
    return (error as Error).message;
  }
}

function colourCheck(scenes: readonly Scene[], label: string, add: (message: string) => void): void {
  const colourOfValue = new Map<number, { colour: string; pattern: string | undefined; viewpoint: Viewpoint }>();
  scenes.forEach((scene, index) => {
    for (const item of itemsOf(scene)) {
      if (item.kind !== "path" || item.value === undefined) continue;
      const colour = item.fill?.color ?? item.stroke?.color;
      if (colour === undefined) continue;
      const pattern = item.fill?.pattern;
      const known = colourOfValue.get(item.value.value);
      if (known === undefined) colourOfValue.set(item.value.value, { colour, pattern, viewpoint: VIEWPOINTS[index]! });
      else if (known.colour !== colour || known.pattern !== pattern) add(`${label} value ${item.value.value} is ${known.colour}/${known.pattern} in the ${known.viewpoint} view and ${colour}/${pattern} in the ${VIEWPOINTS[index]} view`);
    }
  });
}

const QUANTITY_IDS = ["surface-temperature", "skin-temperature", "emitted-radiant-flux", "net-radiative-flux", "sensible-heat-flux", "latent-heat-flux", "moisture-flux"] as const;

function quantityStage(asset: Asset, state: any, query: any, environment: unknown, scenes: Record<Viewpoint, Record<string, Scene>>, label: string, add: (message: string) => void, tally: Tally): void {
  if (query.kind === "clip" && (query.timeS ?? 0) === 0) return;
  const range = { mode: "shared" } as const;
  for (const quantity of QUANTITY_IDS)
    for (const mode of [range, { mode: "auto" } as const]) {
      const rendered = VIEWPOINTS.map((viewpoint) => {
        try {
          const options = { ...defaultDepictionOptions("thermal", "en", state.scale?.denominator ?? scenes[viewpoint]!["thermal"]!.metadata?.scale?.denominator ?? 100, viewpoint, "light"), environment, thermalQuantity: quantity, range: mode };
          tally.renders++;
          return depict(state, options, { thermal: solveThermalAt(asset, query, environment) });
        } catch (error) {
          add(`${label} ${viewpoint} thermal ${quantity}/${mode.mode} fails: ${(error as Error).message}`);
          return undefined;
        }
      });
      if (rendered.some((s) => s === undefined)) continue;
      const reference = legendKey(rendered[0]);
      rendered.forEach((scene, index) => {
        if (legendKey(scene) !== reference) add(`${label} thermal ${quantity}/${mode.mode}: ${VIEWPOINTS[index]} legend differs from the elevation legend`);
      });
      colourCheck(rendered as Scene[], `${label} ${quantity}/${mode.mode}`, add);
    }
}

function checkInstant(asset: Asset, key: string, instant: Instant, add: (message: string) => void, airborneSeen: Set<string>, tally: Tally, scales: readonly number[]): void {
  const query = instant.query;
  for (const environmentName of environmentNames) {
    const environment = environmentName === "asset" ? asset.thermal.environment : presets.find((p) => p.id === environmentName)?.environment;
    if (environment === undefined) {
      add(`unknown environment ${environmentName}`);
      continue;
    }
    const where = `${instant.name} [${environmentName}]`;
    let field: any;
    let transport: any;
    try {
      field = solveThermalAt(asset, query, environment);
      transport = solveTransport(asset, query, environment, { maxAgeS: 2 });
    } catch (error) {
      add(`${where} analysis throws: ${(error as Error).message}`);
      continue;
    }
    for (const denominator of scales) {
      const label = `1:${denominator} ${where}`;
      let state: any;
      try {
        state = evaluate(asset, query, { kind: "scale", scale: { denominator } });
      } catch (error) {
        add(`${label} evaluate throws: ${(error as Error).message}`);
        continue;
      }
      let structure: any;
      const structureOf = (): any => (structure ??= solveStructure(state));
      const scenes = {} as Record<Viewpoint, Record<string, Scene>>;
      for (const viewpoint of VIEWPOINTS) {
        scenes[viewpoint] = {};
        for (const view of VIEWS as string[]) {
          const options = { ...defaultDepictionOptions(view, "en", denominator, viewpoint, "light"), environment, ...(view === "light-solid" ? { transport: ["parcels", "pathlines", "streamlines", "condensation"] } : {}) };
          try {
            const analysis = view === "thermal" ? { thermal: field } : view === "light-solid" ? { transport } : view === "forces" || view === "moments" ? { structure: structureOf() } : {};
            scenes[viewpoint]![view] = depict(state, options, analysis);
            tally.renders++;
          } catch (error) {
            add(`${label} ${viewpoint} ${view} fails: ${(error as Error).message}`);
          }
        }
      }
      const thermal = VIEWPOINTS.map((v) => scenes[v]!["thermal"]);
      if (thermal.some((s) => s === undefined)) continue;
      colourCheck(thermal as Scene[], label, add);
      quantityStage(asset, state, query, environment, scenes, label, add, tally);
      const reference = legendKey(thermal[0]);
      const valuesByView = VIEWPOINTS.map((viewpoint, index) => {
        const drawn = new Set<string>();
        for (const shape of state.shapes[viewpoint]) for (const id of regionIds(shape.shape)) drawn.add(id);
        const values = new Map<string, number[]>();
        for (const item of itemsOf(thermal[index]!)) {
          if (item.kind !== "path" || item.value === undefined || item.region === undefined || item.id.startsWith("halo:")) continue;
          values.set(item.region, [...(values.get(item.region) ?? []), item.value.value]);
        }
        if (legendKey(thermal[index]) !== reference) add(`${label} ${viewpoint} thermal legend differs from the elevation legend`);
        const note = thermal[index]!.legends[0]?.interpretation.en ?? "";
        for (const region of asset.thermal.regions) {
          if (field.regions[region.id] === undefined || drawn.has(region.id)) continue;
          if (!note.includes(region.title.en)) add(`${label} ${viewpoint} region ${region.id} is not drawn and not named in the legend`);
        }
        return values;
      });
      for (const region of asset.thermal.regions) {
        const solved = field.regions[region.id]?.surfaceTempC;
        const all = valuesByView.flatMap((m, i) => (m.get(region.id) ?? []).map((value) => ({ value, viewpoint: VIEWPOINTS[i]! })));
        for (const { value, viewpoint } of all) if (solved !== undefined && Math.abs(value - solved) > 1e-9) add(`${label} region ${region.id} is ${value} in the ${viewpoint} view but ${solved} in the field`);
        if (all.length > 1 && Math.max(...all.map((a) => a.value)) - Math.min(...all.map((a) => a.value)) > 1e-9) add(`${label} region ${region.id} has different values in different views`);
        const anywhere = VIEWPOINTS.some((v) => state.shapes[v].some((s: any) => regionIds(s.shape).includes(region.id)));
        if (field.regions[region.id] === undefined || anywhere) continue;
        const tracer = airborne(asset, region.id);
        if (tracer !== undefined) airborneSeen.add(`${region.id} air-borne (traced by ${tracer})`);
        else if (typeof region.hidden?.en === "string" && region.hidden.en.length > 0 && typeof region.hidden?.de === "string" && region.hidden.de.length > 0) airborneSeen.add(`${region.id} hidden (D40)`);
        else add(`${label} region ${region.id} has no shape in any view and no hidden note`);
      }
      for (const viewpoint of VIEWPOINTS) {
        const scene = scenes[viewpoint]!["light-solid"];
        if (scene === undefined) continue;
        const parcels = itemsOf(scene).filter((i) => i.id.startsWith("parcel:")).length;
        if (parcels !== transport.parcels.length) add(`${label} ${viewpoint} draws ${parcels} of ${transport.parcels.length} parcels`);
        for (const emitter of transport.emitters) {
          const marker = itemsOf(scene).find((i) => i.kind === "path" && i.id === `emitter:${emitter.emitter}`);
          if (marker === undefined) {
            add(`${label} ${viewpoint} emitter ${emitter.emitter} has no marker`);
            continue;
          }
          const xs = marker.commands.flatMap((c: any) => (c.op === "Z" ? [] : [c.x]));
          const ys = marker.commands.flatMap((c: any) => (c.op === "Z" ? [] : [c.y]));
          if (Math.min(...xs) < -1e-6 || Math.max(...xs) > scene.widthMm + 1e-6 || Math.min(...ys) < -1e-6 || Math.max(...ys) > scene.heightMm + 1e-6) add(`${label} ${viewpoint} emitter ${emitter.emitter} lies outside the paper (${scene.widthMm.toFixed(1)} x ${scene.heightMm.toFixed(1)} mm)`);
        }
      }
    }
  }
}

const started = Date.now();
const shardFile = join(outDir, `sweep-${shardIndex}of${shardCount}.jsonl`);
writeFileSync(shardFile, "");
let assets = 0;
let instants = 0;
let findings = 0;
const tally: Tally = { renders: 0 };
catalogEntries().forEach(({ key, asset }, index) => {
  if (index % shardCount !== shardIndex) return;
  if (categoryFilter !== undefined && !key.includes(categoryFilter)) return;
  if (assetFilter !== undefined && !key.endsWith(`/${assetFilter}`)) return;
  assets++;
  const list = new Set<string>();
  const seen = new Set<string>();
  const add = (message: string): void => void list.add(message);
  const drawable = authoredScales(asset).filter((d) => {
    const reason = unsuitableReason(asset, d);
    if (reason !== undefined) add(`unsuitable authored scale 1:${d}: ${reason}`);
    return reason === undefined;
  });
  if (authoredScales(asset).length === 0) add("no authored scale");
  else
    for (const instant of instantsOf(asset)) {
      instants++;
      try {
        checkInstant(asset, key, instant, add, seen, tally, drawable);
      } catch (error) {
        add(`${instant.name} throws: ${(error as Error).message}`);
      }
    }
  findings += list.size;
  appendFileSync(shardFile, `${JSON.stringify({ key, scales: authoredScales(asset), findings: [...list], airborne: [...seen] })}\n`);
  for (const message of list) console.log(`[FINDING] ${key} | ${message}`);
  for (const message of seen) console.log(`[NOTED] ${key} | ${message}`);
  console.error(`[DEBUG] ${key}: ${list.size} findings, ${((Date.now() - started) / 1000).toFixed(0)} s`);
});
appendFileSync(shardFile, `${JSON.stringify({ total: { shard: `${shardIndex}/${shardCount}`, assets, instants, renders: tally.renders, findings, environments: environmentNames, seconds: (Date.now() - started) / 1000 } })}\n`);
console.log(`shard ${shardIndex}/${shardCount}: ${assets} assets, ${instants} instants, ${tally.renders} renders, ${findings} findings, ${((Date.now() - started) / 1000).toFixed(0)} s -> ${shardFile}`);
