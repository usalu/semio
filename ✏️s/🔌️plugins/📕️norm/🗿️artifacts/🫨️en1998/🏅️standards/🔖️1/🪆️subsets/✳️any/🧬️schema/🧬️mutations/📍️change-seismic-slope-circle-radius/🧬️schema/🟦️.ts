/** 📍️ `change-seismic-slope-circle-radius` wire twin: the leaf payload `ChangeSeismicSlopeCircleRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeCircleRadius {
  mutation: "changeSeismicSlopeCircleRadius";
  id: string;
  newCircleRadius: number;
}

export const parseChangeSeismicSlopeCircleRadius: NormWireReader<ChangeSeismicSlopeCircleRadius> = normWireObject<ChangeSeismicSlopeCircleRadius>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeCircleRadius")), id: normWireRequired(normWireString), newCircleRadius: normWireRequired(normWireNumber) });
