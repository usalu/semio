/** 📏️ `change-frame-member-basis-basis` wire twin: the leaf payload `ChangeFrameMemberBasisBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MemberForceBasis, parseMemberForceBasis } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFrameMemberBasisBasis {
  id: string;
  newBasis: MemberForceBasis;
}

export const parseChangeFrameMemberBasisBasis: NormWireReader<ChangeFrameMemberBasisBasis> = normWireObject<ChangeFrameMemberBasisBasis>({ id: normWireRequired(normWireString), newBasis: normWireRequired(parseMemberForceBasis) });
