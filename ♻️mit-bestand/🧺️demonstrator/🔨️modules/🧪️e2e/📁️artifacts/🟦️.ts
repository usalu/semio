import { join } from "node:path";
import policy from "./🔣️.json" with { type: "json" };

/** 🗃️ Keeps the complete service UUID in a short, portable artifact directory. */
export function demonstratorE2eOutputDirectory(root: string, ticket: string | undefined, sessionId: string): string {
  if (!/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/.test(sessionId)) throw new Error("Invalid Demonstrator E2E service UUID");
  const output = join(ticket ? join(ticket, "🗑️generated", policy.namespace) : join(root, "dist/reports", policy.namespace), sessionId.replaceAll("-", ""));
  if (output.length + policy.reservedSuffixChars > policy.maxPathChars) throw new Error(`Demonstrator E2E artifact root exceeds ${policy.maxPathChars - policy.reservedSuffixChars} characters: ${output}`);
  return output;
}

/** 📏️ Checks public runner paths before a test records its diagnostics. */
export function assertDemonstratorE2eArtifactPath(path: string): void {
  if (path.length > policy.maxPathChars) throw new Error(`Demonstrator E2E artifact path exceeds ${policy.maxPathChars} characters: ${path}`);
}
