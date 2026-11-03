/** 🏠️ `change-floor-span` wire twin: the leaf payload `ChangeFloorSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorSpan {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorSpan: NormWireReader<ChangeFloorSpan> = normWireObject<ChangeFloorSpan>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
