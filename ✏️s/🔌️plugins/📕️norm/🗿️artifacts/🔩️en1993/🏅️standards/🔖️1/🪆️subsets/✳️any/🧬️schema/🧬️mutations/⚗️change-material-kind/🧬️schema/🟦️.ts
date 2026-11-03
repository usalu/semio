/** ⚗️ `change-material-kind` wire twin: the leaf payload `ChangeMaterialKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MaterialKind, parseMaterialKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMaterialKind {
  id: string;
  newKind: MaterialKind;
}

export const parseChangeMaterialKind: NormWireReader<ChangeMaterialKind> = normWireObject<ChangeMaterialKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseMaterialKind) });
