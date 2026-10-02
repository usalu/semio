/** 🏗️ `update-crane-inputs` wire twin: the leaf payload `UpdateCraneInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CraneRunway, parseCraneRunway } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateCraneInputs {
  craneRunway: CraneRunway;
}

export const parseUpdateCraneInputs: NormWireReader<UpdateCraneInputs> = normWireObject<UpdateCraneInputs>({ craneRunway: normWireRequired(parseCraneRunway) });
