/** 🏋️ `change-action-gk-line` wire twin: the leaf payload `ChangeActionGKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionGKLine {
  mutation: "changeActionGKLine";
  memberId: string;
  actionId: string;
  newGKLine: number;
}

export const parseChangeActionGKLine: NormWireReader<ChangeActionGKLine> = normWireObject<ChangeActionGKLine>({ mutation: normWireRequired(normWireLiteral("changeActionGKLine")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newGKLine: normWireRequired(normWireNumber) });
