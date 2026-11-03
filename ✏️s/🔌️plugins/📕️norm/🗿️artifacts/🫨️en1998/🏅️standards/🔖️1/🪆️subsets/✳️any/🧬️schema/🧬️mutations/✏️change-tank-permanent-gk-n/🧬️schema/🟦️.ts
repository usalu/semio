/** ✏️ `change-tank-permanent-gk-n` wire twin: the leaf payload `ChangeTankPermanentGkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankPermanentGkN {
  mutation: "changeTankPermanentGkN";
  id: string;
  newPermanentGkN: number;
}

export const parseChangeTankPermanentGkN: NormWireReader<ChangeTankPermanentGkN> = normWireObject<ChangeTankPermanentGkN>({ mutation: normWireRequired(normWireLiteral("changeTankPermanentGkN")), id: normWireRequired(normWireString), newPermanentGkN: normWireRequired(normWireNumber) });
