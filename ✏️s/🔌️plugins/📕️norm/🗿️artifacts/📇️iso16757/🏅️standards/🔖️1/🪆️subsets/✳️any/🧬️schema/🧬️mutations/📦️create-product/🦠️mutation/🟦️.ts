/** mutation payload — mirrors `CreateProduct`. */
import type { Product } from "../../🟦️.ts";

export interface CreateProduct {
  product: Product;
  index?: number;
}
