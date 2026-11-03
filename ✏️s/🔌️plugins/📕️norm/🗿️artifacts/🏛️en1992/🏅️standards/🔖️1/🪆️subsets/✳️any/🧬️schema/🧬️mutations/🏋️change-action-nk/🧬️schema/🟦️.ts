/** 🏋️ `change-action-nk` wire twin: the leaf payload `ChangeActionNk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionNk {
  memberId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeActionNk: NormWireReader<ChangeActionNk> = normWireObject<ChangeActionNk>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
