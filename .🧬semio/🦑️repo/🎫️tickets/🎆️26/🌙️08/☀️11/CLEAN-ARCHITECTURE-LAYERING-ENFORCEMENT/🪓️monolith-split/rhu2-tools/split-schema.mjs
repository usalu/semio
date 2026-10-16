import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const SRC = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
const OUT = process.argv[2];
const text = fs.readFileSync(SRC, "utf8");
const sf = ts.createSourceFile(SRC, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const OSNAMES = new Set(["parseDocumentOpenPackageV1", "documentOpenObject", "documentOpenText", "documentOpenHash", "documentOpenInteger", "documentOpenReceipt", "parseDocumentOpenScope", "parseDocumentOpenFrontier", "parseDocumentOpenIntentV1", "parseDocumentPlanSocketGrantIntentV1", "parseDocumentOpenPlanV1", "documentExecutionTargetByteLength", "parseDocumentExecutionTargetLeaseFieldsV1", "leaseFieldsFromPlanV1", "sameLeaseFieldsV1", "sameExecutionTargetV1", "DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES"]);
const stmtName = st => { if (ts.isFunctionDeclaration(st) && st.name) return [st.name.text]; if (ts.isVariableStatement(st)) return st.declarationList.declarations.map(d => d.name.getText()); return []; };
const f = [], o = [], seen = new Set();
for (const st of sf.statements) {
  const names = stmtName(st);
  const os = names.length > 0 && names.every(n => OSNAMES.has(n));
  if (names.some(n => OSNAMES.has(n)) && !os) throw new Error("mixed statement " + names);
  names.forEach(n => seen.add(n));
  const full = text.slice(st.getFullStart(), st.end);
  (os ? o : f).push({ st, full, names });
}
for (const n of OSNAMES) if (!seen.has(n)) throw new Error("missing OS name " + n);
fs.mkdirSync(OUT, { recursive: true });
fs.writeFileSync(OUT + "/f-body.txt", f.map(x => x.full).join(""));
fs.writeFileSync(OUT + "/os-body.txt", o.map(x => x.full).join(""));
console.log("F statements", f.length, "OS statements", o.length);
console.log("OS blocks:", o.map(x => x.names.join(",")).join(" | "));
