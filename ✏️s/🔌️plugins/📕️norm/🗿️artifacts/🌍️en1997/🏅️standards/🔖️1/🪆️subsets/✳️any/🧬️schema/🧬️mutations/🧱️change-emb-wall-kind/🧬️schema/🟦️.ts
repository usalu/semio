/** 🧱 `change-emb-wall-kind` wire twin: the leaf payload `ChangeEmbWallKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type EmbeddedWallKind, parseEmbeddedWallKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeEmbWallKind {
  mutation: "changeEmbWallKind";
  id: string;
  newKind: EmbeddedWallKind;
}

export const parseChangeEmbWallKind: NormWireReader<ChangeEmbWallKind> = normWireObject<ChangeEmbWallKind>({ mutation: normWireRequired(normWireLiteral("changeEmbWallKind")), id: normWireRequired(normWireString), newKind: normWireRequired(parseEmbeddedWallKind) });
