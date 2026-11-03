/** 🏛️ `change-member-prestress-ecc` wire twin: the leaf payload `ChangeMemberPrestressEcc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPrestressEcc {
  memberId: string;
  newPrestressEccentricity: number;
}

export const parseChangeMemberPrestressEcc: NormWireReader<ChangeMemberPrestressEcc> = normWireObject<ChangeMemberPrestressEcc>({ memberId: normWireRequired(normWireString), newPrestressEccentricity: normWireRequired(normWireNumber) });
