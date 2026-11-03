/** 🏋️ `change-action-vk-punch` wire twin: the leaf payload `ChangeActionVKPunch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionVKPunch {
  memberId: string;
  actionId: string;
  newVKPunch: number;
}

export const parseChangeActionVKPunch: NormWireReader<ChangeActionVKPunch> = normWireObject<ChangeActionVKPunch>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVKPunch: normWireRequired(normWireNumber) });
