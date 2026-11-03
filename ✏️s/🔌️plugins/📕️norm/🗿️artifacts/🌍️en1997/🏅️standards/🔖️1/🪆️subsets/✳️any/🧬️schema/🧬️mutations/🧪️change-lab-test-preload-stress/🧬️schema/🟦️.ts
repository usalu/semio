/** 🧪️ `change-lab-test-preload-stress` wire twin: the leaf payload `ChangeLabTestPreloadStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestPreloadStress {
  mutation: "changeLabTestPreloadStress";
  id: string;
  newPreloadStress: number;
}

export const parseChangeLabTestPreloadStress: NormWireReader<ChangeLabTestPreloadStress> = normWireObject<ChangeLabTestPreloadStress>({ mutation: normWireRequired(normWireLiteral("changeLabTestPreloadStress")), id: normWireRequired(normWireString), newPreloadStress: normWireRequired(normWireNumber) });
