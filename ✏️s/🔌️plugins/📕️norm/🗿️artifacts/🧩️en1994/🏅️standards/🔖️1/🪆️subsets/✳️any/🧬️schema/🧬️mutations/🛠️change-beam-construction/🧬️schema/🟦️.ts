/** 🛠️ `change-beam-construction` wire twin: the leaf payload `ChangeBeamConstruction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConstructionMethod, parseConstructionMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBeamConstruction {
  id: string;
  newConstruction: ConstructionMethod;
}

export const parseChangeBeamConstruction: NormWireReader<ChangeBeamConstruction> = normWireObject<ChangeBeamConstruction>({ id: normWireRequired(normWireString), newConstruction: normWireRequired(parseConstructionMethod) });
