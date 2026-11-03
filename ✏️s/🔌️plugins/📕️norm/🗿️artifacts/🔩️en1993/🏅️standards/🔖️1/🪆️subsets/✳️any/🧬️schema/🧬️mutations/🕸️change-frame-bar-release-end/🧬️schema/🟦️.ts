/** 🕸️ `change-frame-bar-release-end` wire twin: the leaf payload `ChangeFrameBarReleaseEnd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameBarRelease, parseFrameBarRelease } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFrameBarReleaseEnd {
  id: string;
  newReleaseEnd: FrameBarRelease;
}

export const parseChangeFrameBarReleaseEnd: NormWireReader<ChangeFrameBarReleaseEnd> = normWireObject<ChangeFrameBarReleaseEnd>({ id: normWireRequired(normWireString), newReleaseEnd: normWireRequired(parseFrameBarRelease) });
