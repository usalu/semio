/** 🪢️ `update-tension-component-inputs` wire twin: the leaf payload `UpdateTensionComponentInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTensionComponent, type TensionComponent } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateTensionComponentInputs {
  tensionComponent: TensionComponent;
}

export const parseUpdateTensionComponentInputs: NormWireReader<UpdateTensionComponentInputs> = normWireObject<UpdateTensionComponentInputs>({ tensionComponent: normWireRequired(parseTensionComponent) });
