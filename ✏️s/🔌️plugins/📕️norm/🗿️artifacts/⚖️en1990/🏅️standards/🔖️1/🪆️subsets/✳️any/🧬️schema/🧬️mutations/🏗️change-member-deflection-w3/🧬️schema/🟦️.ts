/** 🏗️ `change-member-deflection-w3` wire twin: the leaf payload `ChangeMemberDeflectionW3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflectionW3 {
  mutation: "changeMemberDeflectionW3";
  id: string;
  newDeflectionW3: number;
}

export const parseChangeMemberDeflectionW3: NormWireReader<ChangeMemberDeflectionW3> = normWireObject<ChangeMemberDeflectionW3>({ mutation: normWireRequired(normWireLiteral("changeMemberDeflectionW3")), id: normWireRequired(normWireString), newDeflectionW3: normWireRequired(normWireNumber) });
