/** 🗺️ `change-fire-compartment-area` wire twin: the leaf payload `ChangeFireCompartmentArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireCompartmentArea {
  newFireCompartmentArea: number;
}

export const parseChangeFireCompartmentArea: NormWireReader<ChangeFireCompartmentArea> = normWireObject<ChangeFireCompartmentArea>({ newFireCompartmentArea: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
