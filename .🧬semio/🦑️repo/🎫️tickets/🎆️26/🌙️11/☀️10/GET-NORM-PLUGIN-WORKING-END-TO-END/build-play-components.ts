#!/usr/bin/env repo
import {resolve} from "node:path";
const root = resolve(import.meta.dirname, "../../../../../../..");
const framework = resolve(root, "🧰️framework");
const {BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain} = await import(resolve(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const {scriptCompilerAdmissionControlV1} = await import(resolve(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts"));
const {repositoryCargoNetworkV1} = await import(resolve(framework, "🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts"));
const {buildComponentClosureV1} = await import(resolve(framework, "🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"));
class StandardsBuild extends BundleScript {
  async run(args: string[]) {
    const components = new Set(["norm", "norm-energy", "norm-eurocode", "norm-eurocode-structural", "norm-eurocode-materials"]);
    const selected = args.length ? args : [...components];
    if (selected.some(name => !components.has(name))) throw Error("Unknown Standards component");
    const result = await buildComponentClosureV1(root, new Set(selected), {
      invocation: this.invocation,
      facet: "browser",
      network: repositoryCargoNetworkV1(this.invocation, root),
      control: scriptCompilerAdmissionControlV1(this.invocation, "component:driver", 0),
    });
    console.log(result.report);
  }
}
const router = new ScriptRouter(import.meta.dirname).register("build", StandardsBuild);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, {invocation: original}));
