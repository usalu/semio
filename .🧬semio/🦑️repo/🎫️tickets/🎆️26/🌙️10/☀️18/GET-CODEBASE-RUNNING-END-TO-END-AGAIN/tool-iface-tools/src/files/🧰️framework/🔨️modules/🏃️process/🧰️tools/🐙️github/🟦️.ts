import { runToolSyncV1, runToolV1, type ToolRunOptionsV1, type ToolRunResultV1 } from "../🏃️run/🟦️.ts";

/** 🐙️ What a GitHub query can be told: where to run it and the run options of the tool interface. */
export type GitHubOptionsV1 = Omit<ToolRunOptionsV1, "capture" | "check">;

/** 🔎️ Runs the GitHub command line and answers the trimmed standard output, or undefined when it fails or is missing. */
export function ghOutputSyncV1(args: readonly string[], options: GitHubOptionsV1 = {}): string | undefined {
  try {
    const result = runToolSyncV1("gh", args, options);
    return result.status === 0 ? result.stdout.trim() : undefined;
  } catch { return undefined; }
}

/** 📡️ Runs the GitHub command line with captured output; a non-zero status is part of the result. */
export function ghV1(args: readonly string[], options: GitHubOptionsV1 = {}): Promise<ToolRunResultV1> {
  return runToolV1("gh", args, { ...options, capture: true, check: false });
}
