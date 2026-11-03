/** 📍️ `change-seismic-slope-height` wire twin: the leaf payload `ChangeSeismicSlopeHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeHeight {
  mutation: "changeSeismicSlopeHeight";
  id: string;
  newHeight: number;
}

export const parseChangeSeismicSlopeHeight: NormWireReader<ChangeSeismicSlopeHeight> = normWireObject<ChangeSeismicSlopeHeight>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeHeight")), id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });
