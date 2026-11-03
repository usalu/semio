/** 🪢️ `change-tension-rod-fu` wire twin: the leaf payload `ChangeTensionRodFu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionRodFu {
  id: string;
  newRodFu: number;
}

export const parseChangeTensionRodFu: NormWireReader<ChangeTensionRodFu> = normWireObject<ChangeTensionRodFu>({ id: normWireRequired(normWireString), newRodFu: normWireRequired(normWireNumber) });
