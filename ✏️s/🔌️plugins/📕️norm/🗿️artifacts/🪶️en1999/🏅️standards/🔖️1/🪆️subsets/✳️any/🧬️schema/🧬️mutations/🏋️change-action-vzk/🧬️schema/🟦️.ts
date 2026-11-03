/** 🏋️ `change-action-vzk` wire twin: the leaf payload `ChangeActionVZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionVZK {
  mutation: "changeActionVZK";
  memberId: string;
  actionId: string;
  newVZK: number;
}

export const parseChangeActionVZK: NormWireReader<ChangeActionVZK> = normWireObject<ChangeActionVZK>({ mutation: normWireRequired(normWireLiteral("changeActionVZK")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVZK: normWireRequired(normWireNumber) });
