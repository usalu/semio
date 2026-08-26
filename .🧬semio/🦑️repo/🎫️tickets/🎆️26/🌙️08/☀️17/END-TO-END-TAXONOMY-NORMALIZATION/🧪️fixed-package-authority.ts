import { strict as assert } from "node:assert";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const discoveryPath = join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts");
const {
  classifyPackageSourceRole,
  classifyPackageSourceDisposition,
  fixedFilenameContractIdsForPath,
  fixedFilenameRejectionContractIdForPath,
  loadTaxonomy,
  validateTaxonomy,
} = await import(pathToFileURL(discoveryPath).href);

//#region 🔒️FixedContracts
const taxonomy = loadTaxonomy();
assert.deepEqual(validateTaxonomy(taxonomy), []);
assert.deepEqual(taxonomy.scopedFileKinds, {});
assert.equal(taxonomy.fixedFilenameContracts["cargo-cache-tag"], undefined);
assert.equal(taxonomy.fixedFilenameContracts["root-readme"]?.pathPattern, "README.md");
assert.equal(taxonomy.fixedFilenameContracts["root-license"]?.pathPattern, "LICENSE.md");
assert.equal(taxonomy.fixedFilenameContracts["root-cargo"]?.pathPattern, "Cargo.toml");
assert.equal(taxonomy.fixedFilenameContracts["root-package"]?.pathPattern, "package.json");

const exactFixed = {
  ".cargo/config.toml": "cargo-workspace-config",
  ".codex/config.toml": "codex-workspace-config",
  "pyproject.toml": "root-python-tooling",
  "tsconfig.json": "root-typescript-config",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json": "window-kits-typescript-config",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/go.mod": "repo-cli-go-module",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/go.mod": "repo-mcp-go-module",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/go.mod": "repo-library-go-module",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/go.mod": "repo-coordinator-go-module",
} as const;
for (const [path, contractId] of Object.entries(exactFixed)) {
  assert.ok(existsSync(path));
  assert.deepEqual(fixedFilenameContractIdsForPath(path, taxonomy), [contractId]);
}

const rejectionIdentities = Object.values(taxonomy.fixedFilenameRejectionContracts).flatMap((contract) => contract.sourcePathIdentities);
assert.equal(rejectionIdentities.length, 23);
assert.equal(new Set(rejectionIdentities).size, 23);
for (const path of rejectionIdentities) {
  assert.ok(existsSync(path));
  assert.ok(fixedFilenameRejectionContractIdForPath(path, taxonomy));
}
assert.equal(fixedFilenameRejectionContractIdForPath("not-authorized/progress.md", taxonomy), null);

const collision = structuredClone(taxonomy) as typeof taxonomy;
(collision.fixedFilenameContracts as Record<string, unknown>)["duplicate-root-nx"] = structuredClone(taxonomy.fixedFilenameContracts["root-nx"]!);
assert.throws(() => fixedFilenameContractIdsForPath("nx.json", collision), /equal-specificity fixed filename contracts/u);
//#endregion 🔒️FixedContracts

//#region 📦️PackageBoundaries
assert.equal(taxonomy.ecosystems["🟨️javascript"]?.packageIdentity, "boundary-only");
assert.equal(taxonomy.packageBoundaryRules["🟨️javascript"]?.glueGrammarId, "javascript");
assert.equal(taxonomy.packageBoundaryProfiles["c-cpp"]?.admission, "blocked-until-language-directory-registered");
assert.deepEqual(taxonomy.packageBoundaryProfiles["c-cpp"]?.allowedFileKindIds, ["c-source", "cpp-source"]);
assert.equal(taxonomy.packageGlueGrammar.javascript?.analyzer, "javascript");
assert.equal(taxonomy.packageGlueGrammar["c-cpp"]?.analyzer, "c-cpp");

const javascript = taxonomy.packageGlueGrammar.javascript!;
assert.equal(classifyPackageSourceRole('import { api } from "./api.js";\nexport { api };', javascript), "declaration");
assert.equal(classifyPackageSourceRole("export function solve(value) { return value * 2; }", javascript), "implementation");
const native = taxonomy.packageGlueGrammar["c-cpp"]!;
assert.equal(classifyPackageSourceRole('#include "api.h"\nextern "C" int api_run(int value);', native), "declaration");
assert.equal(classifyPackageSourceRole('extern "C" int api_run(int value) { return core_run(value); }', native), "thin-delegation");
assert.equal(classifyPackageSourceRole("int solve(int value) { for (int i = 0; i < value; ++i) value += i; return value; }", native), "implementation");

const dispositions = taxonomy.packageSourceDispositions;
assert.deepEqual(Object.keys(dispositions).sort(), ["python-init", "root-script", "rust-binary-entry", "rust-library-entry", "typescript-library-entry", "typescript-react-entry"].sort());
assert.equal(dispositions["root-script"]?.disposition, "tool-metadata");
assert.equal(dispositions["python-init"]?.disposition, "adapter-source");
assert.equal(classifyPackageSourceDisposition("const router = new ScriptRouter(import.meta.dir); await runBundleScriptMain(router, import.meta.url);", dispositions["root-script"]!, javascript), "tool-metadata");
assert.equal(classifyPackageSourceDisposition("export function domainAlgorithm() {}", dispositions["root-script"]!, javascript), "unresolved");
//#endregion 📦️PackageBoundaries

console.log(JSON.stringify({
  exactFixedContracts: Object.keys(exactFixed).length,
  fixedRejectionIdentities: rejectionIdentities.length,
  packageBoundaryProfiles: Object.keys(taxonomy.packageBoundaryProfiles).length,
  packageSourceDispositions: Object.keys(dispositions).length,
}));
