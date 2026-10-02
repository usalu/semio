/** 🔢️ `change-connection-shear-planes` wire twin: the leaf payload `ChangeConnectionShearPlanes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionShearPlanes {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionShearPlanes: NormWireReader<ChangeConnectionShearPlanes> = normWireObject<ChangeConnectionShearPlanes>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
