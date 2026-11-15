import { checkScriptInvocation, type ScriptInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTwoHumanCli } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts";
import journey from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { validateNormBrowserArgumentsV1 } from "./📋️arguments/🟦️.ts";
import { inspectNormEditorV1, inspectNormViewerV1 } from "./🧭️journey/🟦️.ts";

const idsOf = (phase: "editor" | "viewer") => journey.checks.filter(row => row.phase === phase).map(row => row.id);

/** 📕️ Runs the norm checks inside the shared two-human journey: side A edits, side B opens the read-only viewer. */
export async function runNormBrowserAcceptanceV1(invocation: ScriptInvocation, repoRoot: string, segments: readonly string[]): Promise<void> {
  checkScriptInvocation(invocation);
  invocation.control.signal.throwIfAborted();
  validateNormBrowserArgumentsV1(segments);
  await runTwoHumanCli(invocation, repoRoot, "", segments, { required: idsOf("editor"), run: inspectNormEditorV1, viewer: { required: idsOf("viewer"), run: inspectNormViewerV1 } });
}
