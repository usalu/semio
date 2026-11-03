/** 🏗️ `change-crane-rail-radius` wire twin: the leaf payload `ChangeCraneRailRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneRailRadius {
  id: string;
  newRailRadius: number;
}

export const parseChangeCraneRailRadius: NormWireReader<ChangeCraneRailRadius> = normWireObject<ChangeCraneRailRadius>({ id: normWireRequired(normWireString), newRailRadius: normWireRequired(normWireNumber) });
