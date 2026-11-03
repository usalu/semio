/** 🛢️ `change-silo-boundary-factor-hoop` wire twin: the leaf payload `ChangeSiloBoundaryFactorHoop`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloBoundaryFactorHoop {
  id: string;
  newBoundaryFactorHoop: number;
}

export const parseChangeSiloBoundaryFactorHoop: NormWireReader<ChangeSiloBoundaryFactorHoop> = normWireObject<ChangeSiloBoundaryFactorHoop>({ id: normWireRequired(normWireString), newBoundaryFactorHoop: normWireRequired(normWireNumber) });
