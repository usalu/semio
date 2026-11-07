import { strict as assert } from "node:assert";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import cases from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/🧬️schema/🔣️.json" with { type: "json" };

const { portableMsvcEnvironmentV1 } = await import(pathToFileURL(join(process.cwd(), "🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/🟦️.ts")).href);
const p = schema.definitions.Protocol.const, receipt = { version: p.version, manifestSha256: p.manifestSha256, compilerVersion: p.compilerVersion, sdkVersion: p.sdkVersion };
for (const row of cases.cases) {
  const actual = portableMsvcEnvironmentV1(row.environment, { platform: row.platform, architecture: row.architecture, root: "D:\\repo", receipt: row.complete ? receipt : undefined, isFile: () => row.complete });
  assert.equal(actual !== null, row.selected, row.name);
}
console.log(`${cases.cases.length} portable MSVC environment cases passed`);
