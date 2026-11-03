/** 🪁 `change-wind-zone` wire twin: the leaf payload `ChangeWindZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindZone {
  newWindZone: number;
}

export const parseChangeWindZone: NormWireReader<ChangeWindZone> = normWireObject<ChangeWindZone>({ newWindZone: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":4})) });
