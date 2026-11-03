/** 🧪️ `change-lab-test-layer-id` wire twin: the leaf payload `ChangeLabTestLayerId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestLayerId {
  mutation: "changeLabTestLayerId";
  id: string;
  newLayerId: string;
}

export const parseChangeLabTestLayerId: NormWireReader<ChangeLabTestLayerId> = normWireObject<ChangeLabTestLayerId>({ mutation: normWireRequired(normWireLiteral("changeLabTestLayerId")), id: normWireRequired(normWireString), newLayerId: normWireRequired(normWireString) });
