import { load, save } from "./lib.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write";
const x = load(id); const a = x.asset; const K = 0.4;
for (const clip of a.clips) {
  if (!/sway|flutter/.test(clip.id) || !clip.loop) continue;
  const keys = clip.keys; const T = clip.durationS; const times = keys.map((k: any) => k.t);
  if (keys.some((k: any) => Object.values(k.bones ?? {}).some((b: any) => b.abductionDeg !== undefined))) { console.log("[DEBUG]", id, clip.id, "already lateral"); continue; }
  const ids = new Set<string>(); for (const k of keys) for (const b of Object.keys(k.bones ?? {})) ids.add(b);
  const val = (bone: string, i: number) => keys[i].bones?.[bone]?.angleDeg ?? 0;
  const at = (bone: string, t: number) => { const tt = ((t % T) + T) % T; let i = times.findIndex((v: number, j: number) => tt >= v && (j === times.length - 1 || tt < times[j + 1])); const t0 = times[i], t1 = i === times.length - 1 ? T : times[i + 1]; const v0 = val(bone, i), v1 = i === times.length - 1 ? val(bone, 0) : val(bone, i + 1); return v0 + (v1 - v0) * (tt - t0) / (t1 - t0); };
  for (const bone of ids) {
    const mean = times.reduce((s: number, _: number, i: number) => s + val(bone, i), 0) / times.length;
    keys.forEach((k: any, i: number) => { const lat = Math.round(K * (at(bone, times[i] - T / 4) - mean) * 100) / 100; k.bones[bone] ??= {}; k.bones[bone] = { ...k.bones[bone], abductionDeg: lat }; });
  }
  console.log("[DEBUG]", id, clip.id, "lateral crosswind added", ids.size, "bones");
}
if (write) save(x);
