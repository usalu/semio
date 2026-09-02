/** mutation payload — mirrors `CreatePropertyDefinition`. */
import type { PropertyDefinition } from "../../🟦️.ts";

export interface CreatePropertyDefinition {
  property_definition: PropertyDefinition;
  index?: number;
}
