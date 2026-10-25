// @vitest-environment node
import { describe, expect, it } from "vitest";
import { execFileSync } from "node:child_process";
import { STORY_SCOPES, buildExactAliasEntries, buildScopeAliases, buildScopeStoryGlobs, buildScopeWatchIgnores, buildScopeWatchRoots, isWatchedPathV1, resolveActiveScopes, scopeActive } from "../../📖️stories/🧭️coordination/🟦️.ts";

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
    expect(buildScopeStoryGlobs(resolveActiveScopes("puzzle"))).toEqual(["../✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/**/🧪️.story.tsx"]);
  });
  it("merges workspace and scope aliases without conflict", () => {
    const aliases = buildScopeAliases(resolveActiveScopes("ui"), { "@semio-tech/ui-react": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript" });
    expect(aliases["@semio-tech/ui-react"]).toBe("🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript");
    expect(aliases["@elements/ui/globals.css"]).toBe("🧰️framework/🔨️modules/🖱️ui/🌐️globals/🎨️.css");
  });
  it("builds exact-match alias entries that leave package subpaths to node resolution", () => {
    const own = import.meta.filename;
    const entries = buildExactAliasEntries({ "@semio-tech/assets": own, "vite/internal": own }, { storybook: "/sb" });
    const tests = (entry: { readonly find: string | RegExp }, specifier: string) => (entry.find instanceof RegExp ? entry.find.test(specifier) : entry.find === specifier);
    expect(entries.map((entry) => entry.replacement)).toEqual([own, own, "/sb"]);
    expect(entries.some((entry) => tests(entry, "@semio-tech/assets/mesh"))).toBe(false);
    expect(entries.some((entry) => tests(entry, "@semio-tech/assets"))).toBe(true);
    const oracle = `
      import { createServer } from "vite";
      const [own, sources] = JSON.parse(process.argv[1]);
      const alias = sources.map(([source, replacement]) => ({ find: new RegExp(source, "u"), replacement }));
      const server = await createServer({ configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, watch: null }, resolve: { alias }, optimizeDeps: { noDiscovery: true, include: [] } });
      const bare = await server.pluginContainer.resolveId("@semio-tech/assets", own);
      const sub = await server.pluginContainer.resolveId("@semio-tech/assets/mesh", own);
      console.log(JSON.stringify([bare?.id ?? null, sub?.id ?? null]));
      await server.close();
    `;
    const sources = entries.flatMap((entry) => (entry.find instanceof RegExp ? [[entry.find.source, entry.replacement]] : []));
    const [bare, sub] = JSON.parse(execFileSync(process.execPath, ["--input-type=module", "-e", oracle, JSON.stringify([own, sources])], { cwd: import.meta.dirname, encoding: "utf8" }).trim()) as [string | null, string | null];
    expect(bare).toBe(own);
    expect(sub).toBeTruthy();
    expect(sub!.startsWith(`${own}/`)).toBe(false);
  }, 60_000);
  it("throws on a genuine key conflict between scopes", () => {
    expect(() => buildScopeAliases([
      { id: "a", titlePrefix: "a", sourceRoots: [], aliases: { x: "1" } },
      { id: "b", titlePrefix: "b", sourceRoots: [], aliases: { x: "2" } },
    ], {})).toThrow(/alias conflict/);
  });
  it("ignores inactive scopes' source roots", () => {
    const ignores = buildScopeWatchIgnores(resolveActiveScopes("ui"));
    expect(ignores).toContain("**/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/**");
    expect(ignores.some((glob) => glob.includes("🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"))).toBe(false);
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

describe("Storybook watch roots", () => {
  const repo = "/repo";
  it("covers the .storybook folder, source roots, story glob bases and deep alias targets only", () => {
    const roots = buildScopeWatchRoots(resolveActiveScopes("ui"), repo, ["a/b/c/index.ts", "top.ts", "x/y.ts"]);
    expect(roots).toContain("/repo/.storybook");
    expect(roots).toContain("/repo/🧰️framework/🔨️modules/🖱️ui/📖️stories");
    expect(roots).toContain("/repo/🧰️framework/🔨️modules/🖼️assets");
    expect(roots).toContain("/repo/a/b/c");
    expect(roots).not.toContain("/repo");
    expect(roots).not.toContain("/repo/x");
  });
  it("walks ancestors of a root and the root itself but nothing else", () => {
    const roots = ["/repo/.storybook", "/repo/a/b/c"];
    for (const path of ["/repo/.storybook", "/repo/.storybook/main.ts", "/repo/a", "/repo/a/b", "/repo/a/b/c/d/e.ts"]) expect(isWatchedPathV1(path, roots), path).toBe(true);
    for (const path of ["/repo/.cache", "/repo/a/z", "/repo/a/b/cx", "/repo/📦️other"]) expect(isWatchedPathV1(path, roots), path).toBe(false);
  });
});
