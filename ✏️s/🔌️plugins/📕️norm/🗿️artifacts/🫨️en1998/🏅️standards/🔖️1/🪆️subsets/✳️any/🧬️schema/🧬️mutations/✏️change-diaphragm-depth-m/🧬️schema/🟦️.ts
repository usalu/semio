/** ✏️ `change-diaphragm-depth-m` wire twin: the leaf payload `ChangeDiaphragmDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmDepthM {
  mutation: "changeDiaphragmDepthM";
  id: string;
  newDepthM: number;
}

export const parseChangeDiaphragmDepthM: NormWireReader<ChangeDiaphragmDepthM> = normWireObject<ChangeDiaphragmDepthM>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmDepthM")), id: normWireRequired(normWireString), newDepthM: normWireRequired(normWireNumber) });
