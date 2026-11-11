/** 🪟 `insert-zone-window` wire twin: the leaf payload `InsertZoneWindow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din4108ZoneWindow, parseDin4108ZoneWindow } from "../../../📸️snapshot/🟦️.ts";

export interface InsertZoneWindow {
  zoneId: string;
  index: number | null;
  window: Din4108ZoneWindow;
}

export const parseInsertZoneWindow: NormWireReader<InsertZoneWindow> = normWireObject<InsertZoneWindow>({ zoneId: normWireRequired(normWireString), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), window: normWireRequired(parseDin4108ZoneWindow) });
