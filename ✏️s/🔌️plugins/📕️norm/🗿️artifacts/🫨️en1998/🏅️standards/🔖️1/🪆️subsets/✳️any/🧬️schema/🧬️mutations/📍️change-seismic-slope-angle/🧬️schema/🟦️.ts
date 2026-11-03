/** 📍️ `change-seismic-slope-angle` wire twin: the leaf payload `ChangeSeismicSlopeAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeAngle {
  mutation: "changeSeismicSlopeAngle";
  id: string;
  newAngle: number;
}

export const parseChangeSeismicSlopeAngle: NormWireReader<ChangeSeismicSlopeAngle> = normWireObject<ChangeSeismicSlopeAngle>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeAngle")), id: normWireRequired(normWireString), newAngle: normWireRequired(normWireNumber) });
