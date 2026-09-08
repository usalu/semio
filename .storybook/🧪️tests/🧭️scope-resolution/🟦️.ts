import { describe, expect, it } from "vitest";
import { STORY_SCOPES, buildScopeAliases, buildScopeStoryGlobs, buildScopeWatchIgnores, resolveActiveScopes, scopeActive } from "../../scopes.ts";

describe("resolveActiveScopes", () => {
  it("returns every scope when the expression is empty", () => {
    expect(resolveActiveScopes("").map((scope) => scope.id)).toEqual(STORY_SCOPES.map((scope) => scope.id));
  });
  it("resolves a hierarchical prefix to itself and its descendants", () => {
    const ids = resolveActiveScopes("puzzle").map((scope) => scope.id);
    expect(ids).toContain("puzzle");
    expect(ids).toContain("puzzle/2d");
    expect(ids).toContain("puzzle/3d");
    expect(ids).not.toContain("ui");
  });
  it("composes multiple comma-separated scopes", () => {
    expect(resolveActiveScopes("ui,puzzle/2d").map((scope) => scope.id)).toEqual(["ui", "puzzle/2d"]);
  });
  it("throws on an unknown scope, listing registered ids", () => {
    expect(() => resolveActiveScopes("not-a-scope")).toThrow(/unknown scope/);
  });
});

describe("Storybook scope projections", () => {
  it("dedupes a child glob subsumed by an active parent", () => {
    expect(buildScopeStoryGlobs(resolveActiveScopes("puzzle"))).toEqual(["./stories/puzzle/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"]);
  });
  it("merges workspace and scope aliases without conflict", () => {
    const aliases = buildScopeAliases(resolveActiveScopes("ui"), { "@semio-tech/ui-react": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react" });
    expect(aliases["@semio-tech/ui-react"]).toBe("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react");
    expect(aliases["@elements/ui/globals.css"]).toBe("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css");
  });
  it("throws on a genuine key conflict between scopes", () => {
    expect(() => buildScopeAliases([
      { id: "a", titlePrefix: "a", sourceRoots: [], aliases: { x: "1" } },
      { id: "b", titlePrefix: "b", sourceRoots: [], aliases: { x: "2" } },
    ], {})).toThrow(/alias conflict/);
  });
  it("ignores inactive scopes' source roots", () => {
    const ignores = buildScopeWatchIgnores(resolveActiveScopes("ui"));
    expect(ignores).toContain("**/✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻️2d/**");
    expect(ignores.some((glob) => glob.includes("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react"))).toBe(false);
  });
  it("ignores nothing when every scope is active", () => {
    expect(buildScopeWatchIgnores(resolveActiveScopes(""))).toEqual([]);
  });
  it("matches an active scope's own prefix and ancestors", () => {
    const ids = resolveActiveScopes("puzzle/2d").map((scope) => scope.id);
    expect(scopeActive(ids, "puzzle")).toBe(true);
    expect(scopeActive(ids, "puzzle/2d")).toBe(true);
    expect(scopeActive(ids, "ui")).toBe(false);
  });
});
