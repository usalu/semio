/** 🔗️ `change-tie-capacity` wire twin: the leaf payload `ChangeTieCapacity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieCapacity {
  wallId: string;
  id: string;
  newCapacityN: number;
}

export const parseChangeTieCapacity: NormWireReader<ChangeTieCapacity> = normWireObject<ChangeTieCapacity>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCapacityN: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
