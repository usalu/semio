/** 🔄️ `change-fatigue-detail-category` wire twin: the leaf payload `ChangeFatigueDetailCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueDetailCategory {
  mutation: "changeFatigueDetailCategory";
  fatigueDetailId: string;
  newDetailCategory: string;
}

export const parseChangeFatigueDetailCategory: NormWireReader<ChangeFatigueDetailCategory> = normWireObject<ChangeFatigueDetailCategory>({ mutation: normWireRequired(normWireLiteral("changeFatigueDetailCategory")), fatigueDetailId: normWireRequired(normWireString), newDetailCategory: normWireRequired(normWireString) });
