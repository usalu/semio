/** 🛢️ `change-silo-fabrication-class` wire twin: the leaf payload `ChangeSiloFabricationClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FabricationToleranceClass, parseFabricationToleranceClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSiloFabricationClass {
  id: string;
  newFabricationClass: FabricationToleranceClass;
}

export const parseChangeSiloFabricationClass: NormWireReader<ChangeSiloFabricationClass> = normWireObject<ChangeSiloFabricationClass>({ id: normWireRequired(normWireString), newFabricationClass: normWireRequired(parseFabricationToleranceClass) });
