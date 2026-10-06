#!/usr/bin/env bun
/** 📋️ Forms TypeScript contract and authoring verification. */
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import{runArtifactTypeScriptPackageMain}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
const publicOptions={suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts"]};
class PublicScript extends BundleScript{async run():Promise<void>{await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/forms-js",publicOptions);}}
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/forms-js",publicOptions);

  }
}
const router = new ScriptRouter(import.meta.dir).register("build",PublicScript).register("check",PublicScript).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "test" });
