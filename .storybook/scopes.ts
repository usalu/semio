// #region 🧲️Header
// 💻️ .storybook/scopes.ts
// Specs: Single source of truth for the root Storybook's composable scope system.
// Summary: Replaces the hardcoded `ui`/`puzzle`/`framework` stacks in `main.ts` with a data-driven
// registry every consumer derives from: story globs, workspace aliases, watch-ignores, static-dir
// assets (e.g. `/plugin-modules` for OS-shell program boot stories), and lazy scope-gated Vite plugins.
// `STORY_SCOPES` = `HAND_CURATED_SCOPES` (scopes needing aliases/assets/vitePlugins, a cross-owner
// sourceRoot, or living under an area not yet migrated to `📦️packages`) + `GENERATED_SCOPES` (every
// package that opts into Storybook coverage via its own manifest, derived from the repo-lib package
// catalog — see `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`).
// Pure data + pure functions only — no Vite/Storybook imports (only `node:fs`-backed catalog discovery)
// — so `script.ts` (bun, CLI validation) and `playwright.config.ts` can import it without dragging in
// the Vite/MDX module graph.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { discoverPackages, loadTaxonomy, readSemioMarkerSubTable } from "../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import type { PlaygroundAssetSpec } from "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds";
import type { OwnedBuildPlugin } from "../🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️build-tooling.ts";

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
  readonly vitePlugins?: () => Promise<OwnedBuildPlugin[]>;
  /**
   * Explicit story glob(s) for this scope, overriding the default `./stories/<id>/**` derivation —
   * added for `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE` W7 story co-location, where a scope's
   * stories live beside their component (`🧱️elements/<Element>/🧪️.story.tsx`, which doesn't match a
   * `*.stories.*` glob) rather than under `.storybook/stories/<id>/`. List every glob this scope needs,
   * including the legacy `./stories/<id>/**` one if any stories still live there.
   */
  readonly storyGlobs?: readonly string[];
};
// #endregion 🔖️ScopeModel

// #region 🔖️ScopeRegistry
const repoRelative = (path: string) => path;

/**
 * @emoji 🗂️ Every HAND-CURATED Storybook scope — scopes that cannot (yet) be derived from a package's
 * own opt-in (custom `aliases`/`assets`/`vitePlugins`, a cross-owner `sourceRoots` entry, more than one
 * scope per package, or an owner under an area that hasn't migrated to `📦️packages` yet — `framework`,
 * `infinite`). Add a row here + `stories/<id>/` for those; everything else should
 * prefer the package-catalog opt-in below (`GENERATED_SCOPES`) instead of a new row here — see
 * `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`.
 */
