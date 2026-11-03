/** 🪢️ `change-tension-f-uk` wire twin: the leaf payload `ChangeTensionFUk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionFUk {
  id: string;
  newFUk: number;
}

export const parseChangeTensionFUk: NormWireReader<ChangeTensionFUk> = normWireObject<ChangeTensionFUk>({ id: normWireRequired(normWireString), newFUk: normWireRequired(normWireNumber) });
