/** ⬇️ `change-footing-embedment` wire twin: the leaf payload `ChangeFootingEmbedment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootingEmbedment {
  mutation: "changeFootingEmbedment";
  id: string;
  newEmbedment: number;
}

export const parseChangeFootingEmbedment: NormWireReader<ChangeFootingEmbedment> = normWireObject<ChangeFootingEmbedment>({ mutation: normWireRequired(normWireLiteral("changeFootingEmbedment")), id: normWireRequired(normWireString), newEmbedment: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
