/** ⛰️ `change-slope-circle-radius` wire twin: the leaf payload `ChangeSlopeCircleRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeCircleRadius {
  mutation: "changeSlopeCircleRadius";
  id: string;
  newCircleRadius: number;
}

export const parseChangeSlopeCircleRadius: NormWireReader<ChangeSlopeCircleRadius> = normWireObject<ChangeSlopeCircleRadius>({ mutation: normWireRequired(normWireLiteral("changeSlopeCircleRadius")), id: normWireRequired(normWireString), newCircleRadius: normWireRequired(normWireNumber) });
