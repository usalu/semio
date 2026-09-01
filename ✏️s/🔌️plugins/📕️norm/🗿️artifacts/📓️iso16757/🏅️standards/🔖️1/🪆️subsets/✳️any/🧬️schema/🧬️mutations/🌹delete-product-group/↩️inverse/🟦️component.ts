/** ↩️ inverse for `DeleteProductGroup` — undo re-`create`s the group from BASE state, mirroring
 * `CreateProductGroup` (not `DeleteProductGroup` — deletion's inverse is a creation). */
import type { CreateProductGroup } from "../../🍀create-product-group/🦠️mutation/🟦️component.ts";

export type DeleteProductGroupInverse = CreateProductGroup;
