/** mutation payload — mirrors `IntroduceProductClass`. */
import type { ProductClass } from "../../🟦️.ts";

export interface IntroduceProductClass {
  productClass: ProductClass;
  index?: number;
}
