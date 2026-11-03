/** 🏠️ `change-floor-joist-spacing` wire twin: the leaf payload `ChangeFloorJoistSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorJoistSpacing {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorJoistSpacing: NormWireReader<ChangeFloorJoistSpacing> = normWireObject<ChangeFloorJoistSpacing>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
