/** 🧱️ `change-plated-stiffener-shape` wire twin: the leaf payload `ChangePlatedStiffenerShape`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StiffenerShape, parseStiffenerShape } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePlatedStiffenerShape {
  id: string;
  newStiffenerShape: StiffenerShape;
}

export const parseChangePlatedStiffenerShape: NormWireReader<ChangePlatedStiffenerShape> = normWireObject<ChangePlatedStiffenerShape>({ id: normWireRequired(normWireString), newStiffenerShape: normWireRequired(parseStiffenerShape) });
