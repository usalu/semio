/** ➖️ `remove-layer` wire twin: the leaf payload `RemoveLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLayer {
  mutation: "removeLayer";
  index: number;
}

export const parseRemoveLayer: NormWireReader<RemoveLayer> = normWireObject<RemoveLayer>({ mutation: normWireRequired(normWireLiteral("removeLayer")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
