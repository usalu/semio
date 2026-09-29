
/** 🧯️ `semio.typed-operation-fault.v1` — TS twin of Rust `app::TypedOperationFault`: one refusal on the Fault lane as a
 * typed record, decoded by type and never split out of text (schema `🔌️plugin/🧬️schema/🧯️typed-operation-fault/🔣️.json`,
 * cases `🔌️plugin/🧫️fixtures/🧯️typed-operation-fault.json`). `code` is the frozen code a host branches on and renders
 * its localized text by, `origin` who refused, `message` the refuser's own detail. */
export const TYPED_OPERATION_FAULT_SCHEMA_V1 = "semio.typed-operation-fault.v1";
/** 🧭️ The layers a typed-operation fault names as its origin — the value names of Rust `FaultOrigin`. */
export const TYPED_OPERATION_FAULT_ORIGINS_V1 = ["edge", "renderer", "os", "module", "plugin", "app", "extension", "framework"] as const;
export type TypedOperationFaultOriginV1 = (typeof TYPED_OPERATION_FAULT_ORIGINS_V1)[number];
export type TypedOperationFaultV1 = Readonly<{ schema: typeof TYPED_OPERATION_FAULT_SCHEMA_V1; code: string; origin: TypedOperationFaultOriginV1; message: string }>;
/** 📏️ `TYPED_OPERATION_FAULT_CODE_BYTES` (`🔌️plugin/🦀️.rs`) — the longest frozen code, in UTF-8 bytes. */
export const TYPED_OPERATION_FAULT_CODE_BYTES = 64;
/** 📏️ `TYPED_OPERATION_FAULT_BYTES` (`🔌️plugin/🦀️.rs`) — the longest fault message, in UTF-8 bytes. */
export const TYPED_OPERATION_FAULT_MESSAGE_BYTES = 256;
/** 🏷️ The code a host answers a Fault-lane page with when the page carries no record — a broken guest, never a refusal. */
export const TYPED_OPERATION_FAULT_PAGE_INVALID_CODE = "interactive-job.fault-page-invalid";

/** 🔎️ Decodes one Fault-lane payload by type; `null` for bytes that are no record — exactly the four members, this
 * schema, a frozen code of 1…{@link TYPED_OPERATION_FAULT_CODE_BYTES} UTF-8 bytes, a known origin and a message of at
 * most {@link TYPED_OPERATION_FAULT_MESSAGE_BYTES} UTF-8 bytes. */
export function decodeTypedOperationFaultV1(bytes: Uint8Array): TypedOperationFaultV1 | null {
  let value: unknown;
  try {
    value = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  } catch {
    return null;
  }
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const record = value as Readonly<Record<string, unknown>>;
  const { schema, code, origin, message } = record;
  if (Object.keys(record).length !== 4 || schema !== TYPED_OPERATION_FAULT_SCHEMA_V1 || typeof code !== "string" || typeof message !== "string") return null;
  const codeBytes = new TextEncoder().encode(code).length;
  if (codeBytes === 0 || codeBytes > TYPED_OPERATION_FAULT_CODE_BYTES || new TextEncoder().encode(message).length > TYPED_OPERATION_FAULT_MESSAGE_BYTES) return null;
  const known = TYPED_OPERATION_FAULT_ORIGINS_V1.find((name) => name === origin);
  return known === undefined ? null : { schema: TYPED_OPERATION_FAULT_SCHEMA_V1, code, origin: known, message };
}

/** 🧯️ The refusal one Fault-lane page states: its record, or — for a page that carries none — a framework record naming
 * the broken page, so no consumer ever reads a refusal out of free text. */
export function typedOperationPageFaultV1(page: Readonly<{ payload: Uint8Array }>): TypedOperationFaultV1 {
  return decodeTypedOperationFaultV1(page.payload) ?? { schema: TYPED_OPERATION_FAULT_SCHEMA_V1, code: TYPED_OPERATION_FAULT_PAGE_INVALID_CODE, origin: "framework", message: new TextDecoder().decode(page.payload).slice(0, TYPED_OPERATION_FAULT_MESSAGE_BYTES) };
}
