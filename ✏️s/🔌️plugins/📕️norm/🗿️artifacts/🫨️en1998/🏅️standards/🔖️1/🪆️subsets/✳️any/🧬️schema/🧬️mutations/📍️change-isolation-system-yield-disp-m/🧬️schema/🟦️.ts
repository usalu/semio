/** 📍️ `change-isolation-system-yield-disp-m` wire twin: the leaf payload `ChangeIsolationSystemYieldDisplacementM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsolationSystemYieldDisplacementM {
  mutation: "changeIsolationSystemYieldDisplacementM";
  id: string;
  newYieldDisplacementM: number;
}

export const parseChangeIsolationSystemYieldDisplacementM: NormWireReader<ChangeIsolationSystemYieldDisplacementM> = normWireObject<ChangeIsolationSystemYieldDisplacementM>({ mutation: normWireRequired(normWireLiteral("changeIsolationSystemYieldDisplacementM")), id: normWireRequired(normWireString), newYieldDisplacementM: normWireRequired(normWireNumber) });
