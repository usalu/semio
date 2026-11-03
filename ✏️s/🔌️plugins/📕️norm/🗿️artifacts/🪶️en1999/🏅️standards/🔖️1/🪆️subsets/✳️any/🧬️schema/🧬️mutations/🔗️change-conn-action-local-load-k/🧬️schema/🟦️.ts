/** 🔗️ `change-conn-action-local-load-k` wire twin: the leaf payload `ChangeConnActionLocalLoadK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionLocalLoadK {
  mutation: "changeConnActionLocalLoadK";
  connectionId: string;
  actionId: string;
  newLocalLoadK: number;
}

export const parseChangeConnActionLocalLoadK: NormWireReader<ChangeConnActionLocalLoadK> = normWireObject<ChangeConnActionLocalLoadK>({ mutation: normWireRequired(normWireLiteral("changeConnActionLocalLoadK")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newLocalLoadK: normWireRequired(normWireNumber) });
