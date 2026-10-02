/** 🎛️ `change-part-number-input` wire twin: the leaf payload `ChangePartNumberInput`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CatalogueValue, parseCatalogueValue } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePartNumberInput {
  key: string;
  newValue: CatalogueValue;
}

export const parseChangePartNumberInput: NormWireReader<ChangePartNumberInput> = normWireObject<ChangePartNumberInput>({ key: normWireRequired(normWireString), newValue: normWireRequired(parseCatalogueValue) });
