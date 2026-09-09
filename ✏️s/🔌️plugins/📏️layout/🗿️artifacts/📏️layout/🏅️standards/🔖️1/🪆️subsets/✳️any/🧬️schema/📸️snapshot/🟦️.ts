/** 📸️ Layout snapshots are the canonical artifact-lane document projection. */
import { parseLayoutArtifact, type LayoutArtifact } from "../🟦️.ts";
export * from "../🟦️.ts";
export type LayoutSnapshot = LayoutArtifact;

/** 📸️ Parses the same exact persisted fields as the artifact contract. */
export function parseLayoutSnapshot(value: unknown, at = "$" ): LayoutSnapshot {
  return parseLayoutArtifact(value, at);
}
