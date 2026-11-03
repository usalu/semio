/** 🔗️ `change-conn-action-torsion-k` wire twin: the leaf payload `ChangeConnActionTorsionK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionTorsionK {
  mutation: "changeConnActionTorsionK";
  connectionId: string;
  actionId: string;
  newTorsionK: number;
}

export const parseChangeConnActionTorsionK: NormWireReader<ChangeConnActionTorsionK> = normWireObject<ChangeConnActionTorsionK>({ mutation: normWireRequired(normWireLiteral("changeConnActionTorsionK")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newTorsionK: normWireRequired(normWireNumber) });
