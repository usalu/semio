/** ↩️ inverse for `DeleteProduct` — undo re-`create`s the product from BASE state, mirroring
 * `CreateProduct` (not `DeleteProduct` — deletion's inverse is a creation, not another deletion). */
import type { CreateProduct } from "../../🍁create-product/🦠️mutation/🟦️.ts";

export type DeleteProductInverse = CreateProduct;
