/** 🔺️ `En1990Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ArtifactChild, parseArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface En1990Diff {
  /** @state artifact */
  gK: number | null;
  /** @state artifact */
  qK: ArtifactChild | null;
  /** @state artifact */
  resistanceKn: number | null;
  /** @state artifact */
  consequenceClass: number | null;
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  seismicAEdKn: number | null;
}

export const parseEn1990Diff: NormWireReader<En1990Diff> = normWireObject<En1990Diff>({ gK: normWireDefault(normWireNullable(normWireNumber), () => null), qK: normWireDefault(normWireNullable(parseArtifactChild), () => null), resistanceKn: normWireDefault(normWireNullable(normWireNumber), () => null), consequenceClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), seismicAEdKn: normWireDefault(normWireNullable(normWireNumber), () => null) });
