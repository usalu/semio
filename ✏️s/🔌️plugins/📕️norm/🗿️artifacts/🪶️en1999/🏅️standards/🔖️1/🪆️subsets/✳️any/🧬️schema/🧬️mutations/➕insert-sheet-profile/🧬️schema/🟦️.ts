/** 🗂️ `insert-sheet-profile` wire twin: the leaf payload `InsertSheetProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SheetProfile, parseSheetProfile } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSheetProfile {
  mutation: "insertSheetProfile";
  index: number;
  sheetProfile: SheetProfile;
}

export const parseInsertSheetProfile: NormWireReader<InsertSheetProfile> = normWireObject<InsertSheetProfile>({ mutation: normWireRequired(normWireLiteral("insertSheetProfile")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), sheetProfile: normWireRequired(parseSheetProfile) });
