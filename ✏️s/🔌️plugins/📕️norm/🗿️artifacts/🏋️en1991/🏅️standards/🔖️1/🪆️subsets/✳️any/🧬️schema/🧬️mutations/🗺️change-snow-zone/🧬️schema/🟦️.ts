/** 🗺️ `change-snow-zone` wire twin: the leaf payload `ChangeSnowZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSnowZone {
  newSnowZone: string;
}

export const parseChangeSnowZone: NormWireReader<ChangeSnowZone> = normWireObject<ChangeSnowZone>({ newSnowZone: normWireRequired(normWireString) });
