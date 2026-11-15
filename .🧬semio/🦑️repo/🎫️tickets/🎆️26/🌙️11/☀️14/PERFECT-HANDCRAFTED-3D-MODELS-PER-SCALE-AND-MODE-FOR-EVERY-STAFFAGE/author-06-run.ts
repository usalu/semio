import { Ctx, auto, buildStructure, buildThermal, decisions, emitters, fillMissing, finish, type Auto, type Note, type Solid, type StructCfg, type ScaleOut } from "./author-06-lib.ts";
import { loadSource, entryOf } from "./volumes-lib.ts";

export type Cfg = {
  emit: Record<string, string>;
  fixY?: Record<string, number>;
  auto?: (scale: number) => Auto;
  struct?: (scale: number) => StructCfg;
  notes: Record<number, Note[]>;
  extraBody?: (c: Ctx) => void;
  extraThermal?: (c: Ctx) => Solid[];
  thermalSkip?: (c: Ctx) => Set<string>;
  more?: (c: Ctx, thermal: Solid[], structure: Solid[]) => void;
  extraNote?: Record<number, { thermal?: Note; structure?: Note }>;
};

export async function run(slug: string, cfg: Cfg): Promise<void> {
  const { entry } = entryOf(`animals/${slug}`);
  const src = loadSource(entry)!;
  const out: Record<number, ScaleOut> = {};
  for (const scale of [50, 100, 200, 500]) {
    if (src.volumes[String(scale)]?.status !== "authored") continue;
    const c = new Ctx(slug, scale);
    auto(c, cfg.auto?.(scale) ?? {});
    cfg.extraBody?.(c);
    const extra = [...emitters(c, cfg.emit, cfg.fixY), ...(cfg.extraThermal?.(c) ?? [])];
    const thermal = buildThermal(c, extra, cfg.thermalSkip?.(c));
    const filled = fillMissing(c, thermal);
    if (filled.length > 0) console.log(`${slug} 1:${scale} filled regions: ${filled.join(",")}`);
    const structure = buildStructure(c, cfg.struct?.(scale) ?? {});
    cfg.more?.(c, thermal, structure);
    out[scale] = { body: c.body, thermal, structure, decisions: decisions(c, cfg.notes[scale] ?? [{ en: "Body modelled from the drawings of this scale.", de: "Körper aus den Zeichnungen dieses Massstabs modelliert." }], thermal, structure, cfg.extraNote?.[scale]) };
  }
  finish(slug, out);
}
