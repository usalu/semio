/** 📍️ `change-seismic-slope-topographic-factor` wire twin: the leaf payload `ChangeSeismicSlopeTopographicFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeTopographicFactor {
  mutation: "changeSeismicSlopeTopographicFactor";
  id: string;
  newTopographicAmplification: number;
}

export const parseChangeSeismicSlopeTopographicFactor: NormWireReader<ChangeSeismicSlopeTopographicFactor> = normWireObject<ChangeSeismicSlopeTopographicFactor>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeTopographicFactor")), id: normWireRequired(normWireString), newTopographicAmplification: normWireRequired(normWireNumber) });
