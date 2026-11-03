import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const t = readFileSync(file, "utf8");
const start = t.indexOf("/** 📐️ Nominal overall real dimensions (bounding box");
const end = t.indexOf("export type Dimensions");
if (start < 0 || end < start) throw new Error("anchor");
const doc = `/** 📐️ Nominal overall real dimensions and total mass: the physical envelope of everything that belongs to the asset in its rest pose (held items, attachments, plates, flanges, lids, footings), without stroke width, ground shading, plumes, transport or annotations; every authored representation reproduces them in every viewpoint within max(2 % of the declared value, 15 mm).
 * \`heightM\` is the extent along world y (up), \`widthM\` along world x (ahead: the real depth of an object drawn in side elevation; paper x of the side elevation and of the plan), \`depthM\` along world z (lateral: paper x of the front elevation, paper up of the plan; the left-right width of an object drawn in side elevation). */
`;
writeFileSync(file, t.slice(0, start) + doc + t.slice(end));
