/** 🏷️ `change-zone-name` wire twin: the leaf payload `ChangeZoneName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneName {
  zoneId: string;
  newName: string;
}

export const parseChangeZoneName: NormWireReader<ChangeZoneName> = normWireObject<ChangeZoneName>({ zoneId: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
