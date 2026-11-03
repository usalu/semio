/** ✏️ `change-silo-content-qk-n` wire twin: the leaf payload `ChangeSiloContentQkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloContentQkN {
  mutation: "changeSiloContentQkN";
  id: string;
  newContentQkN: number;
}

export const parseChangeSiloContentQkN: NormWireReader<ChangeSiloContentQkN> = normWireObject<ChangeSiloContentQkN>({ mutation: normWireRequired(normWireLiteral("changeSiloContentQkN")), id: normWireRequired(normWireString), newContentQkN: normWireRequired(normWireNumber) });
