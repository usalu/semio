/** 🗂️ `remove-sheet-profile` wire twin: the leaf payload `RemoveSheetProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSheetProfile {
  mutation: "removeSheetProfile";
  id: string;
}

export const parseRemoveSheetProfile: NormWireReader<RemoveSheetProfile> = normWireObject<RemoveSheetProfile>({ mutation: normWireRequired(normWireLiteral("removeSheetProfile")), id: normWireRequired(normWireString) });
