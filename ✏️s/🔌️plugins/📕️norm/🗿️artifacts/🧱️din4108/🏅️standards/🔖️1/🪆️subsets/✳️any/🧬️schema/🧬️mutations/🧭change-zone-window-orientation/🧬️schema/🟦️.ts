/** 🧭 `change-zone-window-orientation` wire twin: the leaf payload `ChangeZoneWindowOrientation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneWindowOrientation {
  zoneId: string;
  windowId: string;
  newOrientation: string;
}

export const parseChangeZoneWindowOrientation: NormWireReader<ChangeZoneWindowOrientation> = normWireObject<ChangeZoneWindowOrientation>({ zoneId: normWireRequired(normWireString), windowId: normWireRequired(normWireString), newOrientation: normWireRequired(normWireString) });
