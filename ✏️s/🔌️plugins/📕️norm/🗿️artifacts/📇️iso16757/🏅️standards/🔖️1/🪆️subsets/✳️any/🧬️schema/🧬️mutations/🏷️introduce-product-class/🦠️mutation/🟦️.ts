/** mutation payload — mirrors `IntroduceProductClass`. */
import type { ProductClass } from "../../🟦️.ts";

export interface IntroduceProductClass {
  product_class: ProductClass;
  index?: number;
}
