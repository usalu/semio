/** ↩️ inverse for `ChangePartNumberInput` — restores BASE's value via `change`, or via `remove` if
 * the key was previously absent (this mutation upserts, so a fresh key's undo removes it). */
import type { ChangePartNumberInput } from "../🦠️mutation/🟦️component.ts";
import type { RemovePartNumberInput } from "../../🌿remove-part-number-input/🦠️mutation/🟦️component.ts";

export type ChangePartNumberInputInverse = ChangePartNumberInput | RemovePartNumberInput;
