/** 🏷️ `change-wall-label-de` wire twin: the leaf payload `ChangeWallLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallLabelDe {
  id: string;
  newLabelDe: string;
}

export const parseChangeWallLabelDe: NormWireReader<ChangeWallLabelDe> = normWireObject<ChangeWallLabelDe>({ id: normWireRequired(normWireString), newLabelDe: normWireRequired(normWireString) });
