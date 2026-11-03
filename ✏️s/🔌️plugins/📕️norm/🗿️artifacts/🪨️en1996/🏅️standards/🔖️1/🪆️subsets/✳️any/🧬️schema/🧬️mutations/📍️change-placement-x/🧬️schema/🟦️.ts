/** 📍️ `change-placement-x` wire twin: the leaf payload `ChangePlacementX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementX {
  id: string;
  newX: number;
}

export const parseChangePlacementX: NormWireReader<ChangePlacementX> = normWireObject<ChangePlacementX>({ id: normWireRequired(normWireString), newX: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
