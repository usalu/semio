/** ↔️ `change-connection-spacing` wire twin: the leaf payload `ChangeConnectionSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionSpacing {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionSpacing: NormWireReader<ChangeConnectionSpacing> = normWireObject<ChangeConnectionSpacing>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
