/** 🛤 `change-rail-dynamic-determinant-length` wire twin: the leaf payload `ChangeRailDynamicDeterminantLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailDynamicDeterminantLength {
  id: string;
  newDeterminantLength: number;
}

export const parseChangeRailDynamicDeterminantLength: NormWireReader<ChangeRailDynamicDeterminantLength> = normWireObject<ChangeRailDynamicDeterminantLength>({ id: normWireRequired(normWireString), newDeterminantLength: normWireRequired(normWireNumber) });
