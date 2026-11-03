import { readFileSync, writeFileSync } from "node:fs";
const dir = process.argv[2]!;
const file = `${dir}/🔣️.json`;
const doc = JSON.parse(readFileSync(file, "utf8")) as { $defs: Record<string, any> };
const defs = doc.$defs;
defs.Solid.oneOf = defs.Solid.oneOf.filter((alternative: any) => alternative.properties.kind.const !== "member");
defs.Solid.description = (defs.Solid.description as string).replace(/ member: a prismatic bar[^]*$/u, "");
delete defs.BonePoint;
defs.MemberEnd = {
  description: "An end point of a member: bone-local metres from the origin of its bone at rest.",
  type: "object",
  additionalProperties: false,
  required: ["x", "y", "z"],
  properties: {
    bone: { description: "The bone whose frame the point is given in; default the shape's own bone. It is the shape's own bone or one of its ancestors (validator `member-end-not-ancestor`).", $ref: "#/$defs/Slug" },
    x: { description: "Metres along the bone (u) from its origin at rest.", type: "number" },
    y: { description: "Metres along the bone normal (v) from its origin at rest.", type: "number" },
    z: { description: "Metres laterally (w, left is +) from its origin at rest; required, never defaulted.", type: "number" },
  },
};
defs.MemberSection = {
  description: "The physical section of a member: a rectangle of width w (along bar x bone-w) and height h (completing the orthonormal frame), or a circle of diameter d, all in metres.",
  oneOf: [
    { type: "object", additionalProperties: false, required: ["kind", "w", "h"], properties: { kind: { const: "rect" }, w: { type: "number", exclusiveMinimum: 0 }, h: { type: "number", exclusiveMinimum: 0 } } },
    { type: "object", additionalProperties: false, required: ["kind", "d"], properties: { kind: { const: "circle" }, d: { type: "number", exclusiveMinimum: 0 } } },
  ],
};
const capsule = defs.Shape.oneOf.find((alternative: any) => alternative.properties.kind.const === "capsule");
const common = Object.fromEntries(["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"].map((key) => [key, capsule.properties[key]]));
defs.Shape.oneOf = defs.Shape.oneOf.filter((alternative: any) => alternative.properties.kind.const !== "member");
defs.Shape.oneOf.push({
  description: "A member: a prismatic bar between two bone-local end points with a physical section, carried by the full 3D frames of its bones and drawn in its true foreshortened outline in every viewpoint. It has no planar geometry and no solid; a diagonal is one member and never a bounding rectangle.",
  type: "object",
  additionalProperties: false,
  required: ["kind", "id", "bone", "role", "z", "filled", "tone", "from", "to", "section"],
  properties: { kind: { const: "member" }, ...common, from: { $ref: "#/$defs/MemberEnd" }, to: { $ref: "#/$defs/MemberEnd" }, section: { $ref: "#/$defs/MemberSection" } },
});
writeFileSync(file, `${JSON.stringify(doc, null, 2)}\n`);

let ts = readFileSync(`${dir}/🟦️.ts`, "utf8");
const cut = (from: string, to: string): void => {
  if (!ts.includes(from)) throw new Error(`missing: ${from.slice(0, 70)}`);
  ts = ts.replace(from, to);
};
cut('\n  | { readonly kind: "member"; readonly from: BonePoint; readonly to: BonePoint & { readonly bone?: Slug }; readonly diameterM?: number; readonly widthM?: number; readonly depthM?: number };', ";");
ts = ts.replace(/\n \* `member`: a prismatic bar[^]*?never a bounding rectangle\. \*\//u, " */");
cut("/** 📍️ A point in the bone-local frame of a bone, metres from its origin at rest: `x` along the bone (u), `y` along its normal (v), `z` lateral (w, left is +). */\nexport type BonePoint = { readonly x: number; readonly y: number; readonly z: number };\n\n", "");
cut("/** 🔺️ One of the shape primitives. */\nexport type ShapeGeometry = CapsuleShape | EllipseShape | RectShape | PolyShape | PathShape;", "/** 📍️ An end point of a {@link MemberShape}: metres from the origin of `bone` (default the shape's own bone; the shape's own bone or one of its ancestors) at rest, `x` along the bone (u), `y` along its normal (v), `z` lateral (w, left is +); `z` is required and never defaulted. */\nexport type MemberEnd = { readonly bone?: Slug; readonly x: number; readonly y: number; readonly z: number };\n\n/** 📐️ The physical section of a member: a rectangle of width `w` (along bar x bone-w) and height `h` (completing the orthonormal frame), or a circle of diameter `d`, in metres. */\nexport type MemberSection = { readonly kind: \"rect\"; readonly w: number; readonly h: number } | { readonly kind: \"circle\"; readonly d: number };\n\n/** 🦴️ A prismatic bar between two bone-local end points with a physical section, carried by the full 3D frames of its bones and drawn in its true foreshortened outline in every viewpoint. It has no planar geometry and no `solid`: a diagonal is one member, never a bounding rectangle. */\nexport type MemberShape = { readonly kind: \"member\"; readonly from: MemberEnd; readonly to: MemberEnd; readonly section: MemberSection };\n\n/** 🔺️ One of the shape primitives. */\nexport type ShapeGeometry = CapsuleShape | EllipseShape | RectShape | PolyShape | PathShape | MemberShape;");
writeFileSync(`${dir}/🟦️.ts`, ts);
