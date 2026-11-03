/** 🏋️ `change-anchor-action-source` wire twin: the leaf payload `ChangeAnchorActionSource`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionSource, parseActionSource } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnchorActionSource {
  anchorId: string;
  actionId: string;
  newSource: ActionSource;
}

export const parseChangeAnchorActionSource: NormWireReader<ChangeAnchorActionSource> = normWireObject<ChangeAnchorActionSource>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newSource: normWireRequired(parseActionSource) });
