/** 🪢️ `change-tension-cable-weight` wire twin: the leaf payload `ChangeTensionCableWeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionCableWeight {
  id: string;
  newCableWeight: number;
}

export const parseChangeTensionCableWeight: NormWireReader<ChangeTensionCableWeight> = normWireObject<ChangeTensionCableWeight>({ id: normWireRequired(normWireString), newCableWeight: normWireRequired(normWireNumber) });
