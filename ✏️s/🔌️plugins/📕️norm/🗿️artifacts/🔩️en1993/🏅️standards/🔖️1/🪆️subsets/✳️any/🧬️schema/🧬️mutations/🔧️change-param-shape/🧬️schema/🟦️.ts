/** 🔧️ `change-param-shape` wire twin: the leaf payload `ChangeParamShape`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ParametricShape, parseParametricShape } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeParamShape {
  id: string;
  newShape: ParametricShape;
}

export const parseChangeParamShape: NormWireReader<ChangeParamShape> = normWireObject<ChangeParamShape>({ id: normWireRequired(normWireString), newShape: normWireRequired(parseParametricShape) });
