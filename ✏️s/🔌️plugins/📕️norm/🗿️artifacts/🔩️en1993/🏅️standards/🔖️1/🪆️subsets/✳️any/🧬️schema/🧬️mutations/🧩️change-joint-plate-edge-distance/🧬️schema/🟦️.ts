/** 🧩️ `change-joint-plate-edge-distance` wire twin: the leaf payload `ChangeJointPlateEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateEdgeDistance {
  id: string;
  newEdgeDistance: number;
}

export const parseChangeJointPlateEdgeDistance: NormWireReader<ChangeJointPlateEdgeDistance> = normWireObject<ChangeJointPlateEdgeDistance>({ id: normWireRequired(normWireString), newEdgeDistance: normWireRequired(normWireNumber) });
