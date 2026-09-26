/** mutation payload — mirrors `IntroduceProduct`. */
import type { Product } from "../../🟦️.ts";

export interface IntroduceProduct {
  product: Product;
  index?: number;
}
