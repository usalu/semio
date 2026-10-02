/** 🥶️ `update-cold-formed-inputs` wire twin: the leaf payload `UpdateColdFormedInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ColdFormedMember, parseColdFormedMember } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateColdFormedInputs {
  coldFormedMember: ColdFormedMember;
}

export const parseUpdateColdFormedInputs: NormWireReader<UpdateColdFormedInputs> = normWireObject<UpdateColdFormedInputs>({ coldFormedMember: normWireRequired(parseColdFormedMember) });
