const fs = require("node:fs");
const crypto = require("node:crypto");
const pathModule = require("node:path");
const ts = require("typescript");
const Ajv2020 = require("ajv/dist/2020").default;
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/";
const paths = [
  base + "🎬️media/🚚️lifecycle/🟦️.ts",
  base + "🎬️media/📤️file/🟦️.ts",
  base + "🧪️tests/📤️checked-media-export/🟦️.ts",
  base + "🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx",
  base + "🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔸️document-transfer-ref/🟦️.tsx",
  base + "🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🟦️.tsx",
  base + "🧱️elements/🏛️ShellHost/🧩️shell-core-state/🟦️.ts",
  base + "🧱️elements/🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🟦️.ts",
  base + "🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🟦️.ts",
];
const receipts = paths.map(path => {
  if ((process.cwd() + "/" + path).length > 256) throw new Error("path length: " + path);
  const source = fs.readFileSync(path, "utf8");
  for (const match of source.matchAll(/\bfrom\s+["'](\.[^"']+)["']/gu)) if (!fs.existsSync(pathModule.resolve(pathModule.dirname(path), match[1]))) throw new Error("relative import: " + path + " -> " + match[1]);
  const parsed = ts.transpileModule(source, { fileName: path, reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, jsx: ts.JsxEmit.Preserve } });
  const diagnostics = parsed.diagnostics.filter(diagnostic => diagnostic.category === ts.DiagnosticCategory.Error);
  if (diagnostics.length) throw new Error(ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCurrentDirectory: () => process.cwd(), getCanonicalFileName: name => name, getNewLine: () => "\n" }));
  return { path, sha256: crypto.createHash("sha256").update(source).digest("hex") };
});
const fixtureRoot = base + "🧪️tests/📤️checked-media-export/";
const ajv = new Ajv2020({ strict: true });
for (const file of ["🔣️.json", "📜️declarations.json"]) {
  const validate = ajv.compile(JSON.parse(fs.readFileSync(fixtureRoot + "🧬️schema/" + file, "utf8")));
  if (!validate(JSON.parse(fs.readFileSync(fixtureRoot + file, "utf8")))) throw new Error(JSON.stringify(validate.errors));
}
const policy = JSON.parse(fs.readFileSync(base + "🎬️media/🚚️lifecycle/🧬️schema/🔣️.json", "utf8")).const;
const agentPolicy = JSON.parse(fs.readFileSync(base + "🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧬️schema/🔣️.json", "utf8")).$defs.ExportOperationPolicy.const;
if (JSON.stringify(policy) !== JSON.stringify(agentPolicy)) throw new Error("media operation policy parity");
console.log(JSON.stringify({ kind: "parse-and-schema-only", files: receipts, neutralSchemas: 2, policyParity: true }));
