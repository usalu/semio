/** ✏️ `change-bridge-permanent-gk-n` wire twin: the leaf payload `ChangeBridgePermanentGkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePermanentGkN {
  mutation: "changeBridgePermanentGkN";
  id: string;
  newPermanentGkN: number;
}

export const parseChangeBridgePermanentGkN: NormWireReader<ChangeBridgePermanentGkN> = normWireObject<ChangeBridgePermanentGkN>({ mutation: normWireRequired(normWireLiteral("changeBridgePermanentGkN")), id: normWireRequired(normWireString), newPermanentGkN: normWireRequired(normWireNumber) });
