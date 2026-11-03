/** 📍️ `change-placement-y` wire twin: the leaf payload `ChangePlacementY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementY {
  id: string;
  newY: number;
}

export const parseChangePlacementY: NormWireReader<ChangePlacementY> = normWireObject<ChangePlacementY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
