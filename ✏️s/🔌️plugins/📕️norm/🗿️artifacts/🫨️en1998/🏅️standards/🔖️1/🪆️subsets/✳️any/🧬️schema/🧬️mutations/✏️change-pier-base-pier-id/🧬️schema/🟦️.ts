/** ✏️ `change-pier-base-pier-id` wire twin: the leaf payload `ChangePierBasePierId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBasePierId {
  mutation: "changePierBasePierId";
  id: string;
  newPierId: string;
}

export const parseChangePierBasePierId: NormWireReader<ChangePierBasePierId> = normWireObject<ChangePierBasePierId>({ mutation: normWireRequired(normWireLiteral("changePierBasePierId")), id: normWireRequired(normWireString), newPierId: normWireRequired(normWireString) });
