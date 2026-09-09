/** 🔺️ Canonical sparse document delta with explicit null slots. */
import type { DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import type { WiresArtifact } from "../🟦️.ts";

export interface WiresDiff {
  /** @state artifact */
  artifact: WiresArtifact | null;
  /** @state artifact */
  wiresFixture: DslValue;
  /** @state artifact @child kind=s.stdio.semio */
  content: ArtifactChild | null;
  /** @state artifact */
  meta: DslValue;
}
