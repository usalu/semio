/** 🏋️ `change-action-torsion-k` wire twin: the leaf payload `ChangeActionTorsionK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionTorsionK {
  mutation: "changeActionTorsionK";
  memberId: string;
  actionId: string;
  newTorsionK: number;
}

export const parseChangeActionTorsionK: NormWireReader<ChangeActionTorsionK> = normWireObject<ChangeActionTorsionK>({ mutation: normWireRequired(normWireLiteral("changeActionTorsionK")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newTorsionK: normWireRequired(normWireNumber) });
