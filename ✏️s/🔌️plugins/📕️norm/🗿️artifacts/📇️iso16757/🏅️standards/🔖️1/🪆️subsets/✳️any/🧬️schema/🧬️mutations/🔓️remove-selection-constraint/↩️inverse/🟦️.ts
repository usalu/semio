/** ↩️ inverse for `RemoveSelectionConstraint` — undo re-`add`s the captured constraint (not another
 * `RemoveSelectionConstraint`); out-of-range BASE index ⇒ no mutation. */
import type { AddSelectionConstraint } from "../../🔒️add-selection-constraint/🦠️mutation/🟦️.ts";

export type RemoveSelectionConstraintInverse = AddSelectionConstraint;
