/** ➕️ `insert-layer` wire twin: the leaf payload `InsertLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSoilLayer, type SoilLayer } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLayer {
  mutation: "insertLayer";
  index: number;
  layer: SoilLayer;
}

export const parseInsertLayer: NormWireReader<InsertLayer> = normWireObject<InsertLayer>({ mutation: normWireRequired(normWireLiteral("insertLayer")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), layer: normWireRequired(parseSoilLayer) });
