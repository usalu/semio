/** 🛢️ `change-silo-ring-spacing` wire twin: the leaf payload `ChangeSiloRingSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloRingSpacing {
  id: string;
  newRingSpacing: number;
}

export const parseChangeSiloRingSpacing: NormWireReader<ChangeSiloRingSpacing> = normWireObject<ChangeSiloRingSpacing>({ id: normWireRequired(normWireString), newRingSpacing: normWireRequired(normWireNumber) });
