/** 🏛️ `change-member-prestress-loss-ratio` wire twin: the leaf payload `ChangeMemberPrestressLossRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPrestressLossRatio {
  memberId: string;
  newPrestressLossRatio: number;
}

export const parseChangeMemberPrestressLossRatio: NormWireReader<ChangeMemberPrestressLossRatio> = normWireObject<ChangeMemberPrestressLossRatio>({ memberId: normWireRequired(normWireString), newPrestressLossRatio: normWireRequired(normWireNumber) });
