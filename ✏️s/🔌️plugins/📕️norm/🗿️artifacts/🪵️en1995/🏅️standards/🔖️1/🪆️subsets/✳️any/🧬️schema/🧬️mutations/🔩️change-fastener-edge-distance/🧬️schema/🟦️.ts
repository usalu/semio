/** 🔩️ `change-fastener-edge-distance` wire twin: the leaf payload `ChangeFastenerEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerEdgeDistance {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerEdgeDistance: NormWireReader<ChangeFastenerEdgeDistance> = normWireObject<ChangeFastenerEdgeDistance>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
