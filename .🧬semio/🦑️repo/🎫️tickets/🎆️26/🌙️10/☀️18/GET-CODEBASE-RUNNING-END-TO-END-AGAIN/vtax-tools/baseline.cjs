// usage: node baseline.cjs prepare | swap | restore
// prepare: stores the post-split versions of the three edited files (after/) and derives the pre-split versions (before/). swap: installs the pre-split files. restore: reinstalls the post-split files.
const fs = require("fs"), path = require("path"), crypto = require("crypto");
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const files = {
  authority: L + "🔍️discovery/🔐️taxonomy-authority/🟦️.ts",
  reuse: L + "🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts",
  loading: L + "🧪️tests/🔬️workspace-contract/📖️taxonomy-loading/🟦️.ts",
};
const DIR = "/Users/ueli/.cache/vtax/baseline-swap";
const sha = (t) => crypto.createHash("sha256").update(t).digest("hex").slice(0, 12);
const mode = process.argv[2];
fs.mkdirSync(DIR + "/after", { recursive: true }); fs.mkdirSync(DIR + "/before", { recursive: true });
if (mode === "prepare") {
  for (const [k, f] of Object.entries(files)) fs.copyFileSync(f, `${DIR}/after/${k}.ts`);
  fs.copyFileSync(path.join(__dirname, "orig/taxonomy-authority.ts"), DIR + "/before/authority.ts");
  let reuse = fs.readFileSync(files.reuse, "utf8");
  const hStart = reuse.indexOf("/** 🧾️ The validator and every audit function it calls"), hEnd = reuse.indexOf("/** 🔬️ Instruments only this exact compiled declaration closure");
  reuse = reuse.slice(0, hStart) + reuse.slice(hEnd);
  const lawStart = reuse.indexOf("  const auditOwners = validatorOwners()"), lawEnd = reuse.indexOf("});", lawStart);
  reuse = reuse.slice(0, lawStart) + `  const validator = declaration(authority, "validateTaxonomy"), validationCalls = nodes(validator, ts.isCallExpression);
  expect(validationCalls.filter((node) => node.expression.getText(source) === "createTaxonomyPathMatcher")).toHaveLength(1);
  expect(validationCalls.filter((node) => ["taxonomyPathPatternMatches", "taxonomyPatternExpression"].includes(node.expression.getText(source)))).toEqual([]);
  expect(validationCalls.filter((node) => node.expression.getText(source) === "pathMatcher.matches")).toHaveLength(2);
` + reuse.slice(lawEnd);
  reuse = reuse.replace("const matcherTypeOwners = [...new Set([...normalizationSourceFiles(normalizerPath).keys(), sourcePath, authorityPath, ...validatorOwners().map((entry) => entry.path)])];", "const matcherTypeOwners = [...normalizationSourceFiles(normalizerPath).keys()].concat(sourcePath, authorityPath);");
  fs.writeFileSync(DIR + "/before/reuse.ts", reuse);
  let loading = fs.readFileSync(files.loading, "utf8");
  const a = loading.indexOf('\n  test("each rule group of the decomposed validator'), b = loading.indexOf('\n  test("freezes the four canonical testing collections');
  loading = loading.slice(0, a) + loading.slice(b);
  fs.writeFileSync(DIR + "/before/loading.ts", loading);
  for (const k of Object.keys(files)) console.log(k, "after", sha(fs.readFileSync(`${DIR}/after/${k}.ts`, "utf8")), "before", sha(fs.readFileSync(`${DIR}/before/${k}.ts`, "utf8")));
} else if (mode === "swap" || mode === "restore") {
  const from = mode === "swap" ? "before" : "after";
  if (mode === "swap") for (const [k, f] of Object.entries(files)) { const now = fs.readFileSync(f, "utf8"), saved = fs.readFileSync(`${DIR}/after/${k}.ts`, "utf8"); if (now !== saved) { console.log(`refusing: ${k} changed since prepare`); process.exit(1); } }
  for (const [k, f] of Object.entries(files)) fs.copyFileSync(`${DIR}/${from}/${k}.ts`, f);
  console.log(mode, "done", Object.keys(files).map((k) => k + ":" + sha(fs.readFileSync(files[k], "utf8"))).join(" "));
}
