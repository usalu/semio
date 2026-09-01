/** ↩️ inverse for `RemovePartNumberInput` — restores BASE's value via `change` (not `remove` —
 * removal's inverse is a change, not another removal); missing key ⇒ no mutation. */
import type { ChangePartNumberInput } from "../../🌱change-part-number-input/🦠️mutation/🟦️component.ts";

export type RemovePartNumberInputInverse = ChangePartNumberInput;
