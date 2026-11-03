/** 📍️ `change-placement-x` wire twin: the leaf payload `ChangePlacementX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementX {
  id: string;
  newValue: number;
}

export const parseChangePlacementX: NormWireReader<ChangePlacementX> = normWireObject<ChangePlacementX>({ id: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
