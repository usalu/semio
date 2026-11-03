/** 🏗️ `change-member-sls-limit-char` wire twin: the leaf payload `ChangeMemberSlsLimitChar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSlsLimitChar {
  mutation: "changeMemberSlsLimitChar";
  id: string;
  newSlsLimitChar: number;
}

export const parseChangeMemberSlsLimitChar: NormWireReader<ChangeMemberSlsLimitChar> = normWireObject<ChangeMemberSlsLimitChar>({ mutation: normWireRequired(normWireLiteral("changeMemberSlsLimitChar")), id: normWireRequired(normWireString), newSlsLimitChar: normWireRequired(normWireNumber) });
