/** 🔩️ `change-row-grip-length` wire twin: the leaf payload `ChangeRowGripLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowGripLength {
  id: string;
  newGripLength: number;
}

export const parseChangeRowGripLength: NormWireReader<ChangeRowGripLength> = normWireObject<ChangeRowGripLength>({ id: normWireRequired(normWireString), newGripLength: normWireRequired(normWireNumber) });
