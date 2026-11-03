/** ❄️ `change-sheet-thickness` wire twin: the leaf payload `ChangeSheetThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetThickness {
  mutation: "changeSheetThickness";
  coldFormedId: string;
  newThickness: number;
}

export const parseChangeSheetThickness: NormWireReader<ChangeSheetThickness> = normWireObject<ChangeSheetThickness>({ mutation: normWireRequired(normWireLiteral("changeSheetThickness")), coldFormedId: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
