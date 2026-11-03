/** 🧱️ `change-plated-compression-stress` wire twin: the leaf payload `ChangePlatedCompressionStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedCompressionStress {
  id: string;
  newCompressionStress: number;
}

export const parseChangePlatedCompressionStress: NormWireReader<ChangePlatedCompressionStress> = normWireObject<ChangePlatedCompressionStress>({ id: normWireRequired(normWireString), newCompressionStress: normWireRequired(normWireNumber) });
