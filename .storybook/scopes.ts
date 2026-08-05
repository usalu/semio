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
import type { PlaygroundAssetSpec } from "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";

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
    sourceRoots: [
      repoRelative("🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🔨️modules/🖱️ui/🖼️assets/⚡️implementations/🟦️typescript"),
      repoRelative("✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript"),
    ],
    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript/📦️index.tsx",
      "@elements/ui/globals.css": "🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/🎨️globals.css",
      "@semio-tech/coda-desktop/renderer": "compose/client/ui/desktop/js/renderer.tsx",
      "@semio-tech/compose-rs-wasm": "compose/client/lib/rs/pkg/compose.js",
    },
  },
  {
    id: "styling",
    titlePrefix: "🎨️styling",
    sourceRoots: [repoRelative("🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🟦️typescript")],
  },
  {
    id: "puzzle",
    titlePrefix: "🧩️puzzle",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript")],
  },
  {
    id: "puzzle/2d",
    titlePrefix: "🧩️puzzle🩻️2d",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d"), repoRelative("✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript")],
  },
  {
    id: "puzzle/3d",
    titlePrefix: "🧩️puzzle🧊️3d",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d"), repoRelative("✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript"), repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/⚡️implementations/🟦️typescript")],
  },
  {
    id: "puzzle/5d",
    titlePrefix: "🧩️puzzle🕐️5d",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d"), repoRelative("✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript")],
  },
  {
    id: "block",
    titlePrefix: "🧱️block",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧱️block")],
  },
  {
    id: "compose",
    titlePrefix: "🏘️compose",
    sourceRoots: [repoRelative("compose/client/lib/js"), repoRelative("compose/client/lib/rs"), repoRelative("🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript"), repoRelative("compose/fixture"), repoRelative("compose/dev/algorithm")],
    aliases: {
      "@semio-tech/ui-react/globals.css": "🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/🎨️globals.css",
      "@semio-tech/compose-rs-wasm": "compose/client/lib/rs/pkg/compose.js",
    },
    optimizeDepsExclude: ["@semio-tech/compose-react", "@semio-tech/compose-js", "@semio-tech/assets"],
  },
  {
    id: "compose/ui",
    titlePrefix: "🏘️compose⚛️react",
    sourceRoots: [repoRelative("compose/client/lib/js"), repoRelative("compose/client/lib/rs"), repoRelative("🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript"), repoRelative("compose/fixture")],
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
    sourceRoots: [repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript"), repoRelative("🧰️framework/⚡️implementations/🟦️typescript")],
  },
  {
    id: "framework/hosts",
    titlePrefix: "🛠️framework🔌️hosts",
    sourceRoots: [
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🔨️modules/🗺️surface"),
      repoRelative("🧰️framework/🔨️modules/✍️editor"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/⚡️implementations/🦀️rust"),
    ],
    aliases: {
      "@semio-tech/framework-renderer-react": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx",
      "@semio-tech/framework-core": "🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts",
    },
  },
  {
    id: "framework/os",
    titlePrefix: "🛠️framework🖥️os",
    sourceRoots: [
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust"),
      repoRelative("🧰️framework/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry"),
      repoRelative("🧰️framework/🛍️products/💻️os"),
    ],
    aliases: {
      "@semio-tech/framework-renderer-react": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx",
      "@semio-tech/framework-renderer-wgpu": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust/📦️index.ts",
      "@semio-tech/framework-core": "🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts",
      "/plugin-modules": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/🔌️plugin-modules",
      "/renderer-modules": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📺️renderer-modules",
    },
    assets: [
      { kind: "static-dir", route: "/plugin-modules", root: "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/🔌️plugin-modules" },
      { kind: "static-dir", route: "/renderer-modules", root: "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📺️renderer-modules" },
    ],
    vitePlugins: async () => {
      const { playgroundIframeEmbedHeadersPlugin } = await import(/* @vite-ignore */ "../🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🦀️rust/🟦️vite-elements-assets.ts");
      return [playgroundIframeEmbedHeadersPlugin()];
    },
  },
  {
    id: "infinite",
    titlePrefix: "♾️infinite",
    sourceRoots: [
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/⚡️implementations/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures"),
    ],
    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementations/🟦️typescript/📦️index.tsx",
    },
  },
  {
    id: "cad",
    titlePrefix: "📐️cad",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/📐️cad/🔨️modules/📺️renderer/⚡️implementations/🟦️typescript"), repoRelative("✏️s/🔌️plugins/📐️cad/🖼️assets"), repoRelative("✏️s/🔌️plugins/📐️cad/🧫️fixtures")],
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
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/⚡️implementations/🟦️typescript")],
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
      const aliases = buildScopeAliases(resolveActiveScopes("ui"), { "@semio-tech/ui-react": "🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript" });
      expect(aliases["@semio-tech/ui-react"]).toBe("🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript");
      expect(aliases["@elements/ui/globals.css"]).toBe("🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/🎨️globals.css");
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
      expect(ignores.some((g) => g.includes("🧰️framework/🔨️modules/🖱️ui/⚛️react"))).toBe(false);
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
