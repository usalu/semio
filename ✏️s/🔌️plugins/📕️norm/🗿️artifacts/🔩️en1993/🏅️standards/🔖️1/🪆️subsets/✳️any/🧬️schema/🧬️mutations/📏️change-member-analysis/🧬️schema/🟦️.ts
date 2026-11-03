/** 📏️ `change-member-analysis` wire twin: the leaf payload `ChangeMemberAnalysis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AnalysisKind, parseAnalysisKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberAnalysis {
  id: string;
  newAnalysis: AnalysisKind;
}

export const parseChangeMemberAnalysis: NormWireReader<ChangeMemberAnalysis> = normWireObject<ChangeMemberAnalysis>({ id: normWireRequired(normWireString), newAnalysis: normWireRequired(parseAnalysisKind) });
