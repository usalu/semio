import { runToolSyncV1, runToolV1, type ToolRunOptionsV1, type ToolRunResultV1 } from "../🏃️run/🟦️.ts";

/** 🌿️ What a git query can be told: where to run it and the run options of the tool interface. */
export type GitOptionsV1 = Omit<ToolRunOptionsV1, "capture" | "check">;

/** 🌱️ Runs git and answers the trimmed standard output, or undefined when git fails or is missing: the one-line query of `rev-parse`, `status --porcelain` and friends. */
export function gitOutputSyncV1(args: readonly string[], options: GitOptionsV1 = {}): string | undefined {
  try {
    const result = runToolSyncV1("git", args, options);
    return result.status === 0 ? result.stdout.trim() : undefined;
  } catch { return undefined; }
}

/** 📜️ Runs git synchronously and answers the whole outcome without throwing on a non-zero status. */
export function gitSyncV1(args: readonly string[], options: GitOptionsV1 = {}): ReturnType<typeof runToolSyncV1> {
  return runToolSyncV1("git", args, options);
}

/** 🌳️ Runs git asynchronously with captured output; a non-zero status is part of the result. */
export function gitV1(args: readonly string[], options: GitOptionsV1 = {}): Promise<ToolRunResultV1> {
  return runToolV1("git", args, { ...options, capture: true, check: false });
}
