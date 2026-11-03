/** 📍️ `change-placement-z` wire twin: the leaf payload `ChangePlacementZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementZ {
  id: string;
  newZ: number;
}

export const parseChangePlacementZ: NormWireReader<ChangePlacementZ> = normWireObject<ChangePlacementZ>({ id: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
