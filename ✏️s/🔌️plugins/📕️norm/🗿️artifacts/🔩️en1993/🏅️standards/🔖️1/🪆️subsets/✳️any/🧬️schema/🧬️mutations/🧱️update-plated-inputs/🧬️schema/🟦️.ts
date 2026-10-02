/** 🧱️ `update-plated-inputs` wire twin: the leaf payload `UpdatePlatedInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parsePlatedPanel, type PlatedPanel } from "../../../📸️snapshot/🟦️.ts";

export interface UpdatePlatedInputs {
  platedPanel: PlatedPanel;
}

export const parseUpdatePlatedInputs: NormWireReader<UpdatePlatedInputs> = normWireObject<UpdatePlatedInputs>({ platedPanel: normWireRequired(parsePlatedPanel) });
