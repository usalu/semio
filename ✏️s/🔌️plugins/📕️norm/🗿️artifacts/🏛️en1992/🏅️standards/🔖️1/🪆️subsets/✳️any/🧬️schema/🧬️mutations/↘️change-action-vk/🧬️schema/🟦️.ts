/** ↘️ `change-action-vk` wire twin: the leaf payload `ChangeActionVk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionVk {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeActionVk: NormWireReader<ChangeActionVk> = normWireObject<ChangeActionVk>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
