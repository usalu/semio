/** 💥 `change-impact-action-speed` wire twin: the leaf payload `ChangeImpactActionSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionSpeed {
  id: string;
  newSpeed: number;
}

export const parseChangeImpactActionSpeed: NormWireReader<ChangeImpactActionSpeed> = normWireObject<ChangeImpactActionSpeed>({ id: normWireRequired(normWireString), newSpeed: normWireRequired(normWireNumber) });
