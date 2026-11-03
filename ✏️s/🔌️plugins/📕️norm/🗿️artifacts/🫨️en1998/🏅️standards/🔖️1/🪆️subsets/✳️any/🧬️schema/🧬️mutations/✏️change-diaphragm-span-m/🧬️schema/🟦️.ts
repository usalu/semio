/** ✏️ `change-diaphragm-span-m` wire twin: the leaf payload `ChangeDiaphragmSpanM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmSpanM {
  mutation: "changeDiaphragmSpanM";
  id: string;
  newSpanM: number;
}

export const parseChangeDiaphragmSpanM: NormWireReader<ChangeDiaphragmSpanM> = normWireObject<ChangeDiaphragmSpanM>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmSpanM")), id: normWireRequired(normWireString), newSpanM: normWireRequired(normWireNumber) });
