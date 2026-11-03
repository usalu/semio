/** ↔️ `change-in-plane-shear` wire twin: the leaf payload `ChangeInPlaneShear`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeInPlaneShear {
  id: string;
  newInPlaneShearN: number;
}

export const parseChangeInPlaneShear: NormWireReader<ChangeInPlaneShear> = normWireObject<ChangeInPlaneShear>({ id: normWireRequired(normWireString), newInPlaneShearN: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})) });
