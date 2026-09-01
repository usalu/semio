/** mutation payload — mirrors `CreateProduct`. */
import type { Product } from "../../🟦️component.ts";

export interface CreateProduct {
  product: Product;
  index?: number;
}
