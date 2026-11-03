/** 🏗️ `change-member-sls-limit-qp` wire twin: the leaf payload `ChangeMemberSlsLimitQp`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSlsLimitQp {
  mutation: "changeMemberSlsLimitQp";
  id: string;
  newSlsLimitQp: number;
}

export const parseChangeMemberSlsLimitQp: NormWireReader<ChangeMemberSlsLimitQp> = normWireObject<ChangeMemberSlsLimitQp>({ mutation: normWireRequired(normWireLiteral("changeMemberSlsLimitQp")), id: normWireRequired(normWireString), newSlsLimitQp: normWireRequired(normWireNumber) });
