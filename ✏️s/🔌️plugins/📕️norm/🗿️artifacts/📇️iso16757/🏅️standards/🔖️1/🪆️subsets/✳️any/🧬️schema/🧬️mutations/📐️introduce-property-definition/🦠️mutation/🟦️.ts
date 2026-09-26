/** mutation payload — mirrors `IntroducePropertyDefinition`. */
import type { PropertyDefinition } from "../../🟦️.ts";

export interface IntroducePropertyDefinition {
  property_definition: PropertyDefinition;
  index?: number;
}
