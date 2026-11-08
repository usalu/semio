/** 🆔️ `change-zone-id` wire twin: the leaf payload `ChangeZoneId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneId {
  zoneId: string;
  newId: string;
}

export const parseChangeZoneId: NormWireReader<ChangeZoneId> = normWireObject<ChangeZoneId>({ zoneId: normWireRequired(normWireString), newId: normWireRequired(normWireString) });
