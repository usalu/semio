/** 🔺️ `Din16798Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireLiteral, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din16798VentSystem, type Din16798Zone, parseDin16798VentSystem, parseDin16798Zone } from "../📸️snapshot/🟦️.ts";
import { type Din16798Artifact, parseDin16798Artifact } from "../🟦️.ts";

export interface Din16798Diff {
  /** @state artifact */
  artifact: Din16798Artifact | null;
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  thetaRmC: number | null;
  /** @state artifact */
  outdoorCo2Ppm: number | null;
  /** @state artifact */
  zones: { values: Din16798Zone[]; } | null;
  /** @state artifact */
  ventSystems: { values: Din16798VentSystem[]; } | null;
  /** @state artifact */
  envelopeN50HInv: number | null;
  /** @state artifact */
  envelopeVolumeM3: number | null;
  /** @state artifact */
  cellarAreaM2: number | null;
  /** @state artifact */
  cellarVentilationM3H: number | null;
  /** @state artifact */
  nightSetbackK: number | null;
}

export const parseDin16798Diff: NormWireReader<Din16798Diff> = normWireObject<Din16798Diff>({ artifact: normWireDefault(normWireNullable(parseDin16798Artifact), () => null), annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), thetaRmC: normWireDefault(normWireNullable(normWireNumber), () => null), outdoorCo2Ppm: normWireDefault(normWireNullable(normWireNumber), () => null), zones: normWireDefault(normWireNullable(normWireObject<{ values: Din16798Zone[]; }>({ values: normWireRequired(normWireArray(parseDin16798Zone)) })), () => null), ventSystems: normWireDefault(normWireNullable(normWireObject<{ values: Din16798VentSystem[]; }>({ values: normWireRequired(normWireArray(parseDin16798VentSystem)) })), () => null), envelopeN50HInv: normWireDefault(normWireNullable(normWireNumber), () => null), envelopeVolumeM3: normWireDefault(normWireNullable(normWireNumber), () => null), cellarAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), cellarVentilationM3H: normWireDefault(normWireNullable(normWireNumber), () => null), nightSetbackK: normWireDefault(normWireNullable(normWireNumber), () => null) });
