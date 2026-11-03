/** ✏️ `change-bearing-pier-id` wire twin: the leaf payload `ChangeBearingPierId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingPierId {
  mutation: "changeBearingPierId";
  id: string;
  newPierId: string;
}

export const parseChangeBearingPierId: NormWireReader<ChangeBearingPierId> = normWireObject<ChangeBearingPierId>({ mutation: normWireRequired(normWireLiteral("changeBearingPierId")), id: normWireRequired(normWireString), newPierId: normWireRequired(normWireString) });
