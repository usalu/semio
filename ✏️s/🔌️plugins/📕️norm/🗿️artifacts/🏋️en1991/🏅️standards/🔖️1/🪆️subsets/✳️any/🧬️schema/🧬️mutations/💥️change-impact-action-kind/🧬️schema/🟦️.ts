/** 💥 `change-impact-action-kind` wire twin: the leaf payload `ChangeImpactActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImpactActionKind {
  id: string;
  newKind: number;
}

export const parseChangeImpactActionKind: NormWireReader<ChangeImpactActionKind> = normWireObject<ChangeImpactActionKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
