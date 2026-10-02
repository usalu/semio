/** 🔌️ `add-geometry-connection` wire twin: the leaf payload `AddGeometryConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseVdi3805ConnectionPoint, type Vdi3805ConnectionPoint } from "../../../📸️snapshot/🟦️.ts";

export interface AddGeometryConnection {
  id: string;
  connection: Vdi3805ConnectionPoint;
}

export const parseAddGeometryConnection: NormWireReader<AddGeometryConnection> = normWireObject<AddGeometryConnection>({ id: normWireRequired(normWireString), connection: normWireRequired(parseVdi3805ConnectionPoint) });
