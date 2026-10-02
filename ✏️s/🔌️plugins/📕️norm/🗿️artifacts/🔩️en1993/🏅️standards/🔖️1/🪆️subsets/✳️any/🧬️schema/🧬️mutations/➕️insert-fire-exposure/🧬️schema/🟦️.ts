/** ➕️ `insert-fire-exposure` wire twin: the leaf payload `InsertFireExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FireExposure, parseFireExposure } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFireExposure {
  index: number;
  fireExposure: FireExposure;
}

export const parseInsertFireExposure: NormWireReader<InsertFireExposure> = normWireObject<InsertFireExposure>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), fireExposure: normWireRequired(parseFireExposure) });
