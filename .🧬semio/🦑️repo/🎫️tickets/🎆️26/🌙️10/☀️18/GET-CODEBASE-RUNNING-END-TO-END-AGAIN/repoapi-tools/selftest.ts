import { libTargets, cliClosure } from "./lib.ts";
import { plan, type Ctx } from "./engine.ts";
import { frameworkRules, jsonNames, libRelOf } from "./rules.ts";
import { LIB_REL } from "./lib.ts";

const known = libTargets();
const jn = jsonNames([...known].filter(t => t.endsWith(".json")).map(libRelOf));
const ctx: Ctx = { known, fw: frameworkRules(), closure: new Set(), relative: false, jsonNames: jn, blocked: new Set() };
const L = "../../../" + LIB_REL + "/";
const file = "🌎️hub/🧪️tests/🧪️synthetic/🟦️.ts";
let failed = 0;
const check = (name: string, text: string, expectIncludes: string[], expectExcludes: string[] = []): void => {
  const r = plan(file, text, ctx);
  const ok = expectIncludes.every(s => r.text.includes(s)) && expectExcludes.every(s => !r.text.includes(s)) && r.failures.length === 0;
  if (!ok) { failed++; console.log("FAIL", name, "\n" + r.text, r.failures); } else console.log("ok  ", name);
};
check("framework rename + shorthand", `import { getWorkspaceRoot } from "${L}🗂️workspaces/🟦️.ts";\nconst o = { getWorkspaceRoot };\nexport const r = getWorkspaceRoot();\n`, ['import { findWorkspaceRoot } from "@semio-tech/process/routing";', "{ getWorkspaceRoot: findWorkspaceRoot }", "= findWorkspaceRoot()"]);
check("alias kept", `import { getWorkspaceRoot as root } from "${L}🗂️workspaces/🟦️.ts";\nexport const r = root();\n`, ["{ findWorkspaceRoot as root }", "= root()"]);
check("domain + merge", `import { withRepositoryCacheStoreV1 } from "${L}⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";\nimport { withRepositoryCompilerStoreV1 } from "${L}⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";\nexport const a = [withRepositoryCacheStoreV1, withRepositoryCompilerStoreV1];\n`, ['import { withRepositoryCacheStoreV1, withRepositoryCompilerStoreV1 } from "@semio-tech/repo-lib/storage";']);
check("dead import removed", `import { runBun } from "${L}🏃️process/🍞️bun/🟦️.ts";\nexport const x = 1;\n`, ["export const x = 1;"], ["runBun", "import"]);
check("json default", `import schema from "${L}⚡️caching/📦️artifacts/💾️storage/🧬️schema/🔣️.json" with {type:"json"};\nexport const s = schema;\n`, ['import { artifactsStorageSchema as schema } from "@semio-tech/repo-lib/artifacts";'], ["with"]);
check("dynamic json default", `export const s = (await import("${L}⚡️caching/📦️artifacts/💾️storage/🧬️schema/🔣️.json")).default;\n`, ['import("@semio-tech/repo-lib/artifacts")', ").artifactsStorageSchema"]);
check("type import", `import type { RepositoryCacheStoreV1 } from "${L}⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";\nexport type T = RepositoryCacheStoreV1;\n`, ['import type { RepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";']);
check("mixed framework + domain in one statement", `import { runCmd, tryRun } from "${L}🏃️process/🟦️.ts";\nrunCmd("a", []); tryRun("b", []);\n`, ['import { runCmd } from "@semio-tech/process/execution";', 'import { tryRun } from "@semio-tech/repo-lib/process";']);
check("cargo exact laws drops preparation", `import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "${L}🏃️process/🦀️cargo/🎯️exact/🟦️.ts";\nawait runRepositoryExactCargoLaws({ preparation: configuredRepositoryCargoPreparationPolicyV1(), x: 1 });\n`, ['import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";', "runAdmittedExactCargoLawsV1({ x: 1 })"], ["preparation", "configuredRepositoryCargoPreparationPolicyV1"]);
check("cargo exact laws, leftover usage falls back to domain", `import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "${L}🏃️process/🦀️cargo/🎯️exact/🟦️.ts";\nconst p = configuredRepositoryCargoPreparationPolicyV1();\nawait runRepositoryExactCargoLaws({ preparation: p });\n`, ['from "@semio-tech/repo-lib/cargo"'], ["@semio-tech/process/cargo-test"]);
check("network argument dropped only inside a port call", `import { buildRepositoryCargoArtifacts } from "${L}⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";\nimport { repositoryCargoNetworkV1 } from "${L}🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";\nawait buildRepositoryCargoArtifacts("m", [], "r", { invocation, network: repositoryCargoNetworkV1(invocation, "r"), producer: "artifact" });\n`, ['buildAdmittedCargoArtifactsV1("m", [], "r", { invocation, producer: "artifact" })'], ["repositoryCargoNetworkV1"]);
check("network argument elsewhere keeps the domain entry", `import { repositoryCargoNetworkV1 } from "${L}🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";\nawait other({ network: repositoryCargoNetworkV1(invocation, "r") });\n`, ['from "@semio-tech/repo-lib/cargo"', "network: repositoryCargoNetworkV1(invocation"]);
check("repo product importer keeps domain", `import { runCmd } from "${L}🏃️process/🟦️.ts";\nrunCmd("a", []);\n`, []);
{
  const r = plan("🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts", `import { runCmd } from "../../../../../${LIB_REL}/🏃️process/🟦️.ts";\nrunCmd("a", []);\n`, ctx);
  const ok = r.text.includes('from "@semio-tech/repo-lib/process"') && r.a.length === 0;
  if (!ok) { failed++; console.log("FAIL repo product keeps domain", r.text); } else console.log("ok   repo product importer keeps the repository entry");
}
{
  const r = plan("🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts", `import { runVitest } from "../../../../../${LIB_REL}/🧪️testing/🧪️vitest/🟦️.ts";\nawait runVitest("a", [], "c");\n`, ctx);
  const ok = /from "\.\.\/.*🏃️process\/📦️packages\/🟦️typescript\/🧪️testing\/🧪️vitest\/🟦️\.ts"/u.test(r.text);
  if (!ok) { failed++; console.log("FAIL script relative", r.text); } else console.log("ok   scripts stay relative to the glue");
}
console.log(failed ? `${failed} FAILED` : "all passed");
void cliClosure;
process.exit(failed ? 1 : 0);
