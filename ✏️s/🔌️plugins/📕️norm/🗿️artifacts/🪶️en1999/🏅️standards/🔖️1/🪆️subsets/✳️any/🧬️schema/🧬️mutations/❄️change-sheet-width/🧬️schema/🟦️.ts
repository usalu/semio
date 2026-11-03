/** ❄️ `change-sheet-width` wire twin: the leaf payload `ChangeSheetWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetWidth {
  mutation: "changeSheetWidth";
  coldFormedId: string;
  newWidth: number;
}

export const parseChangeSheetWidth: NormWireReader<ChangeSheetWidth> = normWireObject<ChangeSheetWidth>({ mutation: normWireRequired(normWireLiteral("changeSheetWidth")), coldFormedId: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
