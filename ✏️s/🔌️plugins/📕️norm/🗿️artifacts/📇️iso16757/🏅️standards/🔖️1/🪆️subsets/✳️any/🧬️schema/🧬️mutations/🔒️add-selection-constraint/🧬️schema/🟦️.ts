/** 🔒️ `add-selection-constraint` wire twin: the leaf payload `AddSelectionConstraint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSelectionConstraint, type SelectionConstraint } from "../../../📸️snapshot/🟦️.ts";

export interface AddSelectionConstraint {
  constraint: SelectionConstraint;
}

export const parseAddSelectionConstraint: NormWireReader<AddSelectionConstraint> = normWireObject<AddSelectionConstraint>({ constraint: normWireRequired(parseSelectionConstraint) });
