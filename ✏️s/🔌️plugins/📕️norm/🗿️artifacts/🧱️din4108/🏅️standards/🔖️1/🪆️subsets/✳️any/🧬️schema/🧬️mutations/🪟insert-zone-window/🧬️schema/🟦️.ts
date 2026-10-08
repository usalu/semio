/** 🪟 `insert-zone-window` wire twin: the leaf payload `InsertZoneWindow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ZoneWindow, parseDin4108ZoneWindow } from "../../../📸️snapshot/🟦️.ts";

export interface InsertZoneWindow {
  zoneId: string;
  index?: number | null;
  window: Din4108ZoneWindow;
}

export const parseInsertZoneWindow: NormWireReader<InsertZoneWindow> = normWireObject<InsertZoneWindow>({ zoneId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), window: normWireRequired(parseDin4108ZoneWindow) });
