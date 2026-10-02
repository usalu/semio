/** 📐 `change-fire-compartment-height` wire twin: the leaf payload `ChangeFireCompartmentHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireCompartmentHeight {
  newFireCompartmentHeight: number;
}

export const parseChangeFireCompartmentHeight: NormWireReader<ChangeFireCompartmentHeight> = normWireObject<ChangeFireCompartmentHeight>({ newFireCompartmentHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
