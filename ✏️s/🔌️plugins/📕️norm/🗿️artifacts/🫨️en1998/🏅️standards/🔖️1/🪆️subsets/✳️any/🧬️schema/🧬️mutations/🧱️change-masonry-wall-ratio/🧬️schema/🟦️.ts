/** 🧱️ `change-masonry-wall-ratio` wire twin: the leaf payload `ChangeMasonryWallRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMasonryWallRatio {
  mutation: "changeMasonryWallRatio";
  buildingIndex: number;
  newMasonryWallAreaRatio: number;
}

export const parseChangeMasonryWallRatio: NormWireReader<ChangeMasonryWallRatio> = normWireObject<ChangeMasonryWallRatio>({ mutation: normWireRequired(normWireLiteral("changeMasonryWallRatio")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMasonryWallAreaRatio: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
