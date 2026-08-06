// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import storybook from "eslint-plugin-storybook";

// Root ESLint flat config for Bun/Nx lint (library entrypoints; non-type-checked for green CI).
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import { readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

/** 📁️ Directory names directly under `relPath` (repo-relative), `[]` if unreadable/missing — mirrors the
 * `fs.readdirSync`-derived `PLUGINS` pattern in `.dependency-cruiser.cjs` so this list can never drift
 * from disk. */
function listDirs(relPath) {
  try {
    return readdirSync(join(__dirname, relPath), { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name);
  } catch {
    return [];
  }
}

/** 📦️ The same "package/module family" roots `.dependency-cruiser.cjs`'s `no-cross-package-relative` rule
 * governs (`26/08/06/DEPENDENCY-CRUISER-CONFIG-MODERNIZATION-FOR-TAXONOMY-SHAPE`) — kept in sync by eye
 * since one lives in a `.cjs` and the other in this `.mjs` (a shared JSON list wasn't worth the indirection
 * for 5 literal path segments both files already hardcode elsewhere, e.g. `TECHNOLOGIES`). */
const CROSS_PACKAGE_FAMILIES = [
  { root: "🧰️framework/🛍️products", members: listDirs("🧰️framework/🛍️products") },
  { root: "🧰️framework/🔨️modules", members: listDirs("🧰️framework/🔨️modules") },
  { root: "✏️s/🔨️modules", members: listDirs("✏️s/🔨️modules") },
  { root: "🌎️hub/🔨️modules", members: listDirs("🌎️hub/🔨️modules") },
  { root: "♻️mit-bestand", members: listDirs("♻️mit-bestand") },
];

/** 🪞️ ESLint mirror of dep-cruiser's `no-cross-package-relative`: for every family member, ban a relative
 * import specifier that names one of its SIBLINGS anywhere in its path — same WARN level, same
 * "package-name imports instead" rationale, same self-correcting-from-disk generation style as
 * `.dependency-cruiser.cjs`'s `crossPluginRules`/`crossPackageRelativeRule`. `no-restricted-imports`
 * `patterns` match the literal specifier text (not the resolved path dep-cruiser sees), so this is
 * necessarily a name-based heuristic rather than a byte-for-byte port of the `$1`-capture regex — a
 * specifier is only ever written relative to the importing file, so "does it name a sibling" is the
 * closest equivalent ESLint's own resolution-free rule can check. */
function crossPackageRelativeOverrides() {
  const overrides = [];
  for (const { root, members } of CROSS_PACKAGE_FAMILIES) {
    for (const member of members) {
      const siblings = members.filter((other) => other !== member);
      if (siblings.length === 0) continue;
      overrides.push({
        files: [`${root}/${member}/**`],
        rules: {
          "no-restricted-imports": [
            "warn",
            { patterns: siblings.flatMap((sibling) => [`**/${sibling}`, `**/${sibling}/**`]) },
          ],
        },
      });
    }
  }
  return overrides;
}

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  {
    ignores: ["**/node_modules/**", "**/dist/**", "**/pkg/**"],
  },
  {
    files: ["compose/client/lib/js/index.ts"],
    rules: {
      "@typescript-eslint/no-unused-vars": "off",
    },
  },
  storybook.configs["flat/recommended"],
  ...crossPackageRelativeOverrides(),
);
