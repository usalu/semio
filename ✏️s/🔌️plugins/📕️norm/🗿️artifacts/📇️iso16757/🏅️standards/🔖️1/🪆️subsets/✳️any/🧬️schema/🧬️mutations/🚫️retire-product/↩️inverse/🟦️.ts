/** ↩️ inverse for `RetireProduct` — undo re-`create`s the product from BASE state, mirroring
 * `IntroduceProduct` (not `RetireProduct` — deletion's inverse is a creation, not another deletion). */
import type { IntroduceProduct } from "../../📦️introduce-product/🦠️mutation/🟦️.ts";

export type RetireProductInverse = IntroduceProduct;
