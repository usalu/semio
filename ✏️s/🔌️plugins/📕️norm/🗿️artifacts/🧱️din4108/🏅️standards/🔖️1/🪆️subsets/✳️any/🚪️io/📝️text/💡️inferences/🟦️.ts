/** 📝️ `Din4108InferenceText` wire twin: the serialized text form this facet's grammar and codec speak, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireString } from "@semio-tech/norm-js/registry/contract";

export type Din4108InferenceText = string;

export const parseDin4108InferenceText: NormWireReader<Din4108InferenceText> = normWireString;
