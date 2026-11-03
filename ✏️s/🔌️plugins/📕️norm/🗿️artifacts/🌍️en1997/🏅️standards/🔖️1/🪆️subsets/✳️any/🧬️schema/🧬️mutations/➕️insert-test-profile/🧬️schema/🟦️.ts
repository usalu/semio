/** ➕ `insert-test-profile` wire twin: the leaf payload `InsertTestProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997PileTestProfile, parseEn1997PileTestProfile } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTestProfile {
  mutation: "insertTestProfile";
  pileId: string;
  index: number;
  testProfile: En1997PileTestProfile;
}

export const parseInsertTestProfile: NormWireReader<InsertTestProfile> = normWireObject<InsertTestProfile>({ mutation: normWireRequired(normWireLiteral("insertTestProfile")), pileId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), testProfile: normWireRequired(parseEn1997PileTestProfile) });
