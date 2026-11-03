/** ⛰️ `change-slope-height` wire twin: the leaf payload `ChangeSlopeHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeHeight {
  mutation: "changeSlopeHeight";
  id: string;
  newHeight: number;
}

export const parseChangeSlopeHeight: NormWireReader<ChangeSlopeHeight> = normWireObject<ChangeSlopeHeight>({ mutation: normWireRequired(normWireLiteral("changeSlopeHeight")), id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });
