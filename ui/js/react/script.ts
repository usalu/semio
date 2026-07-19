#!/usr/bin/env bun
/** 🧭 Elements react UI router: `bun ./script.ts <dev|build|lint|test|policy|check-ui-primitives> [args…]`. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { BundleLinter } from "../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, devToolingEnv, resolveTestLevel, runBundleScriptMain, runBunx, runCmd, runVitest } from "../../../repo/lib/js/index.ts";
import { defineLint } from "../../../repo/lib/js/index.ts";

export const policy = defineLint("@semio-tech/ui-react-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

const storybookEnv = (extra: Record<string, string | undefined> = {}) =>
  devToolingEnv({
    WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
    CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    ...extra,
  });

/** This bundle has no local `.storybook` config — its stories live in the root Storybook's `ui` scope, so `dev`/`build` delegate there instead of running a broken standalone instance. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./script.ts", "dev", "storybook", "ui", ...segments], { cwd: this.repoRoot, env: storybookEnv() });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./script.ts", "build", "storybook", ...segments], { cwd: this.repoRoot, env: storybookEnv({ STORYBOOK_SCOPE: "ui" }) });
  }
}

class LintScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "eslint.config.ts", ".", ...segments], this.root, storybookEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root, storybookEnv());
  }
}

//#region 🔖ui-primitives-lint
/** 🧱 Apps compose framework/ui-provided elements only — raw DOM primitives and Storybook `component:` escape hatches
 * are confined to `ui/` and `framework/`. Seeded from a live repo scan on 2026-07-19; a listed file with zero hits
 * is a stale entry (fails), a hit outside this list fails, a hit inside it is allowed. */
export const UI_PRIMITIVES_ALLOWLIST: readonly string[] = [
  ".storybook/compose/algorithm/kit-store/index.tsx",
  ".storybook/stories/compose/algorithm/Cluster.stories.tsx",
  ".storybook/stories/compose/algorithm/CopyAndPaste.stories.tsx",
  ".storybook/stories/compose/algorithm/Delete.stories.tsx",
  ".storybook/stories/compose/algorithm/Drag.stories.tsx",
  ".storybook/stories/compose/algorithm/FindReplaceableTypesInDesigns.stories.tsx",
  ".storybook/stories/compose/algorithm/Flatten.stories.tsx",
  ".storybook/stories/compose/algorithm/KitStore.stories.tsx",
  ".storybook/stories/compose/algorithm/Move.stories.tsx",
  ".storybook/stories/compose/ui/Design.stories.tsx",
  ".storybook/stories/compose/ui/Diagram.stories.tsx",
  ".storybook/stories/compose/ui/Kit.stories.tsx",
  ".storybook/stories/compose/ui/Scene.stories.tsx",
  ".storybook/stories/compose/ui/Type.stories.tsx",
  ".storybook/stories/compose/ui/Vec.stories.tsx",
  ".storybook/stories/compose/ui/Vector.stories.tsx",
  ".storybook/stories/framework/os/os.stories.tsx",
  ".storybook/stories/puzzle/2d/Board.stories.tsx",
  "animate/present/renderer/react/index.tsx",
  "cad/renderer/js/index.tsx",
  "coda/client/bin/assistant/js/mcp-app.tsx",
  "coda/client/ui/desktop/js/renderer.tsx",
  "compose/client/lib/sketchpad/js/boot.tsx",
  "compose/client/ui/3dm/ui/js/index.tsx",
  "infinite/world/r3f/index.tsx",
] as const;

const UI_PRIMITIVES_SKIP_DIRS = new Set(["node_modules", "dist", "target", ".repo", "storybook-static", ".claude", ".git"]);

const UI_PRIMITIVES_EXEMPT_PREFIX = ".storybook/stories/ui/";

const RAW_DOM_PRIMITIVE_RE = /<(button|input|select|form|textarea|dialog|progress|table)\b/;
const RAW_SVG_ICON_RE = /<svg\b/;
const COMPONENT_ESCAPE_HATCH_RE = /component:\s*[A-Z]/;

interface UiPrimitiveHit {
  file: string;
  line: number;
  kind: string;
  text: string;
}

/** 🔍 Walks the repo tree for `.tsx`/`.jsx` raw-DOM-primitive and `component:` escape-hatch hits, keyed by file. */
function collectUiPrimitivesHits(repoRootPath: string): Map<string, UiPrimitiveHit[]> {
  const hitsByFile = new Map<string, UiPrimitiveHit[]>();
  const record = (file: string, hit: UiPrimitiveHit): void => {
    const bucket = hitsByFile.get(file) ?? [];
    bucket.push(hit);
    hitsByFile.set(file, bucket);
  };

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (UI_PRIMITIVES_SKIP_DIRS.has(entry.name)) {
          continue;
        }
        walk(full);
        continue;
      }
      if (!/\.(tsx|jsx)$/.test(entry.name)) {
        continue;
      }
      const rel = full.slice(repoRootPath.length + 1);
      if (rel.startsWith(UI_PRIMITIVES_EXEMPT_PREFIX)) {
        continue;
      }
      const isUiOrFramework = rel.startsWith("ui/") || rel.startsWith("framework/");
      const svgExempt = rel.startsWith("ui/") || (rel.startsWith("framework/") && !rel.startsWith("framework/product/"));
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (!isUiOrFramework && RAW_DOM_PRIMITIVE_RE.test(line)) {
          record(rel, { file: rel, line: i + 1, kind: "raw-dom-primitive", text: line.trim() });
        }
        if (!svgExempt && RAW_SVG_ICON_RE.test(line)) {
          record(rel, { file: rel, line: i + 1, kind: "raw-svg-icon", text: line.trim() });
        }
        if (COMPONENT_ESCAPE_HATCH_RE.test(line)) {
          record(rel, { file: rel, line: i + 1, kind: "component-escape-hatch", text: line.trim() });
        }
      }
    }
  };

  walk(repoRootPath);
  return hitsByFile;
}

/** 🚫 Fails when raw DOM primitives (`<button>`, `<svg>`, …) or `component:` escape hatches leak outside `ui/`/`framework/`,
 * or when the allowlist carries a stale entry for a file that no longer has any hits. */
class CheckUiPrimitivesScript extends BundleScript {
  run(): void {
    const hitsByFile = collectUiPrimitivesHits(this.repoRoot);
    const allowlist = new Set(UI_PRIMITIVES_ALLOWLIST);
    const failures: string[] = [];

    for (const [file, hits] of hitsByFile) {
      if (allowlist.has(file)) {
        continue;
      }
      for (const hit of hits) {
        failures.push(`${hit.file}:${hit.line} [${hit.kind}] ${hit.text}`);
      }
    }
    for (const file of allowlist) {
      if (!hitsByFile.has(file)) {
        failures.push(`${file} [stale-allowlist-entry] listed in UI_PRIMITIVES_ALLOWLIST but has zero hits — remove it`);
      }
    }

    if (failures.length === 0) {
      console.log(`ui/js/react: no UI primitive violations (${allowlist.size} allowlisted files)`);
      return;
    }
    console.error(`ui/js/react: found ${failures.length} UI primitive violation(s):`);
    for (const failure of failures.slice(0, 120)) {
      console.error(`  ${failure}`);
    }
    if (failures.length > 120) {
      console.error(`  … and ${failures.length - 120} more`);
    }
    process.exit(1);
  }
}
//#endregion 🔖ui-primitives-lint

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("test", TestScript)
  .register("typecheck", TypecheckScript)
  .register("check-ui-primitives", CheckUiPrimitivesScript);

await runBundleScriptMain(router, import.meta.url);
