/** 🪢️ `change-tension-pin-inner-thickness` wire twin: the leaf payload `ChangeTensionPinInnerThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinInnerThickness {
  id: string;
  newPinInnerThickness: number;
}

export const parseChangeTensionPinInnerThickness: NormWireReader<ChangeTensionPinInnerThickness> = normWireObject<ChangeTensionPinInnerThickness>({ id: normWireRequired(normWireString), newPinInnerThickness: normWireRequired(normWireNumber) });
