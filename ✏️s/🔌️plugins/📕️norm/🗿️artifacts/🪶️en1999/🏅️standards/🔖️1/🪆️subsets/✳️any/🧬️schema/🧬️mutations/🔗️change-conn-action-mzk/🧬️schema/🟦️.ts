/** 🔗️ `change-conn-action-mzk` wire twin: the leaf payload `ChangeConnActionMZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionMZK {
  mutation: "changeConnActionMZK";
  connectionId: string;
  actionId: string;
  newMZK: number;
}

export const parseChangeConnActionMZK: NormWireReader<ChangeConnActionMZK> = normWireObject<ChangeConnActionMZK>({ mutation: normWireRequired(normWireLiteral("changeConnActionMZK")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMZK: normWireRequired(normWireNumber) });
