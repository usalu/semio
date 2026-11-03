/** ➖️ `remove-fire-exposure` wire twin: the leaf payload `RemoveFireExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFireExposure {
  index: number;
}

export const parseRemoveFireExposure: NormWireReader<RemoveFireExposure> = normWireObject<RemoveFireExposure>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
