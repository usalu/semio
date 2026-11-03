/** ⛰️ `change-slope-circle-centre-z` wire twin: the leaf payload `ChangeSlopeCircleCentreZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeCircleCentreZ {
  mutation: "changeSlopeCircleCentreZ";
  id: string;
  newCircleCentreZ: number;
}

export const parseChangeSlopeCircleCentreZ: NormWireReader<ChangeSlopeCircleCentreZ> = normWireObject<ChangeSlopeCircleCentreZ>({ mutation: normWireRequired(normWireLiteral("changeSlopeCircleCentreZ")), id: normWireRequired(normWireString), newCircleCentreZ: normWireRequired(normWireNumber) });
