/** 🗺️ `change-fire-compartment-area` wire twin: the leaf payload `ChangeFireCompartmentArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireCompartmentArea {
  newFireCompartmentArea: number;
}

export const parseChangeFireCompartmentArea: NormWireReader<ChangeFireCompartmentArea> = normWireObject<ChangeFireCompartmentArea>({ newFireCompartmentArea: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
