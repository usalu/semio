/** 📍️ `change-isolation-system-stiffness` wire twin: the leaf payload `ChangeIsolationSystemStiffnessNPerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsolationSystemStiffnessNPerM {
  mutation: "changeIsolationSystemStiffnessNPerM";
  id: string;
  newStiffnessNPerM: number;
}

export const parseChangeIsolationSystemStiffnessNPerM: NormWireReader<ChangeIsolationSystemStiffnessNPerM> = normWireObject<ChangeIsolationSystemStiffnessNPerM>({ mutation: normWireRequired(normWireLiteral("changeIsolationSystemStiffnessNPerM")), id: normWireRequired(normWireString), newStiffnessNPerM: normWireRequired(normWireNumber) });
