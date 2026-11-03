/** ⚓️ `change-anchor-cracked` wire twin: the leaf payload `ChangeAnchorCracked`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorCracked {
  anchorId: string;
  newCracked: boolean;
}

export const parseChangeAnchorCracked: NormWireReader<ChangeAnchorCracked> = normWireObject<ChangeAnchorCracked>({ anchorId: normWireRequired(normWireString), newCracked: normWireRequired(normWireBoolean) });
