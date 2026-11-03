/** 🛢️ `change-silo-boundary-factor-shear` wire twin: the leaf payload `ChangeSiloBoundaryFactorShear`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloBoundaryFactorShear {
  id: string;
  newBoundaryFactorShear: number;
}

export const parseChangeSiloBoundaryFactorShear: NormWireReader<ChangeSiloBoundaryFactorShear> = normWireObject<ChangeSiloBoundaryFactorShear>({ id: normWireRequired(normWireString), newBoundaryFactorShear: normWireRequired(normWireNumber) });
