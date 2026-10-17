// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineOwnedTestConfig } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const configDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const root = resolve(configDir, "../.."); // extension root

export default defineOwnedTestConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/cad-js-module-aec-building-structure",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["🟦️.ts"] },
    includeSource: ["🟦️.ts"],
  },
});
