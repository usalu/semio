/** 🌙️ `change-night-setback` wire twin: the leaf payload `ChangeNightSetback`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";

export interface ChangeNightSetback {
  newNightSetbackK: number;
}

export const parseChangeNightSetback: NormWireReader<ChangeNightSetback> = normWireObject<ChangeNightSetback>({ newNightSetbackK: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
