/** 🛢️ `change-silo-plan-area` wire twin: the leaf payload `ChangeSiloPlanArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloPlanArea {
  id: string;
  newPlanArea: number;
}

export const parseChangeSiloPlanArea: NormWireReader<ChangeSiloPlanArea> = normWireObject<ChangeSiloPlanArea>({ id: normWireRequired(normWireString), newPlanArea: normWireRequired(normWireNumber) });
