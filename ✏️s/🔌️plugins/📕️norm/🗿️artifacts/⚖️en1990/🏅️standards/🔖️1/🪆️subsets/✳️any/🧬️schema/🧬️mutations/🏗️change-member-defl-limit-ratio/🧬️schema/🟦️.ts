/** 🏗️ `change-member-defl-limit-ratio` wire twin: the leaf payload `ChangeMemberDeflLimitRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflLimitRatio {
  mutation: "changeMemberDeflLimitRatio";
  id: string;
  newDeflectionLimitRatio: number;
}

export const parseChangeMemberDeflLimitRatio: NormWireReader<ChangeMemberDeflLimitRatio> = normWireObject<ChangeMemberDeflLimitRatio>({ mutation: normWireRequired(normWireLiteral("changeMemberDeflLimitRatio")), id: normWireRequired(normWireString), newDeflectionLimitRatio: normWireRequired(normWireNumber) });
