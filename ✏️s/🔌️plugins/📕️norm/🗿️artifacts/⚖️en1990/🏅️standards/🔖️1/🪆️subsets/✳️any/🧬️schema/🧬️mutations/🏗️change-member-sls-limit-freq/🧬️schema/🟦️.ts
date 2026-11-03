/** 🏗️ `change-member-sls-limit-freq` wire twin: the leaf payload `ChangeMemberSlsLimitFreq`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSlsLimitFreq {
  mutation: "changeMemberSlsLimitFreq";
  id: string;
  newSlsLimitFreq: number;
}

export const parseChangeMemberSlsLimitFreq: NormWireReader<ChangeMemberSlsLimitFreq> = normWireObject<ChangeMemberSlsLimitFreq>({ mutation: normWireRequired(normWireLiteral("changeMemberSlsLimitFreq")), id: normWireRequired(normWireString), newSlsLimitFreq: normWireRequired(normWireNumber) });
