/** 🪢️ `change-tension-pin-diameter` wire twin: the leaf payload `ChangeTensionPinDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinDiameter {
  id: string;
  newPinDiameter: number;
}

export const parseChangeTensionPinDiameter: NormWireReader<ChangeTensionPinDiameter> = normWireObject<ChangeTensionPinDiameter>({ id: normWireRequired(normWireString), newPinDiameter: normWireRequired(normWireNumber) });
