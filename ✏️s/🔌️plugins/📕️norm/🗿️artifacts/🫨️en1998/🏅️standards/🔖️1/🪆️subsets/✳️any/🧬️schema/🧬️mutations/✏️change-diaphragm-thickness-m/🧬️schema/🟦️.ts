/** ✏️ `change-diaphragm-thickness-m` wire twin: the leaf payload `ChangeDiaphragmThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmThicknessM {
  mutation: "changeDiaphragmThicknessM";
  id: string;
  newThicknessM: number;
}

export const parseChangeDiaphragmThicknessM: NormWireReader<ChangeDiaphragmThicknessM> = normWireObject<ChangeDiaphragmThicknessM>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmThicknessM")), id: normWireRequired(normWireString), newThicknessM: normWireRequired(normWireNumber) });
