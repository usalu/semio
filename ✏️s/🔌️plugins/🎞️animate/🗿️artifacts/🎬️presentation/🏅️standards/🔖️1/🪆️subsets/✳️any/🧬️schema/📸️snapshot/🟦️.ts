/** 📸️ Presentation snapshot: the shared figure, its tile crops and the shared document child identities. */
import { parsePresentationArtifact, type ArtifactChild, type FigureTileDraft, type FigureTileSource } from "../🟦️.ts";

export interface PresentationSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ source: FigureTileSource;
  /** @state artifact */ tiles: FigureTileDraft[];
  /** @state artifact @child kind=s.stdio.semio */ presentation: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ animation: ArtifactChild;
}

/** 🔎️ Validates a snapshot through the exact Presentation document boundary. */
export function parsePresentationSnapshot(value: unknown, at = "$"): PresentationSnapshot {
  return parsePresentationArtifact(value, at);
}
