/** ➕️ `insert-layer` wire twin: the leaf payload `InsertLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108LayerDocument, parseDin4108LayerDocument } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLayer {
  elementId: string;
  index?: number | null;
  layer: Din4108LayerDocument;
}

export const parseInsertLayer: NormWireReader<InsertLayer> = normWireObject<InsertLayer>({ elementId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), layer: normWireRequired(parseDin4108LayerDocument) });
