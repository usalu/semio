/** 📐 `change-fire-compartment-height` wire twin: the leaf payload `ChangeFireCompartmentHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireCompartmentHeight {
  newFireCompartmentHeight: number;
}

export const parseChangeFireCompartmentHeight: NormWireReader<ChangeFireCompartmentHeight> = normWireObject<ChangeFireCompartmentHeight>({ newFireCompartmentHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
