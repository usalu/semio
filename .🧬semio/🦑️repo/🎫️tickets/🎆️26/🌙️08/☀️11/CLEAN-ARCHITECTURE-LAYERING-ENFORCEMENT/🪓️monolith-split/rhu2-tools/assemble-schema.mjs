import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const write = process.argv[2] === "--write";
const SRC = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
const F_OUT = R + "🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
const OS_OUT = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
const BROWSER_ACTOR_F = R + "🧰️framework/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts";
const RECEIPT_OS = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts";
const text = fs.readFileSync(SRC, "utf8");
const sf = ts.createSourceFile(SRC, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const OSNAMES = new Set(["parseDocumentOpenPackageV1", "documentOpenObject", "documentOpenText", "documentOpenHash", "documentOpenInteger", "documentOpenReceipt", "parseDocumentOpenScope", "parseDocumentOpenFrontier", "parseDocumentOpenIntentV1", "parseDocumentPlanSocketGrantIntentV1", "parseDocumentOpenPlanV1", "documentExecutionTargetByteLength", "parseDocumentExecutionTargetLeaseFieldsV1", "leaseFieldsFromPlanV1", "sameLeaseFieldsV1", "sameExecutionTargetV1", "DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES"]);
const names = st => { if (ts.isFunctionDeclaration(st) && st.name) return [st.name.text]; if (ts.isVariableStatement(st)) return st.declarationList.declarations.map(d => d.name.getText()); return []; };
const fParts = [], oParts = [];
let receiptFnImport = null;
for (const st of sf.statements) {
  const n = names(st);
  let full = text.slice(st.getFullStart(), st.end);
  if (n.length && n.every(x => OSNAMES.has(x))) { oParts.push(full); continue; }
  if (n.some(x => OSNAMES.has(x))) throw new Error("mixed " + n);
  if (ts.isImportDeclaration(st)) {
    const spec = st.moduleSpecifier.text;
    if (spec.endsWith("🏷️component-receipt/🧬️schema/🟦️.ts")) { receiptFnImport = full; continue; }
    if (spec === "./🌐️browser-actor/🟦️.ts") {
      full = full.replace(/import \{[\s\S]*?\} from "\.\/🌐️browser-actor\/🟦️\.ts";/u, 'import type { DocumentExecutionTargetBrowserActorV1, DocumentOpenBrowserActorV1 } from "./🌐️browser-actor/🟦️.ts";');
    }
    if (spec.endsWith("🏷️component-receipt/🟦️.ts")) full = full.replace(/import \{ type ComponentReceiptV1 \} from "[^"]+";/u, 'import type { ComponentReceiptV1 } from "../../🏷️component-receipt/🟦️.ts";');
  }
  fParts.push(full);
}
if (!receiptFnImport) throw new Error("receipt function import not found");
let fText = fParts.join("") + text.slice(sf.statements[sf.statements.length - 1].end);
fText = fText.split('"../../../../../🔨️modules/').join('"../../');
const regionAnchor = "/** 🧱️ Exact byte identity of the verified component, duplicated from {@link DocumentOpenPackageV1}";
if (fText.split(regionAnchor).length !== 2) throw new Error("region anchor");
fText = fText.replace(regionAnchor, "//#region 🪪️ExecutionTargetLease\n" + regionAnchor);
const fSf = ts.createSourceFile(F_OUT, fText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const fExports = new Map();
for (const st of fSf.statements) {
  const exp = ts.canHaveModifiers(st) && (ts.getModifiers(st) ?? []).some(m => m.kind === ts.SyntaxKind.ExportKeyword);
  if ((ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) && exp) fExports.set(st.name.text, "type");
  else if (ts.isFunctionDeclaration(st) && exp && st.name) fExports.set(st.name.text, "value");
  else if (ts.isVariableStatement(st) && exp) st.declarationList.declarations.forEach(d => fExports.set(d.name.getText(), "value"));
  else if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause)) st.exportClause.elements.forEach(e => fExports.set(e.name.text, st.isTypeOnly || e.isTypeOnly ? "type" : "value"));
}
const oBody = oParts.join("");
const oSf = ts.createSourceFile(OS_OUT, oBody, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const used = new Set(); const visit = n => { if (ts.isIdentifier(n)) used.add(n.text); ts.forEachChild(n, visit); }; visit(oSf);
const BA = ["parseDocumentOpenBrowserActorV1", "parseDocumentExecutionTargetBrowserActorV1", "documentBrowserActorLeaseFromPlanV1", "sameDocumentBrowserActorV1"];
const fromF = [...used].filter(n => fExports.has(n) && !BA.includes(n)).sort();
const baNames = ["parseDocumentOpenBrowserActorV1", "parseDocumentExecutionTargetBrowserActorV1", "documentBrowserActorLeaseFromPlanV1", "sameDocumentBrowserActorV1"].filter(n => used.has(n));
const crNames = ["SHIPPED_COMPONENT_MAX_BYTES", "componentReceiptBindsV1", "componentReceiptValueV1", "encodeComponentReceiptV1"].filter(n => used.has(n));
const rel = (to) => { let r = path.relative(path.dirname(OS_OUT), to).normalize("NFC"); if (!r.startsWith(".")) r = "./" + r; return r; };
const list = (arr, typeOf) => arr.map(n => (typeOf(n) === "type" ? "type " : "") + n).join(", ");
const imports = [
  `import { ${list(fromF, n => fExports.get(n))} } from "${rel(F_OUT)}";`,
  `import { ${baNames.join(", ")} } from "${rel(BROWSER_ACTOR_F)}";`,
  `import { ${crNames.join(", ")} } from "${rel(RECEIPT_OS)}";`,
].join("\n");
const header = `/** 🔏️ Document open plan and execution-target lease admission: the strict parsers and full-field relations that hold a verified package projection to its shipped component receipt. The wire shapes are the types of the neutral directory schema; only these parsers depend on the receipt schema authority. */\n${imports}\n`;
const regionEnd = "\n//#endregion 🪪️ExecutionTargetLease\n";
const oText = header + (oBody.startsWith("\n") ? oBody : "\n" + oBody) .replace(/\n+$/u, "\n") + regionEnd;
console.log({ fromF, baNames, crNames, fLines: fText.split("\n").length, oLines: oText.split("\n").length, regionsF: (fText.match(/#region/g) ?? []).length, endF: (fText.match(/#endregion/g) ?? []).length, regionsO: (oText.match(/#region/g) ?? []).length, endO: (oText.match(/#endregion/g) ?? []).length });
fs.mkdirSync(path.dirname(path.join(R, "x")), { recursive: true });
const wd = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rhu2-tools/work/schema/";
fs.mkdirSync(wd, { recursive: true });
fs.writeFileSync(wd + "f-schema.ts", fText); fs.writeFileSync(wd + "os-document-open.ts", oText);
if (write) { fs.mkdirSync(path.dirname(F_OUT), { recursive: true }); fs.mkdirSync(path.dirname(OS_OUT), { recursive: true }); fs.writeFileSync(F_OUT, fText); fs.writeFileSync(OS_OUT, oText); console.log("written"); }
