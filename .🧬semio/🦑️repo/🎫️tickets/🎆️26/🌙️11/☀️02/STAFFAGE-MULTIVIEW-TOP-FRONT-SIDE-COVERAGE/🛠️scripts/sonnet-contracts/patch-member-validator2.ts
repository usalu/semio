import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const start = t.indexOf('        if (shape.solid.kind === "member") {');
const end = t.indexOf('        if (shape.solid.kind === "extruded" && !(shape.solid.z0 < shape.solid.z1))');
if (start < 0 || end < start) throw new Error("block");
t = t.slice(0, start) + t.slice(end);
const anchor = "      if (shape.solid !== undefined) {";
const at = t.indexOf(anchor);
if (at < 0) throw new Error("anchor");
const block = `      if (shape.kind === "member") {
        const lineage = new Set<Slug>();
        for (let cursor: Slug | null | undefined = shape.bone; cursor !== null && cursor !== undefined && !lineage.has(cursor); cursor = byId.get(cursor)?.parent) lineage.add(cursor);
        for (const [name, end] of [["from", shape.from], ["to", shape.to]] as const) {
          need(\`\${sp}.\${name}.bone\`, end.bone, bones, "Bone");
          if (end.bone !== undefined && bones.has(end.bone) && !lineage.has(end.bone)) add(\`\${sp}.\${name}.bone\`, "member-end-not-ancestor", \`Member "\${shape.id}" on bone "\${shape.bone}" ends at bone "\${end.bone}", which is neither its own bone nor an ancestor; the bar would not move rigidly with its bone.\`, \`Stab "\${shape.id}" am Knochen "\${shape.bone}" endet am Knochen "\${end.bone}", der weder sein eigener noch ein Vorfahre ist; der Stab würde sich nicht starr mit seinem Knochen bewegen.\`);
        }
        if (shape.solid !== undefined) add(sp, "member-solid", \`Member "\${shape.id}" sets solid; a member is itself a solid and has no planar geometry.\`, \`Stab "\${shape.id}" setzt solid; ein Stab ist selbst ein Körper und hat keine ebene Geometrie.\`);
        const section = shape.section;
        if (!(section.kind === "rect" ? section.w > 0 && section.h > 0 : section.d > 0)) add(\`\${sp}.section\`, "member-section", \`Member "\${shape.id}" needs a positive section (w and h, or d).\`, \`Stab "\${shape.id}" braucht einen positiven Querschnitt (w und h oder d).\`);
        const same = (shape.from.bone ?? shape.bone) === (shape.to.bone ?? shape.bone);
        if (same && shape.from.x === shape.to.x && shape.from.y === shape.to.y && shape.from.z === shape.to.z) add(sp, "member-degenerate", \`Member "\${shape.id}" starts and ends at the same point.\`, \`Stab "\${shape.id}" beginnt und endet am selben Punkt.\`);
      }
`;
t = t.slice(0, at) + block + t.slice(at);
writeFileSync(file, t);
