/** 🛢️ `change-silo-boundary-factor-axial` wire twin: the leaf payload `ChangeSiloBoundaryFactorAxial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloBoundaryFactorAxial {
  id: string;
  newBoundaryFactorAxial: number;
}

export const parseChangeSiloBoundaryFactorAxial: NormWireReader<ChangeSiloBoundaryFactorAxial> = normWireObject<ChangeSiloBoundaryFactorAxial>({ id: normWireRequired(normWireString), newBoundaryFactorAxial: normWireRequired(normWireNumber) });
