/** ⚠️ Container and schema errors preserve actual owned Value refusal identities. */
import { ValueError } from "../../🌱️value/⚠️refusal/🟦️.ts";
import { TextError } from "../../⚠️diagnostic/🚧️text-error/🟦️.ts";

export type PackErrorData =
  | Readonly<{kind: "BadMagic"}>
  | Readonly<{kind: "UnsupportedVersion"; major: number; minor: number}>
  | Readonly<{kind: "UnknownRequiredFlags"; flags: number}>
  | Readonly<{kind: "Truncated"; offset: bigint}>
  | Readonly<{kind: "ChecksumMismatch"; segment: string; offset: bigint}>
  | Readonly<{kind: "ContentHashMismatch"}>
  | Readonly<{kind: "LimitExceeded"; limit: string}>
  | Readonly<{kind: "RetainedMalformed" | "Malformed"; what: string; offset: bigint; detail: string}>
  | Readonly<{kind: "NonCanonical"; detail: string}>
  | Readonly<{kind: "UnsupportedCodec"; codec: number}>
  | Readonly<{kind: "ValueRefusal"; error: ValueError}>
  | Readonly<{kind: "TextRefusal"; error: TextError}>
  | Readonly<{kind: "Io"; message: string}>;

/** 🗨️ Renders the complete container error taxonomy without classifying message text. */
export function packErrorMessage(data: PackErrorData): string {
  switch (data.kind) {
    case "BadMagic": return "bad magic";
    case "UnsupportedVersion": return `unsupported version ${data.major}.${data.minor}`;
    case "UnknownRequiredFlags": return `unknown required feature bits 0x${data.flags.toString(16)}`;
    case "Truncated": return `truncated at offset ${data.offset}`;
    case "ChecksumMismatch": return `checksum mismatch in ${data.segment} at offset ${data.offset}`;
    case "ContentHashMismatch": return "content hash mismatch";
    case "LimitExceeded": return `limit exceeded: ${data.limit}`;
    case "RetainedMalformed": case "Malformed": return `malformed ${data.what} at offset ${data.offset}: ${data.detail}`;
    case "NonCanonical": return `non-canonical encoding: ${data.detail}`;
    case "UnsupportedCodec": return `unsupported codec ${data.codec}`;
    case "ValueRefusal": return `schema error: ${data.error.message}`;
    case "TextRefusal": return `schema error: ${data.error.toString()}`;
    case "Io": return `io error: ${data.message}`;
  }
}

/** 🪪️ Owns one explicit error payload and retains a typed refusal as its original cause. */
export class PackError extends Error {
  constructor(public readonly data: PackErrorData) {
    super(packErrorMessage(data), {cause: data.kind === "ValueRefusal" || data.kind === "TextRefusal" ? data.error : undefined});
    this.name = "PackError";
  }
  static fromValue(error: ValueError): PackError { return new PackError({kind: "ValueRefusal", error}); }
  static fromText(error: TextError): PackError { return new PackError({kind: "TextRefusal", error}); }
}
