/** mutation payload — mirrors `IntroduceProductIndex`. */
import type { ProductIndex } from "../../🟦️.ts";

export interface IntroduceProductIndex {
  product_index: ProductIndex;
  index?: number;
}
