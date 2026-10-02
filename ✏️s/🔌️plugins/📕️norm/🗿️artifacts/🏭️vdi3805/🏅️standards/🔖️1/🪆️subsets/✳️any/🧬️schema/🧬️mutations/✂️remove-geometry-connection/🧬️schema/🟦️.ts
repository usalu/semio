/** ✂️ `remove-geometry-connection` wire twin: the leaf payload `RemoveGeometryConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveGeometryConnection {
  id: string;
  connectionId: string;
}

export const parseRemoveGeometryConnection: NormWireReader<RemoveGeometryConnection> = normWireObject<RemoveGeometryConnection>({ id: normWireRequired(normWireString), connectionId: normWireRequired(normWireString) });
