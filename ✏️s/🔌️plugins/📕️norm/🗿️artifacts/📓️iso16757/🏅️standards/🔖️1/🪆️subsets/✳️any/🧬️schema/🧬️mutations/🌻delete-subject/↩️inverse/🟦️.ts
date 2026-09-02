/** ↩️ inverse for `DeleteSubject` — undo re-`create`s the subject from BASE state, mirroring
 * `CreateSubject` (not `DeleteSubject` — deletion's inverse is a creation). */
import type { CreateSubject } from "../../🌵create-subject/🦠️mutation/🟦️.ts";

export type DeleteSubjectInverse = CreateSubject;
