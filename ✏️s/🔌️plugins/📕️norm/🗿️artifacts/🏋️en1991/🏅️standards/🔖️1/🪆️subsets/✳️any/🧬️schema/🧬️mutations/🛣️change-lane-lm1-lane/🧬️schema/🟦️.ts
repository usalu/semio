/** 🛣️ `change-lane-lm1-lane` wire twin: the leaf payload `ChangeLaneLm1Lane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLaneLm1Lane {
  id: string;
  newLm1Lane: number;
}

export const parseChangeLaneLm1Lane: NormWireReader<ChangeLaneLm1Lane> = normWireObject<ChangeLaneLm1Lane>({ id: normWireRequired(normWireString), newLm1Lane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
