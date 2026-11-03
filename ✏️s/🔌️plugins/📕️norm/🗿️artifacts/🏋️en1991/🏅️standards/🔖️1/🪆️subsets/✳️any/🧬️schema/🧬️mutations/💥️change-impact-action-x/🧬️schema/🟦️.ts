/** 💥 `change-impact-action-x` wire twin: the leaf payload `ChangeImpactActionX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionX {
  id: string;
  newX: number;
}

export const parseChangeImpactActionX: NormWireReader<ChangeImpactActionX> = normWireObject<ChangeImpactActionX>({ id: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
