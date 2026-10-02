/** 🌀️ `change-vent-sfp` wire twin: the leaf payload `ChangeVentSfp`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentSfp {
  ventId: string;
  newSfpWM3S: number;
}

export const parseChangeVentSfp: NormWireReader<ChangeVentSfp> = normWireObject<ChangeVentSfp>({ ventId: normWireRequired(normWireString), newSfpWM3S: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
