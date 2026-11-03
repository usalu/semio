/** ❄️ `change-sheet-action-qk-line` wire twin: the leaf payload `ChangeSheetActionQKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionQKLine {
  mutation: "changeSheetActionQKLine";
  coldFormedId: string;
  actionId: string;
  newQKLine: number;
}

export const parseChangeSheetActionQKLine: NormWireReader<ChangeSheetActionQKLine> = normWireObject<ChangeSheetActionQKLine>({ mutation: normWireRequired(normWireLiteral("changeSheetActionQKLine")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newQKLine: normWireRequired(normWireNumber) });
