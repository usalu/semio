/** 🏋️ `change-action-local-load-k` wire twin: the leaf payload `ChangeActionLocalLoadK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionLocalLoadK {
  mutation: "changeActionLocalLoadK";
  memberId: string;
  actionId: string;
  newLocalLoadK: number;
}

export const parseChangeActionLocalLoadK: NormWireReader<ChangeActionLocalLoadK> = normWireObject<ChangeActionLocalLoadK>({ mutation: normWireRequired(normWireLiteral("changeActionLocalLoadK")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newLocalLoadK: normWireRequired(normWireNumber) });
