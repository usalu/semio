/** 🧱️ `change-beam-concrete-e-cm-pa` wire twin: the leaf payload `ChangeBeamConcreteECmPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamConcreteECmPa {
  id: string;
  newConcreteECmPa: number;
}

export const parseChangeBeamConcreteECmPa: NormWireReader<ChangeBeamConcreteECmPa> = normWireObject<ChangeBeamConcreteECmPa>({ id: normWireRequired(normWireString), newConcreteECmPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
