import fs from "node:fs";
const s = fs.readFileSync("compose/js/kit-wasm-store.ts", "utf8");
const names = [];
for (const re of [/^export (async )?function (\w+)/gm, /^export const (\w+)/gm, /^export class (\w+)/gm]) {
  let m;
  while ((m = re.exec(s))) names.push(m[2] || m[1]);
}
const omit = new Set(["KIT_EVENT_STREAM_SUBSCRIPTION", "kitReadPointKey", "theKitReadPoint"]);
const filtered = names.filter((n) => !omit.has(n));
const alias = new Map([
  ["Author", "Author as AuthorGraphDto"],
  ["Concept", "Concept as ConceptGraphDto"],
  ["Connection", "Connection as ConnectionGraphDto"],
  ["Connector", "Connector as ConnectorGraphDto"],
  ["Design", "Design as DesignGraphDto"],
  ["Family", "Family as FamilyGraphDto"],
  ["Kit", "Kit as KitGraphDto"],
  ["Piece", "Piece as PieceGraphDto"],
  ["Port", "Port as PortGraphDto"],
  ["Quality", "Quality as QualityGraphDto"],
  ["Representation", "Representation as RepresentationGraphDto"],
  ["Tag", "Tag as TagGraphDto"],
  ["Type", "Type as TypeGraphDto"],
  ["openKit", "openKit as openKitStore"],
]);
const parts = filtered.map((n) => alias.get(n) || n);
const out = `export {\n  ${parts.join(",\n  ")},\n} from "./kit-wasm-store.js";\n`;
fs.writeFileSync(".repo/🎫️/26/05/12/FIELD-ONLY-KIT-READS-CQRS-CLASSES/wasm-value-reexports.txt", out, "utf8");
console.log("count", parts.length);
