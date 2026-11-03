/** 🪢️ `change-tension-cable-stress` wire twin: the leaf payload `ChangeTensionCableStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionCableStress {
  id: string;
  newCableStress: number;
}

export const parseChangeTensionCableStress: NormWireReader<ChangeTensionCableStress> = normWireObject<ChangeTensionCableStress>({ id: normWireRequired(normWireString), newCableStress: normWireRequired(normWireNumber) });
