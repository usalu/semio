/** 🔥️ `update-fire-inputs` wire twin: the leaf payload `UpdateFireInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FireExposure, parseFireExposure } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateFireInputs {
  fireExposure: FireExposure;
}

export const parseUpdateFireInputs: NormWireReader<UpdateFireInputs> = normWireObject<UpdateFireInputs>({ fireExposure: normWireRequired(parseFireExposure) });
