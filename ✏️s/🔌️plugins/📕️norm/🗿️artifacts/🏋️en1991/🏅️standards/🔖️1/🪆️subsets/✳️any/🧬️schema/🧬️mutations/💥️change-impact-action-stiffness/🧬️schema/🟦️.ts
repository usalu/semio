/** 💥 `change-impact-action-stiffness` wire twin: the leaf payload `ChangeImpactActionStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionStiffness {
  id: string;
  newStiffness: number;
}

export const parseChangeImpactActionStiffness: NormWireReader<ChangeImpactActionStiffness> = normWireObject<ChangeImpactActionStiffness>({ id: normWireRequired(normWireString), newStiffness: normWireRequired(normWireNumber) });
