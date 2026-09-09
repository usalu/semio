/** 📸️ CAD persisted snapshot contract. */
import { parseCadArtifact, type CadArtifact } from "../🟦️.ts";
export type { ArtifactChild, CadNode, CadReference, CadReferenceList } from "../🟦️.ts";
export interface CadSnapshot extends CadArtifact {}
/** 🪪️ Parses the persisted snapshot through the same exact document boundary. */
export function parseCadSnapshot(value: unknown, at = "$"): CadSnapshot {
  return parseCadArtifact(value, at);
}
