/** ⛱️ `change-zone-window-shading-fc` wire twin: the leaf payload `ChangeZoneWindowShadingFc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneWindowShadingFc {
  zoneId: string;
  windowId: string;
  newShadingFc: number;
}

export const parseChangeZoneWindowShadingFc: NormWireReader<ChangeZoneWindowShadingFc> = normWireObject<ChangeZoneWindowShadingFc>({ zoneId: normWireRequired(normWireString), windowId: normWireRequired(normWireString), newShadingFc: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
