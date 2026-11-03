/** 🧱️ `change-masonry-wall-ratio` wire twin: the leaf payload `ChangeMasonryWallRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMasonryWallRatio {
  mutation: "changeMasonryWallRatio";
  id: string;
  newMasonryWallAreaRatio: number;
}

export const parseChangeMasonryWallRatio: NormWireReader<ChangeMasonryWallRatio> = normWireObject<ChangeMasonryWallRatio>({ mutation: normWireRequired(normWireLiteral("changeMasonryWallRatio")), id: normWireRequired(normWireString), newMasonryWallAreaRatio: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
