
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { join } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { brepExtensionRetirementOracle, geometryInferenceOracle, channelIdentityOracle } from "../../🧪️tests/🔬️extension-guest-standalone/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runBun } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { packageAdmittedExtensionComponentV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runCanonicalArchitectureScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "original-tessellation-source") {
      if (segments.length !== 1) throw new Error("original-tessellation-source takes no arguments");
      runBun(["test", "../../📐️tessellate/🧪️tests/🟦️.ts"], import.meta.dir);
      console.log(`geometry-inference-oracle cases=${geometryInferenceOracle(import.meta.dir)}`);
      return;
    }
    console.log(`brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(import.meta.dir)}`);
    console.log(`geometry-inference-oracle cases=${geometryInferenceOracle(import.meta.dir)}`);
    console.log(`channel-identity-oracle cases=${channelIdentityOracle(import.meta.dir)}`);
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(["semio-s-plugin-flow-brep", "semio-s-plugin-flow-brep-mesh", "semio-s-plugin-flow-brep-tests"], this.repoRoot, this.invocation, rest);
  }
}

class MeshOracleScript extends BundleScript {
  async run(segments:string[]):Promise<void> {
    if (segments.length) throw new Error("test-mesh-oracle takes no arguments");
    runBun(["test","../../🥽️mesh/🧪️tests/🔬️unit/🟦️.ts"],import.meta.dir);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the build-owned deliverable and accepts no output override");
    await packageAdmittedExtensionComponentV1({ invocation: this.invocation, control: scriptCompilerAdmissionControlV1(this.invocation, "component:package", 0), componentDir: join(import.meta.dir, "../../🧩️component/📦️packages/🦀️rust"), repoRoot: this.repoRoot });
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCanonicalArchitectureScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalArchitectureScript).register("test", TestScript).register("test-mesh-oracle", MeshOracleScript).register("package", PackageScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
