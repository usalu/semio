/** 🏗️ `change-crane-rail-inertia` wire twin: the leaf payload `ChangeCraneRailInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneRailInertia {
  id: string;
  newRailInertia: number;
}

export const parseChangeCraneRailInertia: NormWireReader<ChangeCraneRailInertia> = normWireObject<ChangeCraneRailInertia>({ id: normWireRequired(normWireString), newRailInertia: normWireRequired(normWireNumber) });
