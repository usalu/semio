/** 🔗️ `change-conn-action-vzk` wire twin: the leaf payload `ChangeConnActionVZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionVZK {
  mutation: "changeConnActionVZK";
  connectionId: string;
  actionId: string;
  newVZK: number;
}

export const parseChangeConnActionVZK: NormWireReader<ChangeConnActionVZK> = normWireObject<ChangeConnActionVZK>({ mutation: normWireRequired(normWireLiteral("changeConnActionVZK")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVZK: normWireRequired(normWireNumber) });
