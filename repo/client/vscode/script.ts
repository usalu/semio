#!/usr/bin/env bun
/** 🧭 `@semio-tech/repo-vscode` router: `bun ./script.ts <dev|test|test-e2e|build|lint|build-vsix>`. */
import { BundleScript, ScriptRouter, runBunx, runBundleScriptMain } from "../../lib/js/index.ts";

class DevScript extends BundleScript {
  run(): void {
    runBunx(["vite", "build", "--watch"], this.root);
  }
}

/** ⏱️The extension-host Mocha suite (`js/extension.test.ts`) can only run inside the VSCode test harness — no fast in-repo unit split without a second test file (disallowed). See `test-e2e`. */
class TestScript extends BundleScript {
  run(): void {
    console.log("[test] @semio-tech/repo-vscode has no fast unit suite — run `test-e2e` for the extension-host suite.");
  }
}

/** 🖥️Full extension-host Mocha suite; excluded from the default ≤30s `test` budget. */
class TestE2eScript extends BundleScript {
  run(): void {
    runBunx(["vscode-test"], this.root);
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runBunx(["vite", "build"], this.root);
    runBunx(["vite", "build", "--config", "vite.test.config.ts"], this.root);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "eslint.config.ts", "."], this.root);
  }
}

/** 📦Packages the VSIX; nx's `dependsOn: ["build"]` runs the build target first. */
class BuildVsixScript extends BundleScript {
  run(): void {
    runBunx(["vsce", "package", "--no-dependencies", "--out", "repo.vsix"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("test", TestScript)
  .register("test-e2e", TestE2eScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("build-vsix", BuildVsixScript);

await runBundleScriptMain(router, import.meta.url);
