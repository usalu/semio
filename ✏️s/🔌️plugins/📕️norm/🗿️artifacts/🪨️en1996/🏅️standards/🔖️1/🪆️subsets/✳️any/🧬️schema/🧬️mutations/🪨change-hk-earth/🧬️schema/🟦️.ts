/** 🪨 `change-hk-earth` wire twin: the leaf payload `ChangeHKEarth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHKEarth {
  wallId: string;
  id: string;
  newHKEarthN: number;
}

export const parseChangeHKEarth: NormWireReader<ChangeHKEarth> = normWireObject<ChangeHKEarth>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newHKEarthN: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
