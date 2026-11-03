/** 🪢️ `change-tension-cable-modulus` wire twin: the leaf payload `ChangeTensionCableModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionCableModulus {
  id: string;
  newCableModulus: number;
}

export const parseChangeTensionCableModulus: NormWireReader<ChangeTensionCableModulus> = normWireObject<ChangeTensionCableModulus>({ id: normWireRequired(normWireString), newCableModulus: normWireRequired(normWireNumber) });
