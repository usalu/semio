/** 🧷 `change-anchor-as` wire twin: the leaf payload `ChangeAnchorAs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorAs {
  anchorId: string;
  newValue: number;
}

export const parseChangeAnchorAs: NormWireReader<ChangeAnchorAs> = normWireObject<ChangeAnchorAs>({ anchorId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
