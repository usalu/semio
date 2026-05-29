import { fileURLToPath } from "node:url";
import { runPolicyExit } from "./policy-runner.ts";

/** 🚪When argv contains `policy`, runs this bundle's policy lint and exits. */
export async function dispatchPolicyArgv(segments: string[], scriptUrl: string): Promise<boolean> {
  if (segments[0] !== "policy") return false;
  await runPolicyExit(fileURLToPath(scriptUrl));
  return true;
}
