/** 🔗️ `change-conn-action-nk` wire twin: the leaf payload `ChangeConnActionNK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionNK {
  mutation: "changeConnActionNK";
  connectionId: string;
  actionId: string;
  newNK: number;
}

export const parseChangeConnActionNK: NormWireReader<ChangeConnActionNK> = normWireObject<ChangeConnActionNK>({ mutation: normWireRequired(normWireLiteral("changeConnActionNK")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newNK: normWireRequired(normWireNumber) });
