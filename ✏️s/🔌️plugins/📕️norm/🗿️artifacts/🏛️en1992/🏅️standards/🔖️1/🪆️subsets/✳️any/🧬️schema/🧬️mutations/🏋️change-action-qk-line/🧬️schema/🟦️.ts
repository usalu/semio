/** 🏋️ `change-action-qk-line` wire twin: the leaf payload `ChangeActionQKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionQKLine {
  memberId: string;
  actionId: string;
  newQKLine: number;
}

export const parseChangeActionQKLine: NormWireReader<ChangeActionQKLine> = normWireObject<ChangeActionQKLine>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newQKLine: normWireRequired(normWireNumber) });
