/** 📍️ `change-placement-yaw` wire twin: the leaf payload `ChangePlacementYaw`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementYaw {
  id: string;
  newYaw: number;
}

export const parseChangePlacementYaw: NormWireReader<ChangePlacementYaw> = normWireObject<ChangePlacementYaw>({ id: normWireRequired(normWireString), newYaw: normWireRequired(normWireNumber) });
