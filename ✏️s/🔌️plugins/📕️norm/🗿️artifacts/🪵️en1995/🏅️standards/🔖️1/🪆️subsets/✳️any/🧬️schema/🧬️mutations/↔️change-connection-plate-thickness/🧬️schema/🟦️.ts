/** ↔️ `change-connection-plate-thickness` wire twin: the leaf payload `ChangeConnectionPlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionPlateThickness {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionPlateThickness: NormWireReader<ChangeConnectionPlateThickness> = normWireObject<ChangeConnectionPlateThickness>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
