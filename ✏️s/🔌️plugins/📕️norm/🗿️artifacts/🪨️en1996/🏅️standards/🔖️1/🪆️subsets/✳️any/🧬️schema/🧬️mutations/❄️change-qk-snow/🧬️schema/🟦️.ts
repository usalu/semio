/** ❄️ `change-qk-snow` wire twin: the leaf payload `ChangeQKSnow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeQKSnow {
  wallId: string;
  id: string;
  newQKSnowPa: number;
}

export const parseChangeQKSnow: NormWireReader<ChangeQKSnow> = normWireObject<ChangeQKSnow>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQKSnowPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
