import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

type Spelled = Readonly<Record<string, unknown>>;
type Fixture = Readonly<{
  limits: Readonly<{ codeBytes: number; messageBytes: number; pageBytes: number; untypedCode: string }>;
  records: readonly Readonly<{ name: string; fault: Spelled; record: Spelled }>[];
  decodes: readonly Readonly<{ name: string; bytes: string; record: Spelled | null }>[];
}>;

const utf8 = (text: string): number => new TextEncoder().encode(text).length;

/** 🧵️ A fixture string: `name` itself, or its `nameRepeat` spelling (`text` repeated `count` times). */
function spelled(value: Spelled, name: string): string {
  const direct = value[name];
  if (typeof direct === "string") return direct;
  const repeat = value[`${name}Repeat`] as Readonly<{ text: string; count: number }> | undefined;
  assert(repeat, `fixture value names neither ${name} nor ${name}Repeat`);
  return repeat.text.repeat(repeat.count);
}

/** ✂️ The longest prefix of `text` within `bytes` UTF-8 bytes that ends on a character boundary. */
function clip(text: string, bytes: number): string {
  let kept = "";
  let used = 0;
  for (const character of text) {
    const width = utf8(character);
    if (used + width > bytes) break;
    kept += character;
    used += width;
  }
  return kept;
}

/** ⚖️ The AJV twin of the Rust law `app::typed_operation_fault_tests`: every expected record of the language-agnostic
 * typed-operation fault fixture is derived here from the fixture's own rules, validated by AJV against the published
 * record schema, and fits one result page; exactly the stated byte strings decode to a record. */
export function typedOperationFaultOracle(): number {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🧯️typed-operation-fault/🔣️.json", import.meta.url), "utf8")) as Record<string, unknown> & { $id: string };
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧯️typed-operation-fault.json", import.meta.url), "utf8")) as Fixture;
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  const { codeBytes, messageBytes, pageBytes, untypedCode } = fixture.limits;
  const record = (value: Spelled) => ({ schema: schema.$id, code: spelled(value, "code"), origin: value.origin, message: spelled(value, "message") });
  for (const { name, fault, record: expected } of fixture.records) {
    const code = spelled(fault, "code");
    const derived = { schema: schema.$id, code: code.length === 0 || utf8(code) > codeBytes ? untypedCode : code, origin: fault.origin, message: clip(spelled(fault, "message"), messageBytes) };
    assert.deepEqual(derived, record(expected), name);
    assert(validate(derived), `${name}: ${JSON.stringify(validate.errors)}`);
    assert(utf8(JSON.stringify(derived)) <= pageBytes, `${name}: the encoded record fits one result page`);
  }
  for (const { name, bytes, record: expected } of fixture.decodes) {
    let value: unknown = null;
    try {
      value = JSON.parse(bytes);
    } catch {
      value = null;
    }
    const decoded = validate(value) && utf8((value as { code: string }).code) <= codeBytes && utf8((value as { message: string }).message) <= messageBytes ? value : null;
    assert.deepEqual(decoded, expected === null ? null : record(expected), name);
  }
  return fixture.records.length + fixture.decodes.length;
}

if (import.meta.main) console.log(`typed-operation-fault-oracle cases=${typedOperationFaultOracle()}`);
