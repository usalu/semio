// #region 🧲️Header
// 💻️ .storybook/scopes.ts
// Specs: Single source of truth for the root Storybook's composable scope system.
// Summary: Replaces the three hardcoded `ui`/`puzzle`/`compose` stacks in `main.ts` with a data-driven
// registry every consumer derives from: story globs, workspace aliases, watch-ignores, static-dir
// assets (e.g. `/plugin-modules` for OS-shell program boot stories), and lazy scope-gated Vite plugins.
// Pure data + pure functions only — no Vite/Storybook imports — so `script.ts` (bun, CLI validation)
// and `playwright.config.ts` can import it without dragging in the Vite/MDX module graph.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Plugin } from "vite";
import type { PlaygroundAssetSpec } from "../🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";

export type { PlaygroundAssetSpec };

// #region 🔖️ScopeModel
/** @emoji 🗂️ One composable Storybook slice. `id` is hierarchical and mirrors `stories/<id>/` exactly. */
export type StoryScope = {
  readonly id: string;
  /** Emoji-namespaced `meta.title` prefix every story under `stories/<id>/**` must start with. */
  readonly titlePrefix: string;
  /** Repo-root-relative source dirs this scope's stories import from. Watch-ignores are DERIVED: union(all scopes' roots) − union(active scopes' roots). */
  readonly sourceRoots: readonly string[];
  /** Only irregular aliases (css subpaths, single-file entries, wasm pkg entries, `/plugin-modules`-style routes). Regular `@semio-tech/*` aliases are auto-derived from the workspace scan. */
  readonly aliases?: Readonly<Record<string, string>>;
  /** Extra `optimizeDeps.exclude` entries beyond the workspace-package scan. */
  readonly optimizeDepsExclude?: readonly string[];
  /** Static-dir / tile-proxy / mesh-collection assets served via the existing `playgroundAssetVitePlugins` dispatcher. */
  readonly assets?: readonly PlaygroundAssetSpec[];
  /** Lazy scope-gated Vite plugins (only imported when this scope is active). */
  readonly vitePlugins?: () => Promise<Plugin[]>;
};
// #endregion 🔖️ScopeModel

// #region 🔖️ScopeRegistry
const repoRelative = (path: string) => path;

