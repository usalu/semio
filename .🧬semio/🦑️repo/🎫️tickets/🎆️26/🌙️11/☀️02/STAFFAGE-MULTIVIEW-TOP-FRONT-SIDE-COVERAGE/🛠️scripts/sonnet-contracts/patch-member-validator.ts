import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const anchor = '        if (shape.solid.kind === "extruded" && !(shape.solid.z0 < shape.solid.z1))';
if (!t.includes(anchor)) throw new Error("anchor");
const block = `        if (shape.solid.kind === "member") {
          const member = shape.solid;
          need(\`\${sp}.solid.to.bone\`, member.to.bone, bones, "Bone");
          const circular = member.diameterM !== undefined;
          const rectangular = member.widthM !== undefined || member.depthM !== undefined;
          const valid = (value: number | undefined): boolean => value !== undefined && value > 0;
          if (circular === rectangular || (circular && !valid(member.diameterM)) || (rectangular && !(valid(member.widthM) && valid(member.depthM)))) add(\`\${sp}.solid\`, "member-section", \`Shape "\${shape.id}" is a member and needs either a positive diameterM or both a positive widthM and depthM, not both and not neither.\`, \`Form "\${shape.id}" ist ein Stab und braucht entweder einen positiven diameterM oder sowohl einen positiven widthM als auch depthM, nicht beides und nicht keines von beidem.\`);
          if ((member.to.bone === undefined || member.to.bone === shape.bone) && member.from.x === member.to.x && member.from.y === member.to.y && member.from.z === member.to.z) add(\`\${sp}.solid\`, "member-degenerate", \`Member "\${shape.id}" starts and ends at the same point.\`, \`Stab "\${shape.id}" beginnt und endet am selben Punkt.\`);
        }
`;
t = t.replace(anchor, block + anchor);
writeFileSync(file, t);
