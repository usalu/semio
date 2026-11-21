/** 🏷️ `change-case-label-en` wire twin: the leaf payload `ChangeCaseLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseLabelEn {
  mutation: "changeCaseLabelEn";
  caseId: string;
  newValue: string;
}

export const parseChangeCaseLabelEn: NormWireReader<ChangeCaseLabelEn> = normWireObject<ChangeCaseLabelEn>({ mutation: normWireRequired(normWireLiteral("changeCaseLabelEn")), caseId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
