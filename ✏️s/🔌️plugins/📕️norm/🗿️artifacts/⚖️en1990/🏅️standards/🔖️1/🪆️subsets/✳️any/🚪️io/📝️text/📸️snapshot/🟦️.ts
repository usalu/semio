/** 📝️ `En1990SnapshotText` wire twin: the serialized text form this facet's grammar and codec speak, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireString } from "@semio-tech/norm-js/registry/contract";

export type En1990SnapshotText = string;

export const parseEn1990SnapshotText: NormWireReader<En1990SnapshotText> = normWireString;
