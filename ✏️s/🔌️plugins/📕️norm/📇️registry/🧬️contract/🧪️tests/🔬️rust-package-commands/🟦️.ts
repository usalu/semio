import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";

/** 🏃️ Runs the `TestBytePropertyScript` command under the caller's roots and invocation. */
export async function runTestBytePropertyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments:string[]): Promise<void> {if(segments.length)throw Error("Unknown byte property command");await runArtifactRustTests("semio-s-artifact-norm-contract", context.repoRoot, ["--lib","byte_properties_render_their_bounded_localized_extent"], context.invocation);}
