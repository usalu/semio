/** 📦️ Exact case-sensitive public subpaths agree with their schema and Node's authored export map. */
import { expect, test } from "vitest";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/📦️workspace-exports/🔣️.json";
import importSchema from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🎁️package-projection/🧬️schema/📦️workspace-import/🔣️.json";
import { parseSemanticPackageBrowserProfile } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🎁️package-projection/🟦️.ts";

const root = fileURLToPath(new URL("../../../", import.meta.url));
const taxonomy = JSON.parse(readFileSync(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const validate = new Ajv().compile(importSchema);
const require = createRequire(resolve(root, fixture.manifest));

for (const row of fixture.cases) test(row.specifier, () => {
  expect(validate(row.specifier)).toBe(row.syntax);
  if (row.exported) {
    expect(require.resolve(row.specifier)).toBe(resolve(root, profile.workspaceImports[row.specifier].entryPath));
    expect(parseSemanticPackageBrowserProfile(profile, taxonomy.pathEmojiPolicy.genericEmojiIdentities)).toEqual(profile);
  } else {
    expect(() => require.resolve(row.specifier)).toThrow();
  }
});
