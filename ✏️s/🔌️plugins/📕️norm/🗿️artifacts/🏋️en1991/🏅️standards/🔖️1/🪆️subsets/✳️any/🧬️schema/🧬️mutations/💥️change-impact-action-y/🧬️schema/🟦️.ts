/** 💥 `change-impact-action-y` wire twin: the leaf payload `ChangeImpactActionY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionY {
  id: string;
  newY: number;
}

export const parseChangeImpactActionY: NormWireReader<ChangeImpactActionY> = normWireObject<ChangeImpactActionY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
