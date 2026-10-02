/** 📦️ `add-product` wire twin: the leaf payload `AddProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseVdi3805CatalogueProduct, type Vdi3805CatalogueProduct } from "../../../📸️snapshot/🟦️.ts";

export interface AddProduct {
  product: Vdi3805CatalogueProduct;
  index: number | null;
}

export const parseAddProduct: NormWireReader<AddProduct> = normWireObject<AddProduct>({ product: normWireRequired(parseVdi3805CatalogueProduct), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
