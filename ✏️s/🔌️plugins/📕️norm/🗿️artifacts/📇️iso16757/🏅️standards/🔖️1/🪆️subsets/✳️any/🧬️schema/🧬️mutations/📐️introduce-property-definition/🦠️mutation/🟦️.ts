/** mutation payload — mirrors `IntroducePropertyDefinition`. */
import type { PropertyDefinition } from "../../🟦️.ts";

export interface IntroducePropertyDefinition {
  propertyDefinition: PropertyDefinition;
  index?: number;
}
