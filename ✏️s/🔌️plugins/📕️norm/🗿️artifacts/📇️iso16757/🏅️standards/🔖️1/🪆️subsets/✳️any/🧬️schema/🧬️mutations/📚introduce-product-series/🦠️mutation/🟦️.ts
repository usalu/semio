/** mutation payload — mirrors `IntroduceProductSeries`. */
import type { ProductSeries } from "../../🟦️.ts";

export interface IntroduceProductSeries {
  productSeries: ProductSeries;
  index?: number;
}
