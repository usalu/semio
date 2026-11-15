/** 🧵 Author 47: after `kit-structure.ts --write`, replaces members of blade, culm and plume bones by a thin core (midrib / culm / rachis) that follows the centre path of that bone's own body piece (the blade-1 piece for a blade bone), ends trimmed by 8 %. Usage: `bun author-47-struct-fix.ts <asset> --scale N [--r 0.0045] [--slender]`. */
import { openCtx, saveLists, flag, has, type Json } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
const r = Number(flag(a, "--r", String(0.6 * c.feature)));
const bones: Json[] = c.raw.rig.bones;
const body: Json[] = c.vol.body;
const start: Record<string, number[]> = {};
const out = c.vol.structure.map((s: Json) => {
  if (s.part === "joint") return s;
  if (s.part === "support" && s.kind === "box") { const t = s.size[1] as number; return { ...s, size: [t, s.size[0], s.size[2]], transform: { translation: [-t / 2, 0, 0] } }; }
  if (s.part === "member" && s.bone === "root-plate") return { ...s, sections: [{ at: [0.004, 0, 0], rx: 0.17, ry: 0.17, exponent: 2 }, { at: [0.12, 0, 0], rx: 0.09, ry: 0.09, exponent: 2 }] };
  if (s.part !== "member") return s;
  const mine = body.filter((b) => b.bone === s.bone && b.kind === "loft");
  const piece = mine.find((b) => /-1[ab]$/.test(b.id)) ?? mine[0];
  const rr = Math.max(r, 0.55 * c.feature);
  let at: number[][];
  if (piece === undefined) {
    const L: number = bones.find((b) => b.id === s.bone)!.lengthM;
    at = [[0.05 * L, 0, 0], [0.95 * L, 0, 0]];
  } else {
    const p: number[][] = piece.sections.map((q: Json) => q.at);
    const sorted = p;
    const n = sorted.length;
    const lerp = (t: number): number[] => {
      const x = t * (n - 1);
      const i = Math.min(n - 2, Math.floor(x));
      const f = x - i;
      return sorted[i]!.map((v, k) => v + (sorted[i + 1]![k]! - v) * f);
    };
    at = [0.08, 0.3, 0.5, 0.7, 0.92].map(lerp);
    const e0 = lerp(0.04);
    const e1 = lerp(0.96);
    start[s.bone] = Math.hypot(...e0) <= Math.hypot(...e1) ? e0 : e1;
  }
  return { ...s, sections: at.map((q) => ({ at: q, rx: rr, ry: rr, exponent: 2 })), ...(has(a, "--slender") ? { slender: true } : {}) };
});
const fixed = out.map((s: Json) => (s.part === "joint" && start[s.bone] !== undefined ? { ...s, radii: [Math.max(r, 0.55 * c.feature), Math.max(r, 0.55 * c.feature), Math.max(r, 0.55 * c.feature)], transform: { translation: start[s.bone] } } : s));
saveLists(c, { structure: fixed });
