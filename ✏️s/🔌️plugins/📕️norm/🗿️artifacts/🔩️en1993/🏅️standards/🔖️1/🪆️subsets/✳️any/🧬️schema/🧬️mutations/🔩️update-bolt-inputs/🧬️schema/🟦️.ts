/** 🔩️ `update-bolt-inputs` wire twin: the leaf payload `UpdateBoltInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelJoint, type SteelJoint } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateBoltInputs {
  joint: SteelJoint;
}

export const parseUpdateBoltInputs: NormWireReader<UpdateBoltInputs> = normWireObject<UpdateBoltInputs>({ joint: normWireRequired(parseSteelJoint) });
