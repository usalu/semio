/** ✏️ `change-tank-content-qk-n` wire twin: the leaf payload `ChangeTankContentQkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankContentQkN {
  mutation: "changeTankContentQkN";
  id: string;
  newContentQkN: number;
}

export const parseChangeTankContentQkN: NormWireReader<ChangeTankContentQkN> = normWireObject<ChangeTankContentQkN>({ mutation: normWireRequired(normWireLiteral("changeTankContentQkN")), id: normWireRequired(normWireString), newContentQkN: normWireRequired(normWireNumber) });
