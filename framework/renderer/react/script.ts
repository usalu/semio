#!/usr/bin/env bun
/** @emoji 🎨 `@semio-tech/framework-renderer-react` task router. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "vitest.config.ts");
  }
}

//#region 🔖LintScript
const REGION_BALANCE_FILES = ["os-shell.tsx", "ui-interpreter.tsx", "index.tsx"] as const;

const HOST_FILE_EXEMPT = new Set<string>([]);

/** 🧭Counts unmatched `//#region` / `//#endregion` markers per file — a typo'd region silently corrupts the file's canonical structure. */
function collectRegionBalanceViolations(root: string): string[] {
  const violations: string[] = [];
  for (const name of REGION_BALANCE_FILES) {
    const text = readFileSync(join(root, name), "utf8");
    const opens = (text.match(/#region\b/g) ?? []).length;
    const closes = (text.match(/#endregion\b/g) ?? []).length;
    if (opens !== closes) {
      violations.push(`${name}: ${opens} #region marker(s) vs ${closes} #endregion marker(s)`);
    }
  }
  return violations;
}

/** 🧭Every `components/*-host.tsx` must export exactly one component matching `XxxHost` — the contract `ui-interpreter.tsx`'s host registry table dispatches against. */
function collectHostSignatureViolations(root: string): string[] {
  const violations: string[] = [];
  const componentsDir = join(root, "components");
  for (const entry of readdirSync(componentsDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".tsx") || HOST_FILE_EXEMPT.has(entry.name)) continue;
    const text = readFileSync(join(componentsDir, entry.name), "utf8");
    const hostExports = [...text.matchAll(/^export function ([A-Z][A-Za-z0-9]*Host)\([^)]*: ComponentSceneHostProps\)/gm)].map((m) => m[1]!);
    if (hostExports.length === 0) {
      violations.push(`components/${entry.name}: no exported component matching XxxHost(...)`);
    } else if (hostExports.length > 1) {
      violations.push(`components/${entry.name}: multiple XxxHost exports (${hostExports.join(", ")})`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = [...collectRegionBalanceViolations(this.root), ...collectHostSignatureViolations(this.root)];
    if (violations.length === 0) {
      console.log("framework-renderer-react: region/host-contract lint passed");
      return;
    }
    console.error(`framework-renderer-react: found ${violations.length} lint violation(s):`);
    for (const v of violations) console.error(`  ${v}`);
    process.exit(1);
  }
}
//#endregion 🔖LintScript

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url);
