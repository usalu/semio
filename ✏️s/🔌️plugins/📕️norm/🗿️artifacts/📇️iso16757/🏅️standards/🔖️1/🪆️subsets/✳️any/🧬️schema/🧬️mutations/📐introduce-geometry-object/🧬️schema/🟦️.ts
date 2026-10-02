/** 📐 `introduce-geometry-object` wire twin: the leaf payload `IntroduceGeometryObject`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type GeometryObject, parseGeometryObject } from "../../../📸️snapshot/🟦️.ts";

export interface IntroduceGeometryObject {
  geometryObject: GeometryObject;
}

export const parseIntroduceGeometryObject: NormWireReader<IntroduceGeometryObject> = normWireObject<IntroduceGeometryObject>({ geometryObject: normWireRequired(parseGeometryObject) });
