/** ➖ `remove-zone` wire twin: the leaf payload `RemoveZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface RemoveZone {
  mutation: "removezone";
  id: string;
}

export const parseRemoveZone: NormWireReader<RemoveZone> = normWireObject<RemoveZone>({ mutation: normWireRequired(normWireLiteral("removezone")), id: normWireRequired(normWireString) });
