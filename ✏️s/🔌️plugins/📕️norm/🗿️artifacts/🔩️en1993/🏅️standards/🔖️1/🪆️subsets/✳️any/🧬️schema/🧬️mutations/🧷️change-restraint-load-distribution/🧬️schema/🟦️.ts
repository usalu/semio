/** 🧷️ `change-restraint-load-distribution` wire twin: the leaf payload `ChangeRestraintLoadDistribution`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RestraintDistribution, parseRestraintDistribution } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeRestraintLoadDistribution {
  id: string;
  newLoadDistribution: RestraintDistribution;
}

export const parseChangeRestraintLoadDistribution: NormWireReader<ChangeRestraintLoadDistribution> = normWireObject<ChangeRestraintLoadDistribution>({ id: normWireRequired(normWireString), newLoadDistribution: normWireRequired(parseRestraintDistribution) });
