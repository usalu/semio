/** ➖️ `remove-crane-runway` wire twin: the leaf payload `RemoveCraneRunway`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCraneRunway {
  index: number;
}

export const parseRemoveCraneRunway: NormWireReader<RemoveCraneRunway> = normWireObject<RemoveCraneRunway>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
