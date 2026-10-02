/** ⬜️ `update-hss-inputs` wire twin: the leaf payload `UpdateHssInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type LoadCase, parseLoadCase } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateHssInputs {
  loadCase: LoadCase;
}

export const parseUpdateHssInputs: NormWireReader<UpdateHssInputs> = normWireObject<UpdateHssInputs>({ loadCase: normWireRequired(parseLoadCase) });
