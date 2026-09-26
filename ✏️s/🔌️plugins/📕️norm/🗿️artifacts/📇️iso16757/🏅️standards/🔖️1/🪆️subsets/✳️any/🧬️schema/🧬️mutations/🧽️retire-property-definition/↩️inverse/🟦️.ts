/** ↩️ inverse for `RetirePropertyDefinition` — undo re-`create`s the definition from BASE state,
 * mirroring `IntroducePropertyDefinition` (not `RetirePropertyDefinition`). */
import type { IntroducePropertyDefinition } from "../../📐️introduce-property-definition/🦠️mutation/🟦️.ts";

export type RetirePropertyDefinitionInverse = IntroducePropertyDefinition;
