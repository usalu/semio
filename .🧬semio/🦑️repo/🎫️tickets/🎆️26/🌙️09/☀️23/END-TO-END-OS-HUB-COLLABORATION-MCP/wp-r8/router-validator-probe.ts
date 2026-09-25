import { fixedSourceDispositionDecision } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const content = `import { BundleScript, ScriptRouter, runBundleScriptMain } from "@semio-tech/repo-lib";
import { checkGenerated, generate, previewGenerated } from "./🟦️.ts";

class GenerateScript extends BundleScript {
  async run(): Promise<void> {
    generate();
  }
}

class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    previewGenerated();
  }
}

class CheckScript extends BundleScript {
  async run(): Promise<void> {
    checkGenerated();
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript);

await runBundleScriptMain(router, import.meta.url);
`;
console.log(fixedSourceDispositionDecision("root-script", content)?.role);
