/** 📦️ `introduce-product` wire twin: the leaf payload `IntroduceProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseProduct, type Product } from "../../../📸️snapshot/🟦️.ts";

export interface IntroduceProduct {
  product: Product;
  index: number | null;
}

export const parseIntroduceProduct: NormWireReader<IntroduceProduct> = normWireObject<IntroduceProduct>({ product: normWireRequired(parseProduct), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
