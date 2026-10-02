/** 🪵️ `update-pile-inputs` wire twin: the leaf payload `UpdatePileInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelPile, type SteelPile } from "../../../📸️snapshot/🟦️.ts";

export interface UpdatePileInputs {
  pile: SteelPile;
}

export const parseUpdatePileInputs: NormWireReader<UpdatePileInputs> = normWireObject<UpdatePileInputs>({ pile: normWireRequired(parseSteelPile) });
