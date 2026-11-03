/** ➖️ `remove-bracing` wire twin: the leaf payload `RemoveBracing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBracing {
  index: number;
}

export const parseRemoveBracing: NormWireReader<RemoveBracing> = normWireObject<RemoveBracing>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
