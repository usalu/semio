/** 🖨️ Prints why the product calls a scale of a candidate document of the stroller asset unsuitable for print: the violations of the default print profile per shape, per pose and viewpoint (one pose and one viewpoint's geometry at a time).
 * Usage: `bun r3-stroller-suit.ts <asset document json> [scale ...]` */
import { defaultDepictionOptions } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";

const asset: any = loadAsset(process.argv[2]!);
for (const scale of process.argv.slice(3).map(Number)) {
  const o: any = defaultDepictionOptions("light-line" as never, "en", scale, "elevation" as never);
  const profile = o.profile ?? o.print ?? o.printProfile;
  if (profile === undefined) { console.log(Object.keys(o)); break; }
  for (const pose of asset.poses.filter((p: any) => !p.key)) for (const view of ["elevation", "front", "plan"]) {
    const rep = asset.representations[String(scale)];
    const one = { ...asset, poses: [pose], representations: { ...asset.representations, [String(scale)]: { ...rep, geometry: { elevation: rep.geometry.elevation, front: view === "front" ? rep.geometry.front : [], plan: view === "plan" ? rep.geometry.plan : [], ...(view === "elevation" ? {} : {}) } } } };
    const r: any = resolveRepresentation(one, { denominator: scale } as never, profile);
    if (view !== "elevation" || r.kind === "unsuitable") console.log(scale, pose.id, view, r.kind, JSON.stringify((r.suitability?.violations ?? []).map((v: any) => [v.check, v.shapes.join("+"), v.measuredMm])));
  }
}
