/** 🕳️ `remove-opening` wire twin: the leaf payload `RemoveOpening`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveOpening {
  memberId: string;
  index: number;
}

export const parseRemoveOpening: NormWireReader<RemoveOpening> = normWireObject<RemoveOpening>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