/** @emoji 🗂️ Every registered Storybook scope. Add a row here + `stories/<id>/` to add a new slice — no other file needs to know about it. */
export const STORY_SCOPES: readonly StoryScope[] = [
  {
    id: "ui",
    titlePrefix: "🖱️ui⚛️react",
    sourceRoots: [repoRelative("framework/module/ui/js/react"), repoRelative("framework/module/ui/styling"), repoRelative("framework/module/ui/asset"), repoRelative("s/plugin/puzzle/module/asset"), repoRelative("framework/product/os/module/infinite/canvas/react-renderer"), repoRelative("compose/client/ui/desktop")],
    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "framework/product/os/module/infinite/canvas/react-renderer/index.tsx",
      "@elements/ui/globals.css": "framework/module/ui/js/react/globals.css",
      "@semio-tech/coda-desktop/renderer": "compose/client/ui/desktop/js/renderer.tsx",
    },
  },
  {
    id: "styling",
    titlePrefix: "🎨️styling",
    sourceRoots: [repoRelative("framework/module/ui/styling")],
  },
  {
    id: "puzzle",
    titlePrefix: "🧩️puzzle",
    sourceRoots: [repoRelative("s/plugin/puzzle/module/asset")],
  },
  {
    id: "puzzle/2d",
    titlePrefix: "🧩️puzzle🩻️2d",
    sourceRoots: [repoRelative("s/plugin/puzzle/app/2d"), repoRelative("s/plugin/puzzle/module/asset")],
  },
  {
    id: "puzzle/3d",
    titlePrefix: "🧩️puzzle🧊️3d",
    sourceRoots: [repoRelative("s/plugin/puzzle/app/3d"), repoRelative("s/plugin/puzzle/module/asset"), repoRelative("framework/product/os/module/infinite/world/r3f")],
  },
  {
    id: "puzzle/5d",
    titlePrefix: "🧩️puzzle🕐️5d",
    sourceRoots: [repoRelative("s/plugin/puzzle/app/5d"), repoRelative("s/plugin/puzzle/module/asset")],
  },
  {
    id: "block",
    titlePrefix: "🧱️block",
    sourceRoots: [repoRelative("s/plugin/block")],
  },
  {
    id: "compose",
    titlePrefix: "🏘️compose",
    sourceRoots: [repoRelative("compose/client/lib/js"), repoRelative("compose/client/lib/rs"), repoRelative("framework/module/asset"), repoRelative("compose/fixture"), repoRelative("compose/dev/algorithm")],
    aliases: {
      "@semio-tech/ui-react/globals.css": "framework/module/ui/js/react/globals.css",
      "@semio-tech/compose-rs-wasm": "compose/client/lib/rs/pkg/compose.js",
    },
    optimizeDepsExclude: ["@semio-tech/compose-react", "@semio-tech/compose-js", "@semio-tech/asset"],
  },
  {
    id: "compose/ui",
    titlePrefix: "🏘️compose⚛️react",
    sourceRoots: [repoRelative("compose/client/lib/js"), repoRelative("compose/client/lib/rs"), repoRelative("framework/module/asset"), repoRelative("compose/fixture")],
  },
  {
    // Directory is singular (`stories/compose/algorithm/`) though the story `title`s read "algorithms" (plural) —
    // the id must match the real directory (glob-matched), the titlePrefix documents the human-facing title.
    id: "compose/algorithm",
    titlePrefix: "🏘️compose🧪️algorithms",
    sourceRoots: [repoRelative("compose/dev/algorithm"), repoRelative("compose/client/lib/rs"), repoRelative("compose/fixture")],
    aliases: {
      "@semio-tech/compose-algorithm": "compose/dev/algorithm/js/index.ts",
    },
  },
  {
    id: "framework",
    titlePrefix: "🛠️framework",
    sourceRoots: [repoRelative("framework/product/os/module/renderer/js/react"), repoRelative("framework/js")],
  },
  {
    id: "framework/hosts",
    titlePrefix: "🛠️framework🔌️hosts",
    sourceRoots: [repoRelative("framework/product/os/module/renderer/js/react"), repoRelative("framework/js"), repoRelative("framework/module/surface"), repoRelative("framework/module/editor"), repoRelative("framework/product/os/module/flow/core/rs")],
    aliases: {
      "@semio-tech/framework-renderer-react": "framework/product/os/module/renderer/js/react/index.tsx",
      "@semio-tech/framework-core": "framework/js/index.ts",
    },
  },
  {
    id: "framework/os",
    titlePrefix: "🛠️framework🖥️os",
    sourceRoots: [repoRelative("framework/product/os/module/renderer/js/react"), repoRelative("framework/product/os/module/renderer/wgpu"), repoRelative("framework/js"), repoRelative("framework/product/os/module/plugin/registry"), repoRelative("framework/product/os")],
    aliases: {
      "@semio-tech/framework-renderer-react": "framework/product/os/module/renderer/js/react/index.tsx",
      "@semio-tech/framework-renderer-wgpu": "framework/product/os/module/renderer/wgpu/index.ts",
      "@semio-tech/framework-core": "framework/js/index.ts",
      "/plugin-modules": "framework/product/os/module/dev/js/plugin-modules",
      "/renderer-modules": "framework/product/os/module/dev/js/renderer-modules",
    },
    assets: [
      { kind: "static-dir", route: "/plugin-modules", root: "framework/product/os/module/dev/js/plugin-modules" },
      { kind: "static-dir", route: "/renderer-modules", root: "framework/product/os/module/dev/js/renderer-modules" },
    ],
    vitePlugins: async () => {
      const { playgroundIframeEmbedHeadersPlugin } = await import("../🧰️framework/🔨️module/🖱️ui/🎨️styling/⚡️implementation/🦀️rust/🟦️vite-elements-assets.ts");
      return [playgroundIframeEmbedHeadersPlugin()];
    },
  },
  {
    id: "infinite",
    titlePrefix: "♾️infinite",
    sourceRoots: [repoRelative("framework/product/os/module/infinite/canvas/react-renderer"), repoRelative("framework/product/os/module/infinite/world/r3f"), repoRelative("framework/product/os/module/infinite/fixture")],
    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "framework/product/os/module/infinite/canvas/react-renderer/index.tsx",
    },
  },
  {
    id: "cad",
    titlePrefix: "📐️cad",
    sourceRoots: [repoRelative("s/plugin/cad/module/renderer"), repoRelative("s/plugin/cad/asset"), repoRelative("s/plugin/cad/fixture")],
  },
  {
    id: "coda",
    titlePrefix: "🧠️coda",
    sourceRoots: [repoRelative("compose/client/ui/desktop")],
    aliases: {
      "@semio-tech/coda-desktop/renderer": "compose/client/ui/desktop/js/renderer.tsx",
    },
  },
  {
    id: "animate",
    titlePrefix: "🎬️animate",
    sourceRoots: [repoRelative("s/plugin/animate/app/present/js/renderer/react")],
  },
];
// #endregion 🔖️ScopeRegistry

// #region 🔖️ScopeResolution
/** @emoji 🎯️ A scope token matches a registered scope's id or any of its descendants (`compose` matches `compose/ui`). */
function scopeTokenMatches(token: string, scopeId: string): boolean {
  return scopeId === token || scopeId.startsWith(`${token}/`);
}

