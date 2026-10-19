import { receiveScriptProcessInvocation } from "@semio-tech/framework/process/routing";
import { startNativeProgress } from "@semio-tech/framework/process/execution";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "@semio-tech/framework/process/routing";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";
import { runArtifactRustPackageMain } from "@semio-tech/framework/process/cargo-test";


const cargoName = "semio-s-artifact-norm-en1996";

class RegenerateExampleAssetsScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const stop = startNativeProgress(`artifact-rust:${cargoName}:regenerate-example-assets`);
    try {
      await runRepositoryCommand(this.invocation,"cargo",[
          "run",
          "--locked",
          "--manifest-path",
          resolve(this.root, "Cargo.toml"),
          "--bin",
          "regenerate-en1996-example-assets",
          "--quiet",
        ],this.repoRoot,`artifact-rust:${cargoName}:regenerate-example-assets`,{version:1,maximumElapsedMilliseconds:0,output:null});
    } finally {
      stop();
    }
  }
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(packageRoot: string): Promise<void> {
  const command = process.argv[2];
  
  if (command === "regenerate-example-assets") {
    const router = new ScriptRouter(packageRoot).register("regenerate-example-assets", RegenerateExampleAssetsScript);
    await receiveScriptProcessInvocation(process.env, original => (router).run(["regenerate-example-assets"], original));
  } else {
    await runArtifactRustPackageMain(packageRoot, cargoName, {packages:["semio-s-artifact-norm-en1996-checks","semio-s-artifact-norm-en1996-diff","semio-s-artifact-norm-en1996-editor","semio-s-artifact-norm-en1996-examples","semio-s-artifact-norm-en1996-inferences","semio-s-artifact-norm-en1996-io","semio-s-artifact-norm-en1996-model","semio-s-artifact-norm-en1996-mut-concentrated","semio-s-artifact-norm-en1996-mut-eccentricity","semio-s-artifact-norm-en1996-mut-loads","semio-s-artifact-norm-en1996-mut-mortar","semio-s-artifact-norm-en1996-mut-openings","semio-s-artifact-norm-en1996-mut-project","semio-s-artifact-norm-en1996-mut-reinforcement","semio-s-artifact-norm-en1996-mut-units","semio-s-artifact-norm-en1996-mut-walls","semio-s-artifact-norm-en1996-mutations","semio-s-artifact-norm-en1996-schema","semio-s-artifact-norm-en1996-viewer","semio-s-artifact-norm-en1996"],snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
  }
}
