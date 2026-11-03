/** 🫙 `change-shell-action-torsion-k` wire twin: the leaf payload `ChangeShellActionTorsionK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellActionTorsionK {
  mutation: "changeShellActionTorsionK";
  shellId: string;
  actionId: string;
  newTorsionK: number;
}

export const parseChangeShellActionTorsionK: NormWireReader<ChangeShellActionTorsionK> = normWireObject<ChangeShellActionTorsionK>({ mutation: normWireRequired(normWireLiteral("changeShellActionTorsionK")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newTorsionK: normWireRequired(normWireNumber) });
