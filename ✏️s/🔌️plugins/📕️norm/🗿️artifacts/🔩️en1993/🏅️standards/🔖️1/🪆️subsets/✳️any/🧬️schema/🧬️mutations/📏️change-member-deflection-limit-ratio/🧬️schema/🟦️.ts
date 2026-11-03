/** 📏️ `change-member-deflection-limit-ratio` wire twin: the leaf payload `ChangeMemberDeflectionLimitRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflectionLimitRatio {
  id: string;
  newDeflectionLimitRatio: number;
}

export const parseChangeMemberDeflectionLimitRatio: NormWireReader<ChangeMemberDeflectionLimitRatio> = normWireObject<ChangeMemberDeflectionLimitRatio>({ id: normWireRequired(normWireString), newDeflectionLimitRatio: normWireRequired(normWireNumber) });
