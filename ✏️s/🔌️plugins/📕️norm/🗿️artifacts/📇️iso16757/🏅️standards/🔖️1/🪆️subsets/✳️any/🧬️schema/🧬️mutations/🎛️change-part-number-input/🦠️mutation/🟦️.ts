/** mutation payload — mirrors `ChangePartNumberInput`. */
import type { CatalogueValue } from "../../🟦️.ts";

export interface ChangePartNumberInput {
  key: string;
  new_value: CatalogueValue;
}
