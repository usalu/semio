/** 🧪️ `change-lab-point-shear-stress` wire twin: the leaf payload `ChangeLabPointShearStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabPointShearStress {
  mutation: "changeLabPointShearStress";
  labTestId: string;
  id: string;
  newShearStress: number;
}

export const parseChangeLabPointShearStress: NormWireReader<ChangeLabPointShearStress> = normWireObject<ChangeLabPointShearStress>({ mutation: normWireRequired(normWireLiteral("changeLabPointShearStress")), labTestId: normWireRequired(normWireString), id: normWireRequired(normWireString), newShearStress: normWireRequired(normWireNumber) });
