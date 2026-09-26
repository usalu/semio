/** ↩️ inverse for `RetireProductGroup` — undo re-`create`s the group from BASE state, mirroring
 * `IntroduceProductGroup` (not `RetireProductGroup` — deletion's inverse is a creation). */
import type { IntroduceProductGroup } from "../../🧺️introduce-product-group/🦠️mutation/🟦️.ts";

export type RetireProductGroupInverse = IntroduceProductGroup;
