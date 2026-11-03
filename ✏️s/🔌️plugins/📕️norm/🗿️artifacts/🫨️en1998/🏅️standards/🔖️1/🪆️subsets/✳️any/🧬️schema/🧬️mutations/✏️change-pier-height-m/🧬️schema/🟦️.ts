/** ✏️ `change-pier-height-m` wire twin: the leaf payload `ChangePierHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierHeightM {
  mutation: "changePierHeightM";
  id: string;
  newHeightM: number;
}

export const parseChangePierHeightM: NormWireReader<ChangePierHeightM> = normWireObject<ChangePierHeightM>({ mutation: normWireRequired(normWireLiteral("changePierHeightM")), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireNumber) });
