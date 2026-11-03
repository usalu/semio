/** 🏭️ `change-masonry-class` wire twin: the leaf payload `ChangeMasonryClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996MasonryClass, parseEn1996MasonryClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMasonryClass {
  newMasonryClass: En1996MasonryClass;
}

export const parseChangeMasonryClass: NormWireReader<ChangeMasonryClass> = normWireObject<ChangeMasonryClass>({ newMasonryClass: normWireRequired(parseEn1996MasonryClass) });
