/** 🟤 `change-zone-volume-m3` wire twin: the leaf payload `ChangeZoneVolumeM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneVolumeM3 {
  mutation: "changezonevolumem3";
  zoneId: string;
  newVolumeM3: number;
}

export const parseChangeZoneVolumeM3: NormWireReader<ChangeZoneVolumeM3> = normWireObject<ChangeZoneVolumeM3>({ mutation: normWireRequired(normWireLiteral("changezonevolumem3")), zoneId: normWireRequired(normWireString), newVolumeM3: normWireRequired(normWireNumber) });
