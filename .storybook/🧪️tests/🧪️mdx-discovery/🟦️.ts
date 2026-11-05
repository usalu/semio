/** 📚️ Dependency documentation never becomes authored Storybook MDX input. */
import { expect, test } from "vitest";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import fg from "fast-glob";
import { assertNoOwnedStorybookMdx } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🟦️.ts";
import fixture from "../🧫️fixtures/📚️mdx-discovery/🔣️.json";

for (const row of fixture.cases) test(row.path, () => {
  const root = mkdtempSync(join(tmpdir(), "storybook-mdx-"));
  try {
    const file = join(root, row.path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, "# Documentation");
    const oracle = fg.sync("**/*.mdx", { cwd: root, dot: true, ignore: fixture.ignoredDirectories.map(directory => `**/${directory}/**`) });
    expect(oracle.length > 0).toBe(row.owned);
    if (row.owned) expect(() => assertNoOwnedStorybookMdx(root)).toThrow("owned MDX");
    else expect(() => assertNoOwnedStorybookMdx(root)).not.toThrow();
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
