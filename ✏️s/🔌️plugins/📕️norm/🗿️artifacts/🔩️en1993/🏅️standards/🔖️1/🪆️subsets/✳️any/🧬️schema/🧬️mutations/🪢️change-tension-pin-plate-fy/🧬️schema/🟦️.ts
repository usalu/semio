/** 🪢️ `change-tension-pin-plate-fy` wire twin: the leaf payload `ChangeTensionPinPlateFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinPlateFy {
  id: string;
  newPinPlateFy: number;
}

export const parseChangeTensionPinPlateFy: NormWireReader<ChangeTensionPinPlateFy> = normWireObject<ChangeTensionPinPlateFy>({ id: normWireRequired(normWireString), newPinPlateFy: normWireRequired(normWireNumber) });
