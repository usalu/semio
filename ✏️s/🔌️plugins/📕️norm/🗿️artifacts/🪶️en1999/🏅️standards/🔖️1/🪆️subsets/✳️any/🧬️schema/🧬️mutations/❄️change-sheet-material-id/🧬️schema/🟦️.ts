/** ❄️ `change-sheet-material-id` wire twin: the leaf payload `ChangeSheetMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetMaterialId {
  mutation: "changeSheetMaterialId";
  coldFormedId: string;
  newMaterialId: string;
}

export const parseChangeSheetMaterialId: NormWireReader<ChangeSheetMaterialId> = normWireObject<ChangeSheetMaterialId>({ mutation: normWireRequired(normWireLiteral("changeSheetMaterialId")), coldFormedId: normWireRequired(normWireString), newMaterialId: normWireRequired(normWireString) });
