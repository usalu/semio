/** 🟣 `change-zone-area-m2` wire twin: the leaf payload `ChangeZoneAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneAreaM2 {
  mutation: "changezoneaream2";
  zoneId: string;
  newAreaM2: number;
}

export const parseChangeZoneAreaM2: NormWireReader<ChangeZoneAreaM2> = normWireObject<ChangeZoneAreaM2>({ mutation: normWireRequired(normWireLiteral("changezoneaream2")), zoneId: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireNumber) });
