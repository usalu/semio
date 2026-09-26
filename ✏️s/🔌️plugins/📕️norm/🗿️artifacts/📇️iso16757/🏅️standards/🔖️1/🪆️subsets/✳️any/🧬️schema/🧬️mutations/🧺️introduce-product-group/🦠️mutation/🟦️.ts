/** mutation payload — mirrors `IntroduceProductGroup`. */
import type { ProductGroup } from "../../🟦️.ts";

export interface IntroduceProductGroup {
  product_group: ProductGroup;
  index?: number;
}
