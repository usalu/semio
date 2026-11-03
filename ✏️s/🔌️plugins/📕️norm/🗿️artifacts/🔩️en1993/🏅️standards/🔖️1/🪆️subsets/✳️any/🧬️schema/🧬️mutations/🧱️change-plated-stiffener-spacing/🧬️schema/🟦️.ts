/** 🧱️ `change-plated-stiffener-spacing` wire twin: the leaf payload `ChangePlatedStiffenerSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerSpacing {
  id: string;
  newStiffenerSpacing: number;
}

export const parseChangePlatedStiffenerSpacing: NormWireReader<ChangePlatedStiffenerSpacing> = normWireObject<ChangePlatedStiffenerSpacing>({ id: normWireRequired(normWireString), newStiffenerSpacing: normWireRequired(normWireNumber) });
