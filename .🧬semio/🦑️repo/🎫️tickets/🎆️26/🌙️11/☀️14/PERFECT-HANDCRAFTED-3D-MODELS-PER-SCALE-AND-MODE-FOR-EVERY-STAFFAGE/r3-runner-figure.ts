/** 🏗️ The round 3 runner with blade prosthesis: assembles body, thermal model and load path of every scale and writes `volumes/humans/runner-with-blade-prosthesis.json`.
 * Usage: `bun r3-runner-figure.ts`
 * @see ./r3-runner-body50.ts
 * @see ./r3-humans-runner-with-blade-prosthesis.md */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { Kit, parts, thermalOf, type Json, type V3 } from "./r3-runner-kit.ts";
import { M, body50 } from "./r3-runner-body50.ts";
import { OUT, ASSET_FILE } from "./r3-runner-roots.ts";

export const SCALES = [50, 100, 200, 500] as const;
export type Built = { volume: Json; kit: Kit };
const optional = async (file: string): Promise<Json> => (existsSync(join(import.meta.dir, file)) ? await import(`./${file}`) : {});
const coarse = await optional("r3-runner-coarse.ts");
const lists = await optional("r3-runner-lists.ts");

function thermalDraft(asset: Json, k: Kit, scale: number): Json[] {
  const regions = new Map<string, Json>(asset.thermal.regions.map((r: Json) => [r.id, r]));
  const out = k.solids.flatMap((s) => thermalOf(s, regions.get(k.region.get(s.id)!)!, scale === 50));
  const t = new Kit();
  const e = (id: string, at: V3, radii: V3, region: string, emitter: string): Json => ({ ...t.ell(id, "head", M.skin, 0.3, at, radii), region, tags: ["emitter", emitter] });
  out.push(e("th-nose-breath", [-0.012, -0.118, 0], [0.004, 0.004, 0.008], "face-skin", "nose-breath"), e("th-mouth-breath", [-0.042, -0.106, 0], [0.004, 0.004, 0.0105], "face-skin", "mouth-breath"), e("th-plume", [0.155, -0.008, 0], [0.004, 0.05, 0.042], "singlet", "body-plume"));
  return out;
}

/** 🏗️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): Built {
  const kit = new Kit();
  (scale === 50 || coarse.BODY === undefined ? body50 : coarse.BODY[scale])(kit);
  const thermal = lists.thermal === undefined ? thermalDraft(asset, kit, scale) : lists.thermal(asset, kit, scale);
  const structure = lists.structure === undefined ? parts([new Kit().cap("draft-spine", "torso", M.skin, 0.8, [0, 0.03, 0], [0.5, 0.03, 0], 0.02, 0.02)], "torso", "member") : lists.structure(asset, kit, scale);
  const decisions = lists.DECISIONS === undefined ? [{ en: "Draft.", de: "Entwurf." }] : lists.DECISIONS[scale];
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal, structure, decisions } };
}

/** 📄️ The asset as the candidate document states it (corrected rig, poses and anchors), else the catalog asset. */
export function loadAsset(): Json {
  const candidate = join(OUT, "🔣️.json");
  return JSON.parse(readFileSync(existsSync(candidate) ? candidate : ASSET_FILE, "utf8")).asset as Json;
}

if (import.meta.main) {
  const asset = loadAsset();
  const volumes = Object.fromEntries(SCALES.map((s) => [String(s), build(asset, s).volume]));
  writeFileSync(join(import.meta.dir, "volumes", "humans", "runner-with-blade-prosthesis.json"), `${JSON.stringify({ volumes })}\n`);
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
