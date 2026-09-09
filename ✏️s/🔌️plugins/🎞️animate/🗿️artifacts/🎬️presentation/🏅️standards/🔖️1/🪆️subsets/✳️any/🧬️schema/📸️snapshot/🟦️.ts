/** 📸️ Presentation snapshot retains only shared document child identities. */
import { parsePresentationArtifact, type ArtifactChild } from "../🟦️.ts";

export interface PresentationSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio */ presentation: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ animation: ArtifactChild;
}

/** 🔎️ Validates a snapshot through the exact Presentation document boundary. */
export function parsePresentationSnapshot(value: unknown, at = "$"): PresentationSnapshot {
  return parsePresentationArtifact(value, at);
}
