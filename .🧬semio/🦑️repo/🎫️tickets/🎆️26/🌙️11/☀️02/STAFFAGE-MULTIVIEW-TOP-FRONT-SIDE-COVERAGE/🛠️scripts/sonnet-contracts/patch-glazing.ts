import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const anchor = "      need(`${sp}.region`, shape.region, regions, \"Thermal region\");";
if (!t.includes(anchor)) throw new Error("anchor");
t = t.replace(anchor, "      if (shape.role === \"glazing\" && shape.hatch !== undefined) add(sp, \"glazing-hatch\", `Glazing shape \"${shape.id}\" names the hatch \"${shape.hatch}\"; glazing is transparent, never occludes and is never hatched.`, `Verglasungsform \"${shape.id}\" nennt die Schraffur \"${shape.hatch}\"; Verglasung ist durchsichtig, verdeckt nie und wird nie schraffiert.`);\n" + anchor);
writeFileSync(file, t);
