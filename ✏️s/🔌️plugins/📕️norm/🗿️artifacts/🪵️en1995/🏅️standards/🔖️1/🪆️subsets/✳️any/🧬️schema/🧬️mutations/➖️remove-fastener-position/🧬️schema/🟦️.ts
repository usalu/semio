/** ➖️ `remove-fastener-position` wire twin: the leaf payload `RemoveFastenerPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFastenerPosition {
  fastenerId: string;
  id: string;
}

export const parseRemoveFastenerPosition: NormWireReader<RemoveFastenerPosition> = normWireObject<RemoveFastenerPosition>({ fastenerId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
