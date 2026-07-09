import fs from "node:fs";
import path from "node:path";

const p = path.resolve("compose/react/index.tsx");
let s = fs.readFileSync(p, "utf8");
const parts = ["Representation", "Connection", "Connector", "Attribute", "Benchmark", "Location", "Author", "Folder", "Family", "Design", "Group", "Layer", "Quality", "Concept", "Type", "File", "Stat", "Tag", "Port", "Prop", "Piece", "Kit"];
for (const part of parts) {
  const re = new RegExp(`\\b${part}Id(?!Dto)\\b`, "g");
  s = s.replace(re, `${part}IdDto`);
}
fs.writeFileSync(p, s);
console.log("ok", p);
