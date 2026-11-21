/** 🏷️ `change-case-label-en` wire twin: the leaf payload `ChangeCaseLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseLabelEn {
  caseId: string;
  newValue: string;
}

export const parseChangeCaseLabelEn: NormWireReader<ChangeCaseLabelEn> = normWireObject<ChangeCaseLabelEn>({ caseId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
