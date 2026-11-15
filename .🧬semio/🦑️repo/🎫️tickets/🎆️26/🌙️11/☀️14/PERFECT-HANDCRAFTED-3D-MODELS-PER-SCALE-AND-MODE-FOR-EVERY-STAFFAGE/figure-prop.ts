/** 🛏️ Replaces one prop solid (pillow, bag, phone, duvet skirt) of a human asset by a loft cut from the scale's own drawings with the figure kit's station fitting.
 * Usage: `bun figure-prop.ts <asset> --scale N --bone bone --shapes id,id --id solid-id [--e 3] [--n 12] [--tol 0.2] [--write]`; the solid `id` is replaced in place (keeps material, tone, role); other solids are untouched.
 * @see ./figure.md */
import { drawn, flag, has, openCtx, saveLists, type Json } from "./kit-lib.ts";
import { fitStations, loftStations, personOf, rangeOf, type BoneInfo, type Group } from "./figure-lib.ts";
const args = process.argv.slice(2);
const c = openCtx(args[0]!, Number(flag(args, "--scale")));
const bone = flag(args, "--bone")!;
const ids = new Set(flag(args, "--shapes")!.split(","));
const id = flag(args, "--id")!;
const shapes: Group["shapes"] = [];
for (const view of ["elevation", "front", "plan"] as const) for (const s of drawn(c, view, bone)) if (ids.has(s.id)) shapes.push({ id: s.id, view, role: s.role, rings: s.rings });
const rig = (c.m.rig.bones as Json[]).find((b) => b.id === bone)!;
const info: BoneInfo = { id: bone, part: "torso", frame: c.state.frames[bone] as never, L: rig.lengthM, ant: 1, side: "" };
const g = { shapes };
const ends = rangeOf(info, g, ids) ?? [0, rig.lengthM];
const person = personOf({}, Object.fromEntries((c.m.rig.bones as Json[]).map((b) => [b.id, b])));
const st = fitStations(info, g, { n: Number(flag(args, "--n", "12")), feature: c.feature, tol: Number(flag(args, "--tol", "0.2")) * c.feature, landmarks: false, only: ids, person, part: "torso", ends, e: Number(flag(args, "--e", "3")) });
const old = (c.vol.body as Json[]).find((s) => s.id === id)!;
const sol = { ...loftStations({ id, bone, material: old.material, tone: old.tone, role: old.role }, st), ...(old.region ? { region: old.region } : {}) };
console.log(`${id}: ${st.length} stations, ends ${ends.map((x) => x.toFixed(3))}`);
if (has(args, "--write")) saveLists(c, { body: (c.vol.body as Json[]).map((s) => (s.id === id ? sol : s)) });
