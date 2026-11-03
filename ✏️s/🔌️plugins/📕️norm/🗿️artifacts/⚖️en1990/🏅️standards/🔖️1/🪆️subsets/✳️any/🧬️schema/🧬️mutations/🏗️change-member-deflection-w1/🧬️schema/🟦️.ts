/** 🏗️ `change-member-deflection-w1` wire twin: the leaf payload `ChangeMemberDeflectionW1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflectionW1 {
  mutation: "changeMemberDeflectionW1";
  id: string;
  newDeflectionW1: number;
}

export const parseChangeMemberDeflectionW1: NormWireReader<ChangeMemberDeflectionW1> = normWireObject<ChangeMemberDeflectionW1>({ mutation: normWireRequired(normWireLiteral("changeMemberDeflectionW1")), id: normWireRequired(normWireString), newDeflectionW1: normWireRequired(normWireNumber) });
