import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const base=resolve("🧰️framework/🛍️products/📚️course/");
const require=createRequire(resolve(base,"🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json"));
const React=require("react"), {renderToStaticMarkup}=require("react-dom/server"), ts=require("typescript");
const source=readFileSync(resolve(base,"🎯️targets/⚛️react/🔨️modules/📝️assignments/🟦️.tsx"),"utf8");
const compiled=ts.transpileModule(source,{compilerOptions:{jsx:ts.JsxEmit.ReactJSX,module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2022}});
const exports={};new Function("require","exports",compiled.outputText)(require,exports);
const assignments=JSON.parse(readFileSync(resolve(base,"🧫️fixtures/📝️assignments/🔣️.json"),"utf8")).valid;
for(const locale of ["en","de"]) {
 const markup=renderToStaticMarkup(React.createElement(exports.AssignmentDownloads,{assignments,locale}));
 assert.ok(markup.includes(`href="/assignments/parametrization-${locale}.pdf"`));assert.ok(markup.includes(`download="parametrization-${locale}.pdf"`));
 assert.ok(markup.includes(assignments[0].title[locale]));assert.ok(markup.includes('aria-label='));
}
assert.equal(renderToStaticMarkup(React.createElement(exports.AssignmentDownloads,{assignments,locale:"en",topic:"energy"})),"");
assert.ok(renderToStaticMarkup(React.createElement(exports.AssignmentDownloads,{assignments,locale:"de",topic:"parametric"})).includes("Fingerübung"));
console.log("PASS: localized accessible download markup, English/German filenames and topic filter");
