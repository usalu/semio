/** ↩️ inverse for `DeletePropertyDefinition` — undo re-`create`s the definition from BASE state,
 * mirroring `CreatePropertyDefinition` (not `DeletePropertyDefinition`). */
import type { CreatePropertyDefinition } from "../../🌾create-property-definition/🦠️mutation/🟦️component.ts";

export type DeletePropertyDefinitionInverse = CreatePropertyDefinition;
