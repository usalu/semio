/** 🪨 `change-hk-earth` wire twin: the leaf payload `ChangeHKEarth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeHKEarth {
  wallIndex: number;
  index: number;
  newHKEarthN: number;
}

export const parseChangeHKEarth: NormWireReader<ChangeHKEarth> = normWireObject<ChangeHKEarth>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newHKEarthN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
