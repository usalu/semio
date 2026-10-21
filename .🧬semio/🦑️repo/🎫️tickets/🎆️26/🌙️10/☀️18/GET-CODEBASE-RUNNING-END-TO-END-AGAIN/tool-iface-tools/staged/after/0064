import { findToolV1 } from "../📖️registry/🟦️.ts";
import { toolEnvironmentV1 } from "../🌿️environment/🟦️.ts";
import { runToolV1, type ToolRunOptionsV1, type ToolRunResultV1 } from "../🏃️run/🟦️.ts";

/** 🎭️ Runs the Playwright command line (install, test, codegen) under machine capacity admission; a failure throws unless `check: false`. */
export function playwrightV1(args: readonly string[], options: ToolRunOptionsV1 = {}): Promise<ToolRunResultV1> {
  return runToolV1("playwright", args, { heavy: true, ...options });
}

/** 📦️ Loads the Playwright library behind the adapter: the browser cache is the repository's shared `tools/ms-playwright` (the registry's cache directory for the tool) unless the caller already chose one, so a run never falls back to a stale user-global browser. The module specifier comes from the build plan; the caller types the result. */
export async function loadPlaywrightV1(root: string, specifier = "playwright", environment: NodeJS.ProcessEnv = process.env): Promise<any> {
  const fixed = toolEnvironmentV1(findToolV1("playwright")!, {}, {}, { root });
  environment.PLAYWRIGHT_BROWSERS_PATH ??= fixed.PLAYWRIGHT_BROWSERS_PATH;
  return import(specifier);
}
