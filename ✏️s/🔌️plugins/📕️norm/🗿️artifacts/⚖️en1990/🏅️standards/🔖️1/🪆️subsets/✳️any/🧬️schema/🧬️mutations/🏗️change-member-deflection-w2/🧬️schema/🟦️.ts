/** 🏗️ `change-member-deflection-w2` wire twin: the leaf payload `ChangeMemberDeflectionW2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflectionW2 {
  mutation: "changeMemberDeflectionW2";
  id: string;
  newDeflectionW2: number;
}

export const parseChangeMemberDeflectionW2: NormWireReader<ChangeMemberDeflectionW2> = normWireObject<ChangeMemberDeflectionW2>({ mutation: normWireRequired(normWireLiteral("changeMemberDeflectionW2")), id: normWireRequired(normWireString), newDeflectionW2: normWireRequired(normWireNumber) });
