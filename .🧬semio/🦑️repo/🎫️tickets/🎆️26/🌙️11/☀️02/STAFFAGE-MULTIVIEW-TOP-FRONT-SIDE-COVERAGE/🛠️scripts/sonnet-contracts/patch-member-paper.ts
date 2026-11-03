import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const anchor = "    for (const shape of VIEWPOINTS.flatMap((viewpoint) => geometry[viewpoint] ?? [])) if (shape.bone !== asset.rig.root)";
if (!t.includes(anchor)) throw new Error("anchor");
t = t.replace(anchor, "    if (symbol.units === \"paper-mm\") for (const shape of VIEWPOINTS.flatMap((viewpoint) => geometry[viewpoint] ?? [])) if (shape.kind === \"member\") add(`${path}.geometry`, \"member-in-paper-symbol\", `Symbol \"${symbol.id}\" has units paper-mm but shape \"${shape.id}\" is a member; a member is a bar of physical metres between bone-local points and nothing would be drawn in paper units.`, `Symbol \"${symbol.id}\" hat die Einheit paper-mm, aber die Form \"${shape.id}\" ist ein Stab; ein Stab ist ein Balken in physikalischen Metern zwischen knochenlokalen Punkten, und in Papiereinheiten würde nichts gezeichnet.`);\n" + anchor);
writeFileSync(file, t);
