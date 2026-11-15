import { writeFileSync } from "node:fs";
import { entryOf, frames, restState } from "./volumes-lib.ts";

const [spec, scaleArg] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const scale = Number(scaleArg);
const st = restState(asset, scale);
const pick = (list: any[]) =>
  list.map((e) => ({
    id: e.shape.id,
    bone: e.shape.bone,
    role: e.shape.role ?? "body",
    region: e.shape.region,
    filled: e.shape.filled !== false,
    closed: e.closed,
    kind: e.shape.kind,
    z: e.shape.z ?? 0,
    rings: e.rings.map((r: any[]) => r.map((p) => [Math.round(p.x * 1e5) / 1e5, Math.round(p.y * 1e5) / 1e5])),
  }));
const f = frames(asset);
const out = { bones: f, elevation: pick(st.elevation), front: pick(st.front), plan: pick(st.plan) };
writeFileSync(`🗑️generated/author-08/${entry.slug}.p${scale}.json`, JSON.stringify(out));
