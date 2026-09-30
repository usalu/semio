/** mutation payload — mirrors `IntroduceProductGroup`. */
import type { ProductGroup } from "../../🟦️.ts";

export interface IntroduceProductGroup {
  productGroup: ProductGroup;
  index?: number;
}
