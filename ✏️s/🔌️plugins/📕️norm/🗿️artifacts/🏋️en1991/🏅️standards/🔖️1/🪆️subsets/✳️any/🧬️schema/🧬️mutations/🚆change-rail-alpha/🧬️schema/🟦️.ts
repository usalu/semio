/** 🚆 `change-rail-alpha` wire twin: the leaf payload `ChangeRailAlpha`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailAlpha {
  newRailAlpha: number;
}

export const parseChangeRailAlpha: NormWireReader<ChangeRailAlpha> = normWireObject<ChangeRailAlpha>({ newRailAlpha: normWireRequired(normWireRange(normWireNumber, {"minimum":0.75,"maximum":1.46})) });
