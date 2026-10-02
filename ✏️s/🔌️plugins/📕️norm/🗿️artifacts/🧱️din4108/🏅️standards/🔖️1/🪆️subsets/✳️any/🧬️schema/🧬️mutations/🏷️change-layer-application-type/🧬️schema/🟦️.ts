/** 🏷️ `change-layer-application-type` wire twin: the leaf payload `ChangeLayerApplicationType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerApplicationType {
  elementId: string;
  index: number;
  newApplicationType: string;
}

export const parseChangeLayerApplicationType: NormWireReader<ChangeLayerApplicationType> = normWireObject<ChangeLayerApplicationType>({ elementId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newApplicationType: normWireRequired(normWireString) });
