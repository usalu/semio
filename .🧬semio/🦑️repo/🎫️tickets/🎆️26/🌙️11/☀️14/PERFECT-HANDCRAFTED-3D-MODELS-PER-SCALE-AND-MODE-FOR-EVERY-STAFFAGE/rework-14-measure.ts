/** 📏️ Author 14 ruler: world bounding box of every solid of one list of an asset at a scale and pose, plus the rig table.
 *
 * @see ./rework-brief.md */
import { cli, load, rigAt, queryOf, listsIn } from "./r3-lib.ts";

const { spec, flag, has } = cli(["--scale", "--list", "--pose", "--clip", "--t"]);
const { asset } = load(spec, "rework-14-measure.ts <asset> --scale N [--list body] [--pose ID] [--rig]");
const scale = Number(flag("--scale") ?? 50);
const list = (flag("--list") ?? "body") as "body" | "thermal" | "structure";
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), flag("--clip"), flag("--t")));
const f = (n: number) => n.toFixed(3).padStart(7);
if (has("--rig")) {
  for (const b of asset.rig.bones) {
    const r = rig.bones.find((x) => x.id === b.id)!;
    console.log(`${b.id.padEnd(22)} parent ${(b.parent ?? "-").padEnd(16)} o ${r.o.map(f).join("")} tip ${r.tip.map(f).join("")} len ${f(b.lengthM)} ${JSON.stringify(b.joint)}`);
  }
}
const lists = listsIn(asset, scale, rig.frames);
const all = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
lists[list].forEach((m, i) => {
  const s = lists.solids[list][i]!;
  const bb = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (let k = 0; k < m.world.length; k += 3) for (let a = 0; a < 3; a++) {
    bb[a] = Math.min(bb[a]!, m.world[k + a]!); bb[a + 3] = Math.max(bb[a + 3]!, m.world[k + a]!);
    all[a] = Math.min(all[a]!, m.world[k + a]!); all[a + 3] = Math.max(all[a + 3]!, m.world[k + a]!);
  }
  console.log(`${String(s.id).padEnd(30)} ${String(s.bone).padEnd(16)} ${String(s.kind).padEnd(9)} x ${f(bb[0]!)}${f(bb[3]!)} y ${f(bb[1]!)}${f(bb[4]!)} z ${f(bb[2]!)}${f(bb[5]!)} ${s.material ?? ""} ${s.region ?? ""}${s.layer ? "/" + s.layer : ""} ${s.member ?? ""} ${s.part ?? ""}`);
});
console.log(`ALL x ${f(all[0]!)}${f(all[3]!)} y ${f(all[1]!)}${f(all[4]!)} z ${f(all[2]!)}${f(all[5]!)}  errors ${lists.errors.length}`);
