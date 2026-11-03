import { fileOf, patchView } from "./patch.ts";

const f = fileOf("🛋️furniture", "office-swivel-chair");
const ext = (z0: number, z1: number) => ({ kind: "extruded", z0, z1 });
const solidOf: Record<string, [number, number]> = {
  "seat-shell": [-0.23, 0.23],
  "seat-piping": [-0.17, 0.17],
  "seat-fabric-fold": [-0.2, 0.2],
  "seat-lever": [-0.14, -0.12],
  "backrest-pad": [-0.2, 0.2],
  "backrest-rim": [-0.22, 0.22],
  "backrest-texture": [-0.2, 0.2],
  "backrest-strut": [-0.011, 0.011],
};
const pair = (s: any, id: string, z: [number, number]) => [
  { ...s, id: `${id}-1`, solid: ext(z[0], z[1]) },
  { ...s, id: `${id}--1`, solid: ext(-z[1], -z[0]) },
];
const elevationFix = (shapes: any[], armrestId: string | null): any[] =>
  shapes.flatMap((s) => {
    if (s.solid === undefined && solidOf[s.id] !== undefined) return [{ ...s, solid: ext(...solidOf[s.id]!) }];
    if (armrestId !== null && s.id === armrestId && s.solid === undefined) return pair(s, armrestId, [0.26, 0.28]);
    if (s.id === "armrest-line" && s.solid === undefined) return pair(s, "armrest-line", [0.265, 0.275]);
    return [s];
  });

const twins = new Map<string, any>();
const remember = (shapes: any[]): any[] => {
  for (const s of shapes) twins.set(s.id, s);
  return shapes;
};
const frontFromElevation = (shapes: any[], ids: string[]): any[] =>
  shapes.map((s) => {
    if (!ids.includes(s.id)) return s;
    const e = twins.get(s.id);
    if (e === undefined) throw new Error(`no elevation twin for ${s.id}`);
    return { ...e, z: s.z, id: s.id };
  });

patchView(f, 50, "elevation", (shapes) => remember(elevationFix(shapes, null)));
patchView(f, 50, "front", (shapes) => {
  twins.set("seat-piping-ring", { ...twins.get("seat-piping") });
  return frontFromElevation(shapes, ["seat-piping-ring"]);
});
patchView(f, 100, "elevation", (shapes) => remember(elevationFix(shapes, "armrest-post")));
patchView(f, 100, "front", (shapes) => frontFromElevation(shapes, ["backrest-strut", "armrest-post-1", "armrest-post--1"]));
patchView(f, 200, "elevation", (shapes) => remember(elevationFix(shapes, "armrest")));
patchView(f, 200, "front", (shapes) => frontFromElevation(shapes, ["backrest-strut", "armrest-1", "armrest--1"]));
