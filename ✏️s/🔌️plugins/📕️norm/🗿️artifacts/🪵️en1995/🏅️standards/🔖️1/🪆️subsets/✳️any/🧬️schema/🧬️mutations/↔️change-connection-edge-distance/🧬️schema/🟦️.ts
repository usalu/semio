/** ↔️ `change-connection-edge-distance` wire twin: the leaf payload `ChangeConnectionEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionEdgeDistance {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionEdgeDistance: NormWireReader<ChangeConnectionEdgeDistance> = normWireObject<ChangeConnectionEdgeDistance>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
