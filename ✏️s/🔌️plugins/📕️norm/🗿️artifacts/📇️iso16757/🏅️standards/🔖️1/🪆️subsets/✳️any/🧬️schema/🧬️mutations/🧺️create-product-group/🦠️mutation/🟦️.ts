/** mutation payload — mirrors `CreateProductGroup`. */
import type { ProductGroup } from "../../🟦️.ts";

export interface CreateProductGroup {
  product_group: ProductGroup;
  index?: number;
}
