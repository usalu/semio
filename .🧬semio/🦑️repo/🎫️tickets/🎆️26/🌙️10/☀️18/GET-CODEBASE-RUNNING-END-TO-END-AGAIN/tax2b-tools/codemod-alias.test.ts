import {expect, test} from "bun:test";
import {aliasImportText, aliasSpecifier, REPO} from "../tax2-tools/lib.ts";
const module = `${REPO}/✏️s/🔌️plugins/🀄️wfc/🧪️tests/🔬️typescript-package-commands`;
const routing = `${REPO}/🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts`;
test("a framework glue file exported by the framework package becomes its aliased workspace specifier", () => {
  expect(aliasSpecifier(module, routing)).toBe("@semio-tech/framework/process/routing");
});
test("same-technology targets stay relative", () => {
  expect(aliasSpecifier(module, `${REPO}/✏️s/🔌️plugins/🀄️wfc/🟦️.ts`)).toBeNull();
});
test("unexported cross-technology targets stay relative", () => {
  expect(aliasSpecifier(module, `${REPO}/🧰️framework/🔨️modules/🌱️value/🟦️.ts`)).toBeNull();
});
test("import statement text is rewritten per specifier", () => {
  const text = `import { runCmd } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";`;
  expect(aliasImportText(text, module)).toBe(`import { runCmd } from "@semio-tech/framework/process/execution";`);
});
