/** 🏗️ `change-member-second-order-analysis` wire twin: the leaf payload `ChangeMemberSecondOrderAnalysis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSecondOrderAnalysis {
  mutation: "changeMemberSecondOrderAnalysis";
  id: string;
  newSecondOrderAnalysis: number;
}

export const parseChangeMemberSecondOrderAnalysis: NormWireReader<ChangeMemberSecondOrderAnalysis> = normWireObject<ChangeMemberSecondOrderAnalysis>({ mutation: normWireRequired(normWireLiteral("changeMemberSecondOrderAnalysis")), id: normWireRequired(normWireString), newSecondOrderAnalysis: normWireRequired(normWireNumber) });
