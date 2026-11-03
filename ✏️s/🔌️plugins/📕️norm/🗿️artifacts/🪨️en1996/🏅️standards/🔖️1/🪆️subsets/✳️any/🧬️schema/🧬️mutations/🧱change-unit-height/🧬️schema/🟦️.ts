/** 🧱 `change-unit-height` wire twin: the leaf payload `ChangeUnitHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeUnitHeight {
  id: string;
  newUnitHeightM: number;
}

export const parseChangeUnitHeight: NormWireReader<ChangeUnitHeight> = normWireObject<ChangeUnitHeight>({ id: normWireRequired(normWireString), newUnitHeightM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
