/** ✏️ `change-pier-xm` wire twin: the leaf payload `ChangePierXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierXM {
  mutation: "changePierXM";
  id: string;
  newXM: number;
}

export const parseChangePierXM: NormWireReader<ChangePierXM> = normWireObject<ChangePierXM>({ mutation: normWireRequired(normWireLiteral("changePierXM")), id: normWireRequired(normWireString), newXM: normWireRequired(normWireNumber) });
