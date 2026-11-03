/** 🗂️ `remove-fatigue-category` wire twin: the leaf payload `RemoveFatigueCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueCategory {
  mutation: "removeFatigueCategory";
  id: string;
}

export const parseRemoveFatigueCategory: NormWireReader<RemoveFatigueCategory> = normWireObject<RemoveFatigueCategory>({ mutation: normWireRequired(normWireLiteral("removeFatigueCategory")), id: normWireRequired(normWireString) });
