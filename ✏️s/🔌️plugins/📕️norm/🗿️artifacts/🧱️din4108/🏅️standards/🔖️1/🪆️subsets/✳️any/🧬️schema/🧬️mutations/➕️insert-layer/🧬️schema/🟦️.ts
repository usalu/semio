/** ➕️ `insert-layer` wire twin: the leaf payload `InsertLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din4108LayerDocument, parseDin4108LayerDocument } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLayer {
  elementId: string;
  index: number | null;
  layer: Din4108LayerDocument;
}

export const parseInsertLayer: NormWireReader<InsertLayer> = normWireObject<InsertLayer>({ elementId: normWireRequired(normWireString), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), layer: normWireRequired(parseDin4108LayerDocument) });
