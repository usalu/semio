/** ✏️ `change-diaphragm-chord-area-m2` wire twin: the leaf payload `ChangeDiaphragmChordAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmChordAreaM2 {
  mutation: "changeDiaphragmChordAreaM2";
  id: string;
  newChordAreaM2: number;
}

export const parseChangeDiaphragmChordAreaM2: NormWireReader<ChangeDiaphragmChordAreaM2> = normWireObject<ChangeDiaphragmChordAreaM2>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmChordAreaM2")), id: normWireRequired(normWireString), newChordAreaM2: normWireRequired(normWireNumber) });
