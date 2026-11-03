/** 🏗️ `change-crane-rail-fixed` wire twin: the leaf payload `ChangeCraneRailFixed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneRailFixed {
  id: string;
  newRailFixed: number;
}

export const parseChangeCraneRailFixed: NormWireReader<ChangeCraneRailFixed> = normWireObject<ChangeCraneRailFixed>({ id: normWireRequired(normWireString), newRailFixed: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
