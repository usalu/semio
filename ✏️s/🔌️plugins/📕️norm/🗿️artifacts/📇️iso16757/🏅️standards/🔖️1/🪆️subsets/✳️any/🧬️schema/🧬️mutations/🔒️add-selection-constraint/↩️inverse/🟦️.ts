/** ↩️ inverse for `AddSelectionConstraint` — undo is `remove-selection-constraint` at the index the
 * append landed on (not another `AddSelectionConstraint`). */
import type { RemoveSelectionConstraint } from "../../🔓️remove-selection-constraint/🦠️mutation/🟦️.ts";

export type AddSelectionConstraintInverse = RemoveSelectionConstraint;
