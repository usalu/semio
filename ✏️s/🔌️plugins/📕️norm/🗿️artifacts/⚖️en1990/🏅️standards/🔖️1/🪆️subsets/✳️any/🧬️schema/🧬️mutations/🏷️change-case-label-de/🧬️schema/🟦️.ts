/** 🏷️ `change-case-label-de` wire twin: the leaf payload `ChangeCaseLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseLabelDe {
  mutation: "changeCaseLabelDe";
  caseId: string;
  newValue: string;
}

export const parseChangeCaseLabelDe: NormWireReader<ChangeCaseLabelDe> = normWireObject<ChangeCaseLabelDe>({ mutation: normWireRequired(normWireLiteral("changeCaseLabelDe")), caseId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
