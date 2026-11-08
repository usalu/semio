/** 📍️ `change-isolation-system-capacity-m` wire twin: the leaf payload `ChangeIsolationSystemCapacityM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsolationSystemCapacityM {
  mutation: "changeIsolationSystemCapacityM";
  id: string;
  newCapacityM: number;
}

export const parseChangeIsolationSystemCapacityM: NormWireReader<ChangeIsolationSystemCapacityM> = normWireObject<ChangeIsolationSystemCapacityM>({ mutation: normWireRequired(normWireLiteral("changeIsolationSystemCapacityM")), id: normWireRequired(normWireString), newCapacityM: normWireRequired(normWireNumber) });
