/** 🕸️ `change-frame-bar-release-start` wire twin: the leaf payload `ChangeFrameBarReleaseStart`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameBarRelease, parseFrameBarRelease } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFrameBarReleaseStart {
  id: string;
  newReleaseStart: FrameBarRelease;
}

export const parseChangeFrameBarReleaseStart: NormWireReader<ChangeFrameBarReleaseStart> = normWireObject<ChangeFrameBarReleaseStart>({ id: normWireRequired(normWireString), newReleaseStart: normWireRequired(parseFrameBarRelease) });
