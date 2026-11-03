/** 🪢️ `change-tension-pin-fup` wire twin: the leaf payload `ChangeTensionPinFup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinFup {
  id: string;
  newPinFup: number;
}

export const parseChangeTensionPinFup: NormWireReader<ChangeTensionPinFup> = normWireObject<ChangeTensionPinFup>({ id: normWireRequired(normWireString), newPinFup: normWireRequired(normWireNumber) });
