/** ↕️ `update-through-thickness-inputs` wire twin: the leaf payload `UpdateThroughThicknessInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelSection, type SteelSection } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateThroughThicknessInputs {
  section: SteelSection;
}

export const parseUpdateThroughThicknessInputs: NormWireReader<UpdateThroughThicknessInputs> = normWireObject<UpdateThroughThicknessInputs>({ section: normWireRequired(parseSteelSection) });
