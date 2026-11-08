/** ➖️ `remove-critical-region` wire twin: the leaf payload `RemoveCriticalRegion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCriticalRegion {
  mutation: "removeCriticalRegion";
  id: string;
}

export const parseRemoveCriticalRegion: NormWireReader<RemoveCriticalRegion> = normWireObject<RemoveCriticalRegion>({ mutation: normWireRequired(normWireLiteral("removeCriticalRegion")), id: normWireRequired(normWireString) });
