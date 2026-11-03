/** 🛢️ `change-silo-ring-inertia` wire twin: the leaf payload `ChangeSiloRingInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloRingInertia {
  id: string;
  newRingInertia: number;
}

export const parseChangeSiloRingInertia: NormWireReader<ChangeSiloRingInertia> = normWireObject<ChangeSiloRingInertia>({ id: normWireRequired(normWireString), newRingInertia: normWireRequired(normWireNumber) });
