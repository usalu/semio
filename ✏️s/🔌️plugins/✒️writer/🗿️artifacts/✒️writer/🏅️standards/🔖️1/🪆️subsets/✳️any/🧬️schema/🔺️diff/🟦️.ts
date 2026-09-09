/** 🔺️ Writer sparse durable document delta. */
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import type { WriterArtifact } from "../🟦️.ts";

export interface WriterDiff {
  /** @state artifact */
  artifact: WriterArtifact | null;
  /** @state artifact */
  schema: string | null;
  /** @state artifact */
  id: string | null;
  /** @state artifact */
  languageId: string | null;
  /** @state artifact */
  uri: string | null;
  /** @state artifact @child kind=s.stdio.semio */
  document: ArtifactChild | null;
}
