/** mutation payload — mirrors `IntroduceProductSeries`. */
import type { ProductSeries } from "../../🟦️.ts";

export interface IntroduceProductSeries {
  product_series: ProductSeries;
  index?: number;
}
