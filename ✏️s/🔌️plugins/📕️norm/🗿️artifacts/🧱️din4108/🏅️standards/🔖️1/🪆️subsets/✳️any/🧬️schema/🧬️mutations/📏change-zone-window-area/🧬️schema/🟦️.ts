/** 📏 `change-zone-window-area` wire twin: the leaf payload `ChangeZoneWindowArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneWindowArea {
  zoneId: string;
  windowId: string;
  newAreaM2: number;
}

export const parseChangeZoneWindowArea: NormWireReader<ChangeZoneWindowArea> = normWireObject<ChangeZoneWindowArea>({ zoneId: normWireRequired(normWireString), windowId: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