/** @emoji 🧵️ Parses `STORYBOOK_SCOPE` (comma-separated scope ids/prefixes) into the active `StoryScope[]`. Empty → every scope. Throws listing registered ids on an unknown token. */
export function resolveActiveScopes(expr: string): StoryScope[] {
  const tokens = expr
    .split(",")
    .map((t) => t.trim())
    .filter(Boolean);
  if (tokens.length === 0) return [...STORY_SCOPES];
  const known = new Set(STORY_SCOPES.map((s) => s.id));
  for (const token of tokens) {
    if (!STORY_SCOPES.some((s) => scopeTokenMatches(token, s.id))) {
      throw new Error(`[storybook] unknown scope ${JSON.stringify(token)}. Registered scopes: ${Array.from(known).join(", ")}`);
    }
  }
  return STORY_SCOPES.filter((s) => tokens.some((token) => scopeTokenMatches(token, s.id)));
}

/** @emoji 📖️ One story glob per active scope id (parent globs already subsume children via `**`, so a de-duplicated top-level set suffices). */
export function buildScopeStoryGlobs(activeScopes: readonly StoryScope[]): string[] {
  const ids = activeScopes.map((s) => s.id);
  const topLevel = ids.filter((id) => !ids.some((other) => other !== id && scopeTokenMatches(other, id)));
  return topLevel.map((id) => `./stories/${id}/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`);
}

/** @emoji 🔗️ Merges auto-derived workspace aliases with each active scope's irregular `aliases`. Throws on a key registered with two different values (config-time conflict, not silent last-wins). */
export function buildScopeAliases(activeScopes: readonly StoryScope[], workspaceAliases: Readonly<Record<string, string>>): Record<string, string> {
  const alias: Record<string, string> = { ...workspaceAliases };
  for (const scope of activeScopes) {
    for (const [key, value] of Object.entries(scope.aliases ?? {})) {
      if (alias[key] !== undefined && alias[key] !== value) {
        throw new Error(`[storybook] alias conflict for ${JSON.stringify(key)}: ${JSON.stringify(alias[key])} (workspace/earlier scope) vs ${JSON.stringify(value)} (scope ${JSON.stringify(scope.id)})`);
      }
      alias[key] = value;
    }
  }
  return alias;
}

/** @emoji 👁️ Watch-ignore inversion: ignore every source root NOT owned by an active scope, so adding a scope automatically shrinks/grows everyone's ignore set correctly. */
export function buildScopeWatchIgnores(activeScopes: readonly StoryScope[]): string[] {
  const activeRoots = new Set(activeScopes.flatMap((s) => s.sourceRoots));
  const allRoots = new Set(STORY_SCOPES.flatMap((s) => s.sourceRoots));
  const inactiveRoots = Array.from(allRoots).filter((root) => !activeRoots.has(root));
  return inactiveRoots.map((root) => `**/${root}/**`);
}

/** @emoji 🎯️ True when `prefix` is (a prefix of) an active scope id — used by `preview.tsx` to gate scope-specific behavior via `__STORYBOOK_ACTIVE_SCOPES__`. */
export function scopeActive(activeScopeIds: readonly string[], prefix: string): boolean {
  return activeScopeIds.some((id) => scopeTokenMatches(prefix, id));
}
// #endregion 🔖️ScopeResolution

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("resolveActiveScopes", () => {
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
  });

  describe("buildScopeAliases", () => {
    it("merges workspace and scope aliases without conflict", () => {
      const aliases = buildScopeAliases(resolveActiveScopes("ui"), { "@semio-tech/ui-react": "framework/module/ui/js/react" });
      expect(aliases["@semio-tech/ui-react"]).toBe("framework/module/ui/js/react");
      expect(aliases["@elements/ui/globals.css"]).toBe("framework/module/ui/js/react/globals.css");
    });

    it("throws on a genuine key conflict between scopes", () => {
      expect(() =>
        buildScopeAliases(
          [
            { id: "a", titlePrefix: "a", sourceRoots: [], aliases: { x: "1" } },
            { id: "b", titlePrefix: "b", sourceRoots: [], aliases: { x: "2" } },
          ],
          {},
        ),
      ).toThrow(/alias conflict/);
    });
  });

  describe("buildScopeWatchIgnores", () => {
    it("ignores inactive scopes' source roots", () => {
      const ignores = buildScopeWatchIgnores(resolveActiveScopes("ui"));
      expect(ignores).toContain("**/compose/client/lib/js/**");
      expect(ignores.some((g) => g.includes("framework/module/ui/js/react"))).toBe(false);
    });

    it("ignores nothing when every scope is active", () => {
      expect(buildScopeWatchIgnores(resolveActiveScopes(""))).toEqual([]);
    });
  });

  describe("scopeActive", () => {
    it("matches an active scope's own prefix and ancestors", () => {
      const ids = resolveActiveScopes("compose/algorithm").map((s) => s.id);
      expect(scopeActive(ids, "compose")).toBe(true);
      expect(scopeActive(ids, "compose/algorithm")).toBe(true);
      expect(scopeActive(ids, "ui")).toBe(false);
    });
  });
}
