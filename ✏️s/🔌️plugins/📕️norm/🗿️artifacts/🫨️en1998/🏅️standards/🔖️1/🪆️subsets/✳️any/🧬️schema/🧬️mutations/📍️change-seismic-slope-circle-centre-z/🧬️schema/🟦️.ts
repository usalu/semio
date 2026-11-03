/** 📍️ `change-seismic-slope-circle-centre-z` wire twin: the leaf payload `ChangeSeismicSlopeCircleCentreZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeCircleCentreZ {
  mutation: "changeSeismicSlopeCircleCentreZ";
  id: string;
  newCircleCentreZ: number;
}

export const parseChangeSeismicSlopeCircleCentreZ: NormWireReader<ChangeSeismicSlopeCircleCentreZ> = normWireObject<ChangeSeismicSlopeCircleCentreZ>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeCircleCentreZ")), id: normWireRequired(normWireString), newCircleCentreZ: normWireRequired(normWireNumber) });
