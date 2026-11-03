/** 🧱️ `change-fastened-joint-edge-distance` wire twin: the leaf payload `ChangeFastenedJointEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointEdgeDistance {
  id: string;
  newEdgeDistance: number;
}

export const parseChangeFastenedJointEdgeDistance: NormWireReader<ChangeFastenedJointEdgeDistance> = normWireObject<ChangeFastenedJointEdgeDistance>({ id: normWireRequired(normWireString), newEdgeDistance: normWireRequired(normWireNumber) });
