/** 📍️ `change-isolation-system-strength-n` wire twin: the leaf payload `ChangeIsolationSystemStrengthN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsolationSystemStrengthN {
  mutation: "changeIsolationSystemStrengthN";
  id: string;
  newStrengthN: number;
}

export const parseChangeIsolationSystemStrengthN: NormWireReader<ChangeIsolationSystemStrengthN> = normWireObject<ChangeIsolationSystemStrengthN>({ mutation: normWireRequired(normWireLiteral("changeIsolationSystemStrengthN")), id: normWireRequired(normWireString), newStrengthN: normWireRequired(normWireNumber) });
