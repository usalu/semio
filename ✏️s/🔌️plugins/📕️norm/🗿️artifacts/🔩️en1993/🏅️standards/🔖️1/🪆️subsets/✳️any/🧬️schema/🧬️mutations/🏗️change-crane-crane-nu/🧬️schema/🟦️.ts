/** 🏗️ `change-crane-crane-nu` wire twin: the leaf payload `ChangeCraneCraneNu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneCraneNu {
  id: string;
  newCraneNu: number;
}

export const parseChangeCraneCraneNu: NormWireReader<ChangeCraneCraneNu> = normWireObject<ChangeCraneCraneNu>({ id: normWireRequired(normWireString), newCraneNu: normWireRequired(normWireNumber) });
