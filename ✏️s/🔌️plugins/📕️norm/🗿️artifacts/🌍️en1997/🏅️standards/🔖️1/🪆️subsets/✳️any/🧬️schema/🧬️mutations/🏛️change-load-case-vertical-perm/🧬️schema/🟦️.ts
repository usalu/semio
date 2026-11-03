/** 🏛️ `change-load-case-vertical-perm` wire twin: the leaf payload `ChangeLoadCaseVerticalPerm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseVerticalPerm {
  mutation: "changeLoadCaseVerticalPerm";
  footingId: string;
  id: string;
  newVerticalPermanent: number;
}

export const parseChangeLoadCaseVerticalPerm: NormWireReader<ChangeLoadCaseVerticalPerm> = normWireObject<ChangeLoadCaseVerticalPerm>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseVerticalPerm")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newVerticalPermanent: normWireRequired(normWireNumber) });
