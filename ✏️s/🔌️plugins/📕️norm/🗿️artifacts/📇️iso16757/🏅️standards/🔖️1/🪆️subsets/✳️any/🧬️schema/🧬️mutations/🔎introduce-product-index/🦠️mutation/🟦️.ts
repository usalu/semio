/** mutation payload — mirrors `IntroduceProductIndex`. */
import type { ProductIndex } from "../../🟦️.ts";

export interface IntroduceProductIndex {
  productIndex: ProductIndex;
  index?: number;
}
