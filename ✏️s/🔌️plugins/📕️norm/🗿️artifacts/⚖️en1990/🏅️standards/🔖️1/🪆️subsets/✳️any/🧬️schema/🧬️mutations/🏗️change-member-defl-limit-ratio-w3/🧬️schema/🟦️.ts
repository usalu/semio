/** 🏗️ `change-member-defl-limit-ratio-w3` wire twin: the leaf payload `ChangeMemberDeflLimitRatioW3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflLimitRatioW3 {
  mutation: "changeMemberDeflLimitRatioW3";
  id: string;
  newDeflectionLimitRatioW3: number;
}

export const parseChangeMemberDeflLimitRatioW3: NormWireReader<ChangeMemberDeflLimitRatioW3> = normWireObject<ChangeMemberDeflLimitRatioW3>({ mutation: normWireRequired(normWireLiteral("changeMemberDeflLimitRatioW3")), id: normWireRequired(normWireString), newDeflectionLimitRatioW3: normWireRequired(normWireNumber) });
