/** 🏷️ `change-wall-label-de` wire twin: the leaf payload `ChangeWallLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallLabelDe {
  index: number;
  newLabelDe: string;
}

export const parseChangeWallLabelDe: NormWireReader<ChangeWallLabelDe> = normWireObject<ChangeWallLabelDe>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newLabelDe: normWireRequired(normWireString) });
