/** 📅️ `change-vent-inspection` wire twin: the leaf payload `ChangeVentInspection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentInspection {
  ventId: string;
  newYearsSinceInspection: number;
}

export const parseChangeVentInspection: NormWireReader<ChangeVentInspection> = normWireObject<ChangeVentInspection>({ ventId: normWireRequired(normWireString), newYearsSinceInspection: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
