/** 📸️ Forms persisted snapshot shares the document's exact field contract. */
import { parseFormsArtifact } from "../🟦️.ts";
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface FormsSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  structure: ArtifactChild;
  /** @state artifact */
  results: ArtifactChild;
}

/** 🔎️ Decodes a persisted Forms document with canonical child identities. */
export function parseFormsSnapshot(value: unknown, at = "$"): FormsSnapshot {
  return parseFormsArtifact(value, at);
}
