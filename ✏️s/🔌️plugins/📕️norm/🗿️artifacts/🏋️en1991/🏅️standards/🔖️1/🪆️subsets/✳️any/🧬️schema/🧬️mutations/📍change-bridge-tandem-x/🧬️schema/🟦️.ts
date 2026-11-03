/** 📍 `change-bridge-tandem-x` wire twin: the leaf payload `ChangeBridgeTandemX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeTandemX {
  newBridgeTandemX: number;
}

export const parseChangeBridgeTandemX: NormWireReader<ChangeBridgeTandemX> = normWireObject<ChangeBridgeTandemX>({ newBridgeTandemX: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
