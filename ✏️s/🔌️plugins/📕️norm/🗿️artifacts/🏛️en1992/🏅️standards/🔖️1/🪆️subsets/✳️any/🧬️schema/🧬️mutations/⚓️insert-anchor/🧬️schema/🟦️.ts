/** ⚓️ `insert-anchor` wire twin: the leaf payload `InsertAnchor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Anchor, parseAnchor } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAnchor {
  index: number;
  anchor: Anchor;
}

export const parseInsertAnchor: NormWireReader<InsertAnchor> = normWireObject<InsertAnchor>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), anchor: normWireRequired(parseAnchor) });
