/** ⬇️ `change-column-action-force-n` wire twin: the leaf payload `ChangeColumnActionForceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeColumnActionForceN {
  index: number;
  actionIndex: number;
  newNKN: number;
}

export const parseChangeColumnActionForceN: NormWireReader<ChangeColumnActionForceN> = normWireObject<ChangeColumnActionForceN>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), actionIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newNKN: normWireRequired(normWireNumber) });
