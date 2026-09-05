/** ↩️ inverse for `ChangePartNumberInput` — restores BASE's value via `change`, or via `remove` if
 * the key was previously absent (this mutation upserts, so a fresh key's undo removes it). */
import type { ChangePartNumberInput } from "../🦠️mutation/🟦️.ts";
import type { RemovePartNumberInput } from "../../🔌️remove-part-number-input/🦠️mutation/🟦️.ts";

export type ChangePartNumberInputInverse = ChangePartNumberInput | RemovePartNumberInput;
