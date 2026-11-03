/** ↔️ `change-footing-width` wire twin: the leaf payload `ChangeFootingWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootingWidth {
  mutation: "changeFootingWidth";
  id: string;
  newWidth: number;
}

export const parseChangeFootingWidth: NormWireReader<ChangeFootingWidth> = normWireObject<ChangeFootingWidth>({ mutation: normWireRequired(normWireLiteral("changeFootingWidth")), id: normWireRequired(normWireString), newWidth: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