export const HAND_CURATED_SCOPES: readonly StoryScope[] = [
  {
    id: "ui",
    titlePrefix: "🖱️ui⚛️react",
    sourceRoots: [
      repoRelative("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react"),
      repoRelative("🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript"),
      repoRelative("🧰️framework/🔨️modules/🖱️ui/🖼️assets"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript"),
    ],
    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx",
      "@elements/ui/globals.css": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css",
    },
    // 🎫️ 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE W7: most stories moved to co-locate with their
    // component (🧱️elements/<Element>/🧪️.story.tsx); the legacy glob stays for stories whose component
    // is still barrel-inline (not yet extracted) or whose target element dir is already occupied by
    // another story sharing its name (the fixed single-leaf-filename taxonomy holds one story file per
    // dir — see 📋️w0-status.md's W7 section for the full per-story disposition).
    storyGlobs: ["./stories/ui/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)", "../🧰️framework/🔨️modules/🖱️ui/🧱️elements/**/🧪️.story.tsx"],
  },
  {
    id: "puzzle",
    titlePrefix: "🧩️puzzle",
    sourceRoots: [],
  },
  {
    id: "puzzle/2d",
    titlePrefix: "🧩️puzzle🩻️2d",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻️2d")],
  },
  {
    id: "puzzle/3d",
    titlePrefix: "🧩️puzzle🧊️3d",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d"), repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript")],
  },
  {
    id: "puzzle/5d",
    titlePrefix: "🧩️puzzle🕐️5d",
    sourceRoots: [repoRelative("✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d")],
  },
  {
    id: "framework",
    titlePrefix: "🛠️framework",
    sourceRoots: [repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react"), repoRelative("🧰️framework/📦️packages/🟦️typescript")],
  },
  {
    id: "framework/hosts",
    titlePrefix: "🛠️framework🔌️hosts",
    sourceRoots: [
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react"),
      repoRelative("🧰️framework/📦️packages/🟦️typescript"),
      repoRelative("🧰️framework/🔨️modules/🗺️surface"),
      repoRelative("🧰️framework/🔨️modules/✍️editor"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust"),
    ],
    aliases: {
      "@semio-tech/framework-renderer-react": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
      "@semio-tech/framework": "🧰️framework/📦️packages/🟦️typescript/🟦️.ts",
    },
  },
  {
    id: "framework/os",
    titlePrefix: "🛠️framework🖥️os",
    sourceRoots: [
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust"),
      repoRelative("🧰️framework/📦️packages/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"),
      repoRelative("🧰️framework/🛍️products/💻️os"),
    ],
    aliases: {
      "@semio-tech/framework-renderer-react": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
      "@semio-tech/framework-renderer-wgpu": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer-boot/🟦️.ts",
      "@semio-tech/framework": "🧰️framework/📦️packages/🟦️typescript/🟦️.ts",
      "/plugin-modules": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules",
      "/renderer-modules": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📺️renderer-modules",
    },
    assets: [
      { kind: "static-dir", route: "/plugin-modules", root: "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules" },
      { kind: "static-dir", route: "/renderer-modules", root: "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📺️renderer-modules" },
    ],
    vitePlugins: async () => {
      const { playgroundIframeEmbedHeadersPlugin } = await import(/* @vite-ignore */ "../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts");
      return [playgroundIframeEmbedHeadersPlugin()];
    },
  },
  {
    id: "infinite",
    titlePrefix: "♾️infinite",
    sourceRoots: [
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript"),
      repoRelative("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures"),
    ],
    aliases: {
      "@semio-tech/infinite-canvas-react-renderer": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx",
    },
  }
];
// #endregion 🔖️ScopeRegistry

// #region 🔖️GeneratedScopes
/**
 * @emoji 🏷️ Opt-in Storybook coverage a package declares in its OWN manifest — rust
 * `[package.metadata.semio.storybook]`, TS `package.json`'s `"semio": {"storybook": {...}}` (read via
 * `readSemioMarkerSubTable`, the generic per-package opt-in mechanism in the shared repo-lib discovery
 * module). `sourceRoots`/`storyGlobs` entries are OWNER-relative (joined against the discovered
 * package's `ownerRel`; `"."` denotes the owner root itself) so a scope survives its owning package's
 * directory moving without a manifest edit — only `📦️packages/…`'s own manifest and this generator need
 * updating, never a literal full path here. Deliberately minimal (id/titlePrefix/sourceRoots/storyGlobs
 * only, one scope per package, no `aliases`/`assets`/`vitePlugins`) — a package needing more than this
 * stays a `HAND_CURATED_SCOPES` entry instead. See
 * `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`.
 */
type StorybookOptIn = {
  readonly id?: string;
  readonly titlePrefix?: string;
  readonly sourceRoots?: readonly string[];
  readonly storyGlobs?: readonly string[];
};

/** @emoji 🧹️ Narrows the untyped `Record<string, unknown>` `readSemioMarkerSubTable` returns down to the fields `StorybookOptIn` actually understands, dropping anything else silently (a package's opt-in table is its own manifest's business — this generator only reads what it needs). */
function coerceStorybookOptIn(raw: Record<string, unknown> | undefined): StorybookOptIn | undefined {
  if (!raw) return undefined;
  const strings = (value: unknown): readonly string[] | undefined => (Array.isArray(value) ? value.filter((v): v is string => typeof v === "string") : undefined);
  return {
    id: typeof raw.id === "string" ? raw.id : undefined,
    titlePrefix: typeof raw.titlePrefix === "string" ? raw.titlePrefix : undefined,
    sourceRoots: strings(raw.sourceRoots),
    storyGlobs: strings(raw.storyGlobs),
  };
}

/** @emoji 📖️ Every package-catalog-derived scope: walks `discoverPackages(repoRoot)`, keeps only packages whose manifest opts into `storybook`, and resolves each declared owner-relative `sourceRoots`/`storyGlobs` entry against that package's real, freshly-discovered `ownerRel` — never a literal path baked in here. */
export function buildGeneratedScopes(repoRoot: string): readonly StoryScope[] {
  const taxonomy = loadTaxonomy();
  const scopes: StoryScope[] = [];
  for (const pkg of discoverPackages(repoRoot, taxonomy)) {
    const optIn = coerceStorybookOptIn(readSemioMarkerSubTable(join(repoRoot, pkg.manifestPath), pkg.lang, "storybook", taxonomy));
    if (!optIn) continue;
    if (!optIn.sourceRoots || optIn.sourceRoots.length === 0) {
      throw new Error(`[storybook] "${pkg.manifestPath}" opts into Storybook coverage but declares no sourceRoots.`);
    }
    const id = optIn.id ?? pkg.id;
    scopes.push({
      id,
      titlePrefix: optIn.titlePrefix ?? id,
      sourceRoots: optIn.sourceRoots.map((root) => join(pkg.ownerRel, root).replaceAll("\\", "/")),
      ...(optIn.storyGlobs ? { storyGlobs: optIn.storyGlobs.map((glob) => join(pkg.ownerRel, glob).replaceAll("\\", "/")) } : {}),
    });
  }
  return scopes;
}

const HERE = dirname(fileURLToPath(import.meta.url));

/** @emoji 🏠️ `.storybook/scopes.ts` sits directly under the repo root — one `resolve(..)` up from `HERE`. */
export function repoRootFromHere(): string {
  return resolve(HERE, "..");
}

/** @emoji 🗂️ Every package-catalog-derived scope, resolved against the real on-disk repo root. */
export const GENERATED_SCOPES: readonly StoryScope[] = buildGeneratedScopes(repoRootFromHere());
// #endregion 🔖️GeneratedScopes

// #region 🔖️ScopeMerge
/** @emoji 🗂️ Every registered Storybook scope: `HAND_CURATED_SCOPES` plus every package-catalog opt-in (`GENERATED_SCOPES`). Throws on an id collision (config-time conflict, never silent last-wins — same discipline as `buildScopeAliases`). */
export const STORY_SCOPES: readonly StoryScope[] = (() => {
  const merged: StoryScope[] = [...HAND_CURATED_SCOPES];
  const seenIds = new Set(merged.map((s) => s.id));
  for (const scope of GENERATED_SCOPES) {
    if (seenIds.has(scope.id)) throw new Error(`[storybook] generated scope id ${JSON.stringify(scope.id)} collides with a HAND_CURATED_SCOPES entry — rename one.`);
    seenIds.add(scope.id);
    merged.push(scope);
  }
  return merged;
})();
// #endregion 🔖️ScopeMerge

// #region 🔖️ScopeResolution
/** @emoji 🎯️ A scope token matches a registered scope's id or any of its descendants (`puzzle` matches `puzzle/2d`). */
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

/** @emoji 📖️ One or more story globs per active scope (parent globs already subsume children via `**`, so a de-duplicated top-level set suffices). A scope with `storyGlobs` set uses those verbatim instead of the default `./stories/<id>/**` derivation — see `StoryScope.storyGlobs`. */
export function buildScopeStoryGlobs(activeScopes: readonly StoryScope[]): string[] {
  const ids = activeScopes.map((s) => s.id);
  const topLevel = activeScopes.filter((s) => !ids.some((other) => other !== s.id && scopeTokenMatches(other, s.id)));
  return topLevel.flatMap((s) => s.storyGlobs ?? [`./stories/${s.id}/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`]);
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
  });

  describe("buildScopeAliases", () => {
    it("merges workspace and scope aliases without conflict", () => {
      const aliases = buildScopeAliases(resolveActiveScopes("ui"), { "@semio-tech/ui-react": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react" });
      expect(aliases["@semio-tech/ui-react"]).toBe("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react");
      expect(aliases["@elements/ui/globals.css"]).toBe("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css");
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
      expect(ignores).toContain("**/✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻️2d/**");
      expect(ignores.some((g) => g.includes("🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react"))).toBe(false);
    });

    it("ignores nothing when every scope is active", () => {
      expect(buildScopeWatchIgnores(resolveActiveScopes(""))).toEqual([]);
    });
  });

  describe("scopeActive", () => {
    it("matches an active scope's own prefix and ancestors", () => {
      const ids = resolveActiveScopes("puzzle/2d").map((s) => s.id);
      expect(scopeActive(ids, "puzzle")).toBe(true);
      expect(scopeActive(ids, "puzzle/2d")).toBe(true);
      expect(scopeActive(ids, "ui")).toBe(false);
    });
  });
}
