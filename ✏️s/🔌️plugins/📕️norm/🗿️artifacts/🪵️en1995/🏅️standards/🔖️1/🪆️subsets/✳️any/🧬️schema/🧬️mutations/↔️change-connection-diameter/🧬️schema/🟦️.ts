/** ↔️ `change-connection-diameter` wire twin: the leaf payload `ChangeConnectionDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionDiameter {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionDiameter: NormWireReader<ChangeConnectionDiameter> = normWireObject<ChangeConnectionDiameter>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
