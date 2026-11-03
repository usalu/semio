/** 🏋️ `change-action-point-force` wire twin: the leaf payload `ChangeActionPointForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionPointForce {
  memberId: string;
  actionId: string;
  newPointForce: number;
}

export const parseChangeActionPointForce: NormWireReader<ChangeActionPointForce> = normWireObject<ChangeActionPointForce>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newPointForce: normWireRequired(normWireNumber) });
