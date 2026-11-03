/** ❄️ `change-sheet-action-gk-line` wire twin: the leaf payload `ChangeSheetActionGKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionGKLine {
  mutation: "changeSheetActionGKLine";
  coldFormedId: string;
  actionId: string;
  newGKLine: number;
}

export const parseChangeSheetActionGKLine: NormWireReader<ChangeSheetActionGKLine> = normWireObject<ChangeSheetActionGKLine>({ mutation: normWireRequired(normWireLiteral("changeSheetActionGKLine")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newGKLine: normWireRequired(normWireNumber) });
