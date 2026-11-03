/** 🪢️ `change-tension-pin-plate-thickness` wire twin: the leaf payload `ChangeTensionPinPlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinPlateThickness {
  id: string;
  newPinPlateThickness: number;
}

export const parseChangeTensionPinPlateThickness: NormWireReader<ChangeTensionPinPlateThickness> = normWireObject<ChangeTensionPinPlateThickness>({ id: normWireRequired(normWireString), newPinPlateThickness: normWireRequired(normWireNumber) });
