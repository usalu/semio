/** 🔄 `remove-fatigue-spectrum` wire twin: the leaf payload `RemoveFatigueSpectrum`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueSpectrum {
  mutation: "removeFatigueSpectrum";
  index: number;
}

export const parseRemoveFatigueSpectrum: NormWireReader<RemoveFatigueSpectrum> = normWireObject<RemoveFatigueSpectrum>({ mutation: normWireRequired(normWireLiteral("removeFatigueSpectrum")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
