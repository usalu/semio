/** ↩️ inverse for `RetireSubject` — undo re-`create`s the subject from BASE state, mirroring
 * `IntroduceSubject` (not `RetireSubject` — deletion's inverse is a creation). */
import type { IntroduceSubject } from "../../🌳️introduce-subject/🦠️mutation/🟦️.ts";

export type RetireSubjectInverse = IntroduceSubject;
