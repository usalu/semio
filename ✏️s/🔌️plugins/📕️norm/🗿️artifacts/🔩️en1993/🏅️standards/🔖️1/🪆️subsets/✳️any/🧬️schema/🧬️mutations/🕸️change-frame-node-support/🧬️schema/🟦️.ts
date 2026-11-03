/** 🕸️ `change-frame-node-support` wire twin: the leaf payload `ChangeFrameNodeSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameNodeSupport, parseFrameNodeSupport } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFrameNodeSupport {
  id: string;
  newSupport: FrameNodeSupport;
}

export const parseChangeFrameNodeSupport: NormWireReader<ChangeFrameNodeSupport> = normWireObject<ChangeFrameNodeSupport>({ id: normWireRequired(normWireString), newSupport: normWireRequired(parseFrameNodeSupport) });
