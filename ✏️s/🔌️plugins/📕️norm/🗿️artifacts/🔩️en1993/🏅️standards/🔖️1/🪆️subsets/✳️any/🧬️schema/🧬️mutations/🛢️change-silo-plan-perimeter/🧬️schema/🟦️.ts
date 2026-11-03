/** 🛢️ `change-silo-plan-perimeter` wire twin: the leaf payload `ChangeSiloPlanPerimeter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloPlanPerimeter {
  id: string;
  newPlanPerimeter: number;
}

export const parseChangeSiloPlanPerimeter: NormWireReader<ChangeSiloPlanPerimeter> = normWireObject<ChangeSiloPlanPerimeter>({ id: normWireRequired(normWireString), newPlanPerimeter: normWireRequired(normWireNumber) });
