/** 🔎 `introduce-product-index` wire twin: the leaf payload `IntroduceProductIndex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseProductIndex, type ProductIndex } from "../../../📸️snapshot/🟦️.ts";

export interface IntroduceProductIndex {
  productIndex: ProductIndex;
  index: number | null;
}

export const parseIntroduceProductIndex: NormWireReader<IntroduceProductIndex> = normWireObject<IntroduceProductIndex>({ productIndex: normWireRequired(parseProductIndex), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
