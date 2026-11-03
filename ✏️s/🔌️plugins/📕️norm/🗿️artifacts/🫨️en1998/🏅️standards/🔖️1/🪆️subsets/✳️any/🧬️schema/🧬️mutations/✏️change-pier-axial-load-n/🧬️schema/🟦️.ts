/** ✏️ `change-pier-axial-load-n` wire twin: the leaf payload `ChangePierAxialLoadN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierAxialLoadN {
  mutation: "changePierAxialLoadN";
  id: string;
  newAxialLoadN: number;
}

export const parseChangePierAxialLoadN: NormWireReader<ChangePierAxialLoadN> = normWireObject<ChangePierAxialLoadN>({ mutation: normWireRequired(normWireLiteral("changePierAxialLoadN")), id: normWireRequired(normWireString), newAxialLoadN: normWireRequired(normWireNumber) });
