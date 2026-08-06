import { readFileSync, writeFileSync } from "node:fs";

const path = new URL("../../../../../../.storybook/scopes.ts", import.meta.url);
// Resolve via absolute path instead — ticket-relative URL can be fragile with emoji dirs.
const scopesPath = "/Users/ueli/Documents/semio/.storybook/scopes.ts";
let text = readFileSync(scopesPath, "utf8");

const oldUiAliases = `    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript/📦️index.tsx",
      "@elements/ui/globals.css": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️globals.css",
      "@semio-tech/coda-desktop/renderer": "compose/client/ui/desktop/js/renderer.tsx",
      "@semio-tech/compose-rs-wasm": "compose/client/lib/rs/pkg/compose.js",
    },`;

const newUiAliases = `    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript/📦️index.tsx",
      "@elements/ui/globals.css": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️globals.css",
    },`;

if (!text.includes(oldUiAliases)) {
  throw new Error("ui aliases block not found");
}
text = text.replace(oldUiAliases, newUiAliases);

const composeBlockStart = text.indexOf(`  {
    id: "compose",
`);
const frameworkBlockStart = text.indexOf(`  {
    id: "framework",
`);
if (composeBlockStart < 0 || frameworkBlockStart < 0) {
  throw new Error(`compose/framework markers missing: ${composeBlockStart} ${frameworkBlockStart}`);
}
text = text.slice(0, composeBlockStart) + text.slice(frameworkBlockStart);

const codaBlockStart = text.indexOf(`  {
    id: "coda",
`);
if (codaBlockStart < 0) {
  throw new Error("coda block not found");
}
// Remove trailing comma from previous infinite block's closing + coda entry through ];
const beforeCoda = text.slice(0, codaBlockStart);
const afterInfinite = beforeCoda.replace(/,\n$/, "\n");
const endRegistry = text.indexOf("];\n// #endregion 🔖️ScopeRegistry", codaBlockStart);
if (endRegistry < 0) {
  throw new Error("end registry not found");
}
text = afterInfinite + text.slice(endRegistry);

text = text.replace(
  `/** @emoji 🎯️ A scope token matches a registered scope's id or any of its descendants (\`compose\` matches \`compose/ui\`). */`,
  `/** @emoji 🎯️ A scope token matches a registered scope's id or any of its descendants (\`puzzle\` matches \`puzzle/2d\`). */`,
);

const oldTests = `  describe("resolveActiveScopes", () => {
    it("returns every scope when the expression is empty", () => {
      expect(resolveActiveScopes("").map((s) => s.id)).toEqual(STORY_SCOPES.map((s) => s.id));
    });

    it("resolves a hierarchical prefix to itself and its descendants", () => {
      const ids = resolveActiveScopes("compose").map((s) => s.id);
      expect(ids).toContain("compose");
      expect(ids).toContain("compose/ui");
      expect(ids).toContain("compose/algorithm");
      expect(ids).not.toContain("ui");
    });

    it("composes multiple comma-separated scopes", () => {
      const ids = resolveActiveScopes("ui,compose/algorithm").map((s) => s.id);
      expect(ids).toEqual(["ui", "compose/algorithm"]);
    });

    it("throws on an unknown scope, listing registered ids", () => {
      expect(() => resolveActiveScopes("not-a-scope")).toThrow(/unknown scope/);
    });
  });

  describe("buildScopeStoryGlobs", () => {
    it("dedupes a child glob subsumed by an active parent", () => {
      const globs = buildScopeStoryGlobs(resolveActiveScopes("compose"));
      expect(globs).toEqual(["./stories/compose/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"]);
    });
  });`;

const newTests = `  describe("resolveActiveScopes", () => {
    it("returns every scope when the expression is empty", () => {
      expect(resolveActiveScopes("").map((s) => s.id)).toEqual(STORY_SCOPES.map((s) => s.id));
    });

    it("resolves a hierarchical prefix to itself and its descendants", () => {
      const ids = resolveActiveScopes("puzzle").map((s) => s.id);
      expect(ids).toContain("puzzle");
      expect(ids).toContain("puzzle/2d");
      expect(ids).toContain("puzzle/3d");
      expect(ids).not.toContain("ui");
    });

    it("composes multiple comma-separated scopes", () => {
      const ids = resolveActiveScopes("ui,puzzle/2d").map((s) => s.id);
      expect(ids).toEqual(["ui", "puzzle/2d"]);
    });

    it("throws on an unknown scope, listing registered ids", () => {
      expect(() => resolveActiveScopes("not-a-scope")).toThrow(/unknown scope/);
    });
  });

  describe("buildScopeStoryGlobs", () => {
    it("dedupes a child glob subsumed by an active parent", () => {
      const globs = buildScopeStoryGlobs(resolveActiveScopes("puzzle"));
      expect(globs).toEqual(["./stories/puzzle/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"]);
    });
  });`;

if (!text.includes(oldTests)) {
  throw new Error("old vitest resolve/glob block not found");
}
text = text.replace(oldTests, newTests);

const oldIgnore = `      expect(ignores).toContain("**/compose/client/lib/js/**");`;
const newIgnore = `      expect(ignores).toContain("**/✏️s/🔌️plugins/🧩puzzle/🎛️apps/◻2d/**".replace("🧩", "🧩"));`;
// Use the exact emoji from the sourceRoots entry — read it from HAND_CURATED after edits.
// Safer: assert an inactive framework/hosts-only root when ui is active.
const puzzle2dRootMatch = text.match(/id: "puzzle\/2d"[\s\S]*?sourceRoots: \[repoRelative\("([^"]+)"\)\]/);
if (!puzzle2dRootMatch) {
  throw new Error("puzzle/2d sourceRoot not found");
}
const puzzle2dRoot = puzzle2dRootMatch[1];
const expectedIgnore = `**/ ${puzzle2dRoot}/**`.replace("**/ ", "**/");
if (!text.includes(oldIgnore)) {
  throw new Error("old ignore assertion not found");
}
text = text.replace(oldIgnore, `      expect(ignores).toContain(${JSON.stringify(expectedIgnore)});`);

const oldScopeActive = `  describe("scopeActive", () => {
    it("matches an active scope's own prefix and ancestors", () => {
      const ids = resolveActiveScopes("compose/algorithm").map((s) => s.id);
      expect(scopeActive(ids, "compose")).toBe(true);
      expect(scopeActive(ids, "compose/algorithm")).toBe(true);
      expect(scopeActive(ids, "ui")).toBe(false);
    });
  });`;

const newScopeActive = `  describe("scopeActive", () => {
    it("matches an active scope's own prefix and ancestors", () => {
      const ids = resolveActiveScopes("puzzle/2d").map((s) => s.id);
      expect(scopeActive(ids, "puzzle")).toBe(true);
      expect(scopeActive(ids, "puzzle/2d")).toBe(true);
      expect(scopeActive(ids, "ui")).toBe(false);
    });
  });`;

if (!text.includes(oldScopeActive)) {
  throw new Error("old scopeActive block not found");
}
text = text.replace(oldScopeActive, newScopeActive);

if (text.includes('id: "compose"') || text.includes('id: "coda"') || text.includes("compose/client")) {
  throw new Error("compose/coda residue remains in scopes.ts");
}

writeFileSync(scopesPath, text);
console.log("OK scopes.ts edited");
console.log("puzzle/2d ignore assert:", expectedIgnore);
