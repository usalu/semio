/** 🧱 `change-unit-width` wire twin: the leaf payload `ChangeUnitWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUnitWidth {
  id: string;
  newUnitWidthM: number;
}

export const parseChangeUnitWidth: NormWireReader<ChangeUnitWidth> = normWireObject<ChangeUnitWidth>({ id: normWireRequired(normWireString), newUnitWidthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
