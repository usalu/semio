/** 📤️ Dumps the pose frames, dimensions and the closed filled drawing shapes (world rings per view) of every authored scale of one asset to 🗑️generated/author-44/<slug>.dump.json. Usage: bun author-44-dump.ts <category>/<slug> */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED, entryOf, poseState, VIEWS } from "./volumes-lib.ts";

const spec = process.argv[2]!;
const { entry, asset } = entryOf(spec);
const AUTH = new Set(["body", "detail", "glazing"]);
const out: any = { dimensions: asset.dimensions, frames: {}, pose: asset.presentation.defaultPose, scales: {} };
for (const d of [50, 100, 200, 500]) {
  if (asset.representations[String(d)]?.status !== "authored") continue;
  const st = poseState(asset, d);
  out.frames = st.frames;
  const views: any = {};
  for (const view of VIEWS) {
    views[view] = st[view]
      .filter((s: any) => s.closed && AUTH.has(s.shape.role) && s.rings.length > 0)
      .map((s: any) => ({ id: s.shape.id, bone: s.shape.bone, role: s.shape.role, rings: s.rings.map((r: any[]) => r.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)])) }));
  }
  out.scales[d] = views;
}
const dir = join(GENERATED, "author-44");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, `${entry.slug}.dump.json`), JSON.stringify(out));
process.stdout.write(`dumped ${entry.slug}\n`);
