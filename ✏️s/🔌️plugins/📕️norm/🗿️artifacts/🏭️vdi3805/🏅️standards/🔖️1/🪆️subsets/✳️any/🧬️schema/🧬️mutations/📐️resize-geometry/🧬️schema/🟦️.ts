/** 📐️ `resize-geometry` wire twin: the leaf payload `ResizeGeometry`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BoundingBox, parseBoundingBox } from "../../../📸️snapshot/🟦️.ts";

export interface ResizeGeometry {
  id: string;
  newBbox: BoundingBox;
}

export const parseResizeGeometry: NormWireReader<ResizeGeometry> = normWireObject<ResizeGeometry>({ id: normWireRequired(normWireString), newBbox: normWireRequired(parseBoundingBox) });
