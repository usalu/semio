/** ⭕ `insert-bar-layer` wire twin: the leaf payload `InsertBarLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BarLayer, parseBarLayer } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBarLayer {
  memberId: string;
  index: number;
  barLayer: BarLayer;
}

export const parseInsertBarLayer: NormWireReader<InsertBarLayer> = normWireObject<InsertBarLayer>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), barLayer: normWireRequired(parseBarLayer) });
