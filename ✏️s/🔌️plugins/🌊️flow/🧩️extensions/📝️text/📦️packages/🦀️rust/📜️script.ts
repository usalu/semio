#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { join } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { packageAdmittedExtensionComponentV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(["semio-s-plugin-flow-text"], this.repoRoot, this.invocation, rest);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the build-owned deliverable and accepts no output override");
    await packageAdmittedExtensionComponentV1({ invocation: this.invocation, control: scriptCompilerAdmissionControlV1(this.invocation, "component:package", 0), componentDir: join(import.meta.dir, "../../🧩️component/📦️packages/🦀️rust"), repoRoot: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
