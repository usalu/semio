/** 📚 `introduce-product-series` wire twin: the leaf payload `IntroduceProductSeries`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseProductSeries, type ProductSeries } from "../../../📸️snapshot/🟦️.ts";

export interface IntroduceProductSeries {
  productSeries: ProductSeries;
  index: number | null;
}

export const parseIntroduceProductSeries: NormWireReader<IntroduceProductSeries> = normWireObject<IntroduceProductSeries>({ productSeries: normWireRequired(parseProductSeries), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
