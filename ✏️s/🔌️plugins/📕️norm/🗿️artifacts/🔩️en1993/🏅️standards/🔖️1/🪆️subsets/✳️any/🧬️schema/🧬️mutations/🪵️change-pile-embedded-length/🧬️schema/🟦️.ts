/** 🪵️ `change-pile-embedded-length` wire twin: the leaf payload `ChangePileEmbeddedLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileEmbeddedLength {
  id: string;
  newEmbeddedLength: number;
}

export const parseChangePileEmbeddedLength: NormWireReader<ChangePileEmbeddedLength> = normWireObject<ChangePileEmbeddedLength>({ id: normWireRequired(normWireString), newEmbeddedLength: normWireRequired(normWireNumber) });
