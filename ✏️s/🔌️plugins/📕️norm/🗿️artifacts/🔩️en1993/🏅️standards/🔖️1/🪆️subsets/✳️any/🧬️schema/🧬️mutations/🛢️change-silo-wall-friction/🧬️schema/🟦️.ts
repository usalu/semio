/** 🛢️ `change-silo-wall-friction` wire twin: the leaf payload `ChangeSiloWallFriction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloWallFriction {
  id: string;
  newWallFriction: number;
}

export const parseChangeSiloWallFriction: NormWireReader<ChangeSiloWallFriction> = normWireObject<ChangeSiloWallFriction>({ id: normWireRequired(normWireString), newWallFriction: normWireRequired(normWireNumber) });
