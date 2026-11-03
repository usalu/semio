/** 🧪️ `change-lab-point-normal-stress` wire twin: the leaf payload `ChangeLabPointNormalStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabPointNormalStress {
  mutation: "changeLabPointNormalStress";
  labTestId: string;
  id: string;
  newNormalStress: number;
}

export const parseChangeLabPointNormalStress: NormWireReader<ChangeLabPointNormalStress> = normWireObject<ChangeLabPointNormalStress>({ mutation: normWireRequired(normWireLiteral("changeLabPointNormalStress")), labTestId: normWireRequired(normWireString), id: normWireRequired(normWireString), newNormalStress: normWireRequired(normWireNumber) });
