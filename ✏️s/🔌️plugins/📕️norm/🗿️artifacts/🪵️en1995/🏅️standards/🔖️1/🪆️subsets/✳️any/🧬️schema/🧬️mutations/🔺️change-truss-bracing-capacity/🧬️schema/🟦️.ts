/** 🔺️ `change-truss-bracing-capacity` wire twin: the leaf payload `ChangeTrussBracingCapacity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussBracingCapacity {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussBracingCapacity: NormWireReader<ChangeTrussBracingCapacity> = normWireObject<ChangeTrussBracingCapacity>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
