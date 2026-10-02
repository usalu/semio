/** ↔️ `change-connection-edge-distance` wire twin: the leaf payload `ChangeConnectionEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionEdgeDistance {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionEdgeDistance: NormWireReader<ChangeConnectionEdgeDistance> = normWireObject<ChangeConnectionEdgeDistance>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
