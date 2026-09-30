/** mutation payload — mirrors `ChangePartNumberInput`. */
import type { CatalogueValue } from "../../🟦️.ts";

export interface ChangePartNumberInput {
  key: string;
  newValue: CatalogueValue;
}
