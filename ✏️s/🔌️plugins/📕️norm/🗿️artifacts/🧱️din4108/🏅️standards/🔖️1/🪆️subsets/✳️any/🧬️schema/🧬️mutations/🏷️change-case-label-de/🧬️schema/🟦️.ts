/** 🏷️ `change-case-label-de` wire twin: the leaf payload `ChangeCaseLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseLabelDe {
  caseId: string;
  newValue: string;
}

export const parseChangeCaseLabelDe: NormWireReader<ChangeCaseLabelDe> = normWireObject<ChangeCaseLabelDe>({ caseId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
