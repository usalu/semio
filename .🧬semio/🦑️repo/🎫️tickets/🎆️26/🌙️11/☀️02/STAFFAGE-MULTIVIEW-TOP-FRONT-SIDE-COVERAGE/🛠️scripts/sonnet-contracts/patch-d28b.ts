import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const lines = readFileSync(file, "utf8").split("\n");
const find = (needle: string, from = 0): number => {
  const i = lines.findIndex((l, n) => n >= from && l.includes(needle));
  if (i < 0) throw new Error(`missing ${needle}`);
  return i;
};
const decl = find("    const ambiguous = new Map<Slug, { ids: string[]; thetaDeg: number; errorMm: number }>();");
lines.splice(decl, 1);
const check = find("  const checkShapes = (path: string");
lines.splice(check, 0, "  const ambiguous = new Map<Slug, { ids: Set<string>; where: Set<string>; thetaDeg: number; errorMm: number }>();");
const entry = find("ambiguous.set(shape.bone, { ids: [], thetaDeg: 0, errorMm: 0 })");
lines[entry] = lines[entry]!.replace("{ ids: [], thetaDeg: 0, errorMm: 0 }", "{ ids: new Set(), where: new Set(), thetaDeg: 0, errorMm: 0 }");
const idsPush = find("entry.ids.push(shape.id);");
lines[idsPush] = "          entry.ids.add(shape.id);\n          entry.where.add(`1:${denominator} ${viewpoint}`);";
const emit = find("    for (const [bone, { ids, thetaDeg, errorMm }] of ambiguous) add(path,");
lines.splice(emit, 1);
const after = find("  for (const { path, denominator, paperMm, offered } of drawable) {");
lines.splice(after, 0,
  "  for (const [bone, { ids, where, thetaDeg, errorMm }] of ambiguous) add(\"representations\", \"lateral-ambiguous\", `${ids.size} flat shape(s) (${[...ids].slice(0, 6).join(\", \")}${ids.size > 6 ? \", ...\" : \"\"}) on bone \"${bone}\" would be drawn up to ${errorMm.toFixed(2)} mm wrong on paper (above the ${DEFAULT_PRINT_PROFILE.minFeatureMm} mm print minimum) in ${[...where].join(\", \")}: the bone leaves the shapes' plane by up to ${thetaDeg.toFixed(1)} degrees in a pose or clip, so author them as solids (extruded or revolved).`, `${ids.size} flache Form(en) (${[...ids].slice(0, 6).join(\", \")}${ids.size > 6 ? \", ...\" : \"\"}) am Knochen \"${bone}\" würden auf dem Papier um bis zu ${errorMm.toFixed(2)} mm falsch gezeichnet (über dem Druckminimum von ${DEFAULT_PRINT_PROFILE.minFeatureMm} mm) in ${[...where].join(\", \")}: der Knochen verlässt die Ebene der Formen in einer Pose oder einem Clip um bis zu ${thetaDeg.toFixed(1)} Grad, daher als Körper (extrudiert oder rotiert, solid) zeichnen.`);",
);
writeFileSync(file, lines.join("\n"));
