/** 📏️ `change-member-defl-limit-ratio` wire twin: the leaf payload `ChangeMemberDeflLimitRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflLimitRatio {
  id: string;
  newDeflectionLimitRatio: number;
}

export const parseChangeMemberDeflLimitRatio: NormWireReader<ChangeMemberDeflLimitRatio> = normWireObject<ChangeMemberDeflLimitRatio>({ id: normWireRequired(normWireString), newDeflectionLimitRatio: normWireRequired(normWireNumber) });
