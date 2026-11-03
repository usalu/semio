/** 🪢️ `change-tension-pin-gap` wire twin: the leaf payload `ChangeTensionPinGap`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionPinGap {
  id: string;
  newPinGap: number;
}

export const parseChangeTensionPinGap: NormWireReader<ChangeTensionPinGap> = normWireObject<ChangeTensionPinGap>({ id: normWireRequired(normWireString), newPinGap: normWireRequired(normWireNumber) });
