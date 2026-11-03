/** 📏️ `change-member-end-moment-ratio-psi` wire twin: the leaf payload `ChangeMemberEndMomentRatioPsi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberEndMomentRatioPsi {
  id: string;
  newEndMomentRatioPsi: number;
}

export const parseChangeMemberEndMomentRatioPsi: NormWireReader<ChangeMemberEndMomentRatioPsi> = normWireObject<ChangeMemberEndMomentRatioPsi>({ id: normWireRequired(normWireString), newEndMomentRatioPsi: normWireRequired(normWireNumber) });
