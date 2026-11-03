import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const lines = readFileSync(file, "utf8").split("\n");
const at = lines.findIndex((l) => l.includes('add(sp, "lateral-ambiguous"'));
const head = lines[at]!;
const cut = head.indexOf(") add(sp, \"lateral-ambiguous\"");
lines[at] = head.slice(0, cut + 1) + " (ambiguous.get(shape.bone) ?? ambiguous.set(shape.bone, []).get(shape.bone)!).push(shape.id);";
const start = lines.findIndex((l) => l.startsWith("  const checkShapes = ("));
lines.splice(start + 1, 0, "    const ambiguous = new Map<Slug, string[]>();");
const end = lines.findIndex((l, i) => i > at && l === "    });" && lines[i + 1] === "  };");
lines.splice(end + 1, 0,
  "    for (const [bone, ids] of ambiguous) add(path, \"lateral-ambiguous\", `${ids.length} shape(s) (${ids.slice(0, 6).join(\", \")}${ids.length > 6 ? \", ...\" : \"\"}) sit on bone \"${bone}\", which is spread or twisted in a pose or clip (or rides on a bone that is): their projection under that rotation is undefined unless they are solids (extruded or revolved).`, `${ids.length} Form(en) (${ids.slice(0, 6).join(\", \")}${ids.length > 6 ? \", ...\" : \"\"}) sitzen am Knochen \"${bone}\", der in einer Pose oder einem Clip gespreizt oder verdreht wird (oder an einem solchen Knochen hängt): ihre Projektion unter dieser Drehung ist nur als Körper (extrudiert oder rotiert, solid) eindeutig.`);");
writeFileSync(file, lines.join("\n"));
