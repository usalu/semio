/** 🪢️ `change-tension-pin-fyp` wire twin: the leaf payload `ChangeTensionPinFyp`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinFyp {
  id: string;
  newPinFyp: number;
}

export const parseChangeTensionPinFyp: NormWireReader<ChangeTensionPinFyp> = normWireObject<ChangeTensionPinFyp>({ id: normWireRequired(normWireString), newPinFyp: normWireRequired(normWireNumber) });
