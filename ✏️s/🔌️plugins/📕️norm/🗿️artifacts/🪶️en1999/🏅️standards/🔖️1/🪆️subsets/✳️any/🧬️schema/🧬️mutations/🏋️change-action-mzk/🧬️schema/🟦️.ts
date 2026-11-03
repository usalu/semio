/** 🏋️ `change-action-mzk` wire twin: the leaf payload `ChangeActionMZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionMZK {
  mutation: "changeActionMZK";
  memberId: string;
  actionId: string;
  newMZK: number;
}

export const parseChangeActionMZK: NormWireReader<ChangeActionMZK> = normWireObject<ChangeActionMZK>({ mutation: normWireRequired(normWireLiteral("changeActionMZK")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMZK: normWireRequired(normWireNumber) });
