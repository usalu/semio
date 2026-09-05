#!/usr/bin/env python3
"""🟦️ W3 — generates the TypeScript mirrors of the `🔣️json` io leaves for `◻️2d` and `🖐️5d`.

Second implementation (CLAUDE.md multi-implementation): the writer decides key order, float
formatting and string escaping on its own from the declared field tables, so a byte-for-byte match
against the Rust leaf's `json_text` output is a real cross-language agreement, not an echo.

`🧊️3d` is deliberately NOT generated — see `📓️w3-io.md` §"TypeScript gaps".
"""
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
BLOCK = ROOT / "✏️s/🔌️plugins/🧱️block/🗿️artifacts"

SHARED_TABLES = '''// #region 🧬️SharedRecordTables
/** 🏷️ One JSON member of a record, in the exact order the Rust struct declares it. */
export type BlockJsonField =
  | Readonly<{ key: string; kind: "text" | "float" | "bool" | "textList" | "floatTuple" }>
  | Readonly<{ key: string; kind: "optionalText" | "optionalFloat" | "optionalFloatTuple" }>
  | Readonly<{ key: string; kind: "record"; fields: readonly BlockJsonField[] }>
  | Readonly<{ key: string; kind: "table"; fields: readonly BlockJsonField[] }>;

/** 🪪️ `BlockKindIdentity` (`✏️s/🔌️plugins/🧱️block/🦀️.rs`). */
export const BLOCK_KIND_IDENTITY_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "label", kind: "text" },
  { key: "variant", kind: "optionalText" },
  { key: "description", kind: "text" },
  { key: "icon", kind: "optionalText" },
  { key: "unit", kind: "optionalText" },
];

/** 🏷️ `BlockAttribute`. */
export const BLOCK_ATTRIBUTE_FIELDS: readonly BlockJsonField[] = [
  { key: "key", kind: "text" },
  { key: "value", kind: "text" },
  { key: "definition", kind: "optionalText" },
];

/** 👤️ `BlockAuthor`. */
export const BLOCK_AUTHOR_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "email", kind: "optionalText" },
];

/** 🔗️ `BlockCompatibilityRule`. */
export const BLOCK_COMPATIBILITY_RULE_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "source", kind: "text" },
  { key: "target", kind: "text" },
  { key: "bidirectional", kind: "bool" },
];

/** 🧱️ `BlockRepresentation`. */
export const BLOCK_REPRESENTATION_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "meshUrl", kind: "optionalText" },
  { key: "tags", kind: "textList" },
  { key: "lod", kind: "optionalText" },
  { key: "description", kind: "text" },
  { key: "attributes", kind: "table", fields: BLOCK_ATTRIBUTE_FIELDS },
];

/** 🎥️ `BlockCamera2d`. */
export const BLOCK_CAMERA_2D_FIELDS: readonly BlockJsonField[] = [
  { key: "x", kind: "float" },
  { key: "y", kind: "float" },
  { key: "zoom", kind: "float" },
];

/** 🎥️ `BlockCamera3d`. */
export const BLOCK_CAMERA_3D_FIELDS: readonly BlockJsonField[] = [
  { key: "position", kind: "floatTuple" },
  { key: "target", kind: "floatTuple" },
  { key: "zoom", kind: "float" },
];

/** 📝️ `BlockMeta`. */
export const BLOCK_META_FIELDS: readonly BlockJsonField[] = [{ key: "description", kind: "text" }];
// #endregion 🧬️SharedRecordTables
'''

WRITER = '''// #region 🔢️FloatFormat
/**
 * 🔢️ `f64` → JSON lexeme, byte-identical to the Rust leaf's writer
 * (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs::write_float`): shortest round-tripping digits,
 * fixed notation while the decimal exponent is in `-5..=15` (a whole number always keeping an
 * explicit `.0` so it never collapses onto its integer twin), exponential with a signed `e±` suffix
 * otherwise. `Number.prototype.toExponential()` with no argument is specified to emit exactly the
 * shortest uniquely-identifying digit string, which is the same digit string Rust's `{{:e}}` picks.
 */
export function formatBlockFloat(value: number): string {{
  if (!Number.isFinite(value)) return "null";
  if (value === 0) return Object.is(value, -0) ? "-0.0" : "0.0";
  const negative = value < 0;
  const [mantissa, exponentText] = Math.abs(value).toExponential().split("e");
  const digits = mantissa.replace(".", "");
  const exponent = Number(exponentText);
  const sign = negative ? "-" : "";
  if (exponent >= -5 && exponent <= 15) {{
    if (exponent >= digits.length - 1) return `${{sign}}${{digits}}${{"0".repeat(exponent - (digits.length - 1))}}.0`;
    if (exponent >= 0) return `${{sign}}${{digits.slice(0, exponent + 1)}}.${{digits.slice(exponent + 1)}}`;
    return `${{sign}}0.${{"0".repeat(-exponent - 1)}}${{digits}}`;
  }}
  const fraction = digits.length > 1 ? `.${{digits.slice(1)}}` : "";
  return `${{sign}}${{digits[0]}}${{fraction}}e${{exponent >= 0 ? "+" : "-"}}${{Math.abs(exponent)}}`;
}}
// #endregion 🔢️FloatFormat

// #region 🔤️StringEscape
/**
 * 🔤️ JSON string literal, byte-identical to the Rust writer: only `"`, `\\\\` and the C0 controls are
 * escaped (`\\b \\f \\n \\r \\t` shorthands, `\\u00XX` otherwise); every non-ASCII character passes
 * through raw, matching `serde_json`'s default writer.
 */
export function writeBlockJsonString(text: string): string {{
  let out = '"';
  for (const character of text) {{
    switch (character) {{
      case '"':
        out += '\\\\"';
        break;
      case "\\\\":
        out += "\\\\\\\\";
        break;
      case "\\b":
        out += "\\\\b";
        break;
      case "\\f":
        out += "\\\\f";
        break;
      case "\\n":
        out += "\\\\n";
        break;
      case "\\r":
        out += "\\\\r";
        break;
      case "\\t":
        out += "\\\\t";
        break;
      default:
        out += character < " " ? `\\\\u${{character.codePointAt(0)!.toString(16).padStart(4, "0")}}` : character;
    }}
  }}
  return out + '"';
}}
// #endregion 🔤️StringEscape

// #region ✍️Writer
/** ✍️ Writes one record as compact rfc8259, members in declaration order, optionals skipped. */
export function writeBlockJsonRecord(record: Record<string, unknown>, fields: readonly BlockJsonField[]): string {{
  const members: string[] = [];
  for (const field of fields) {{
    const value = record[field.key];
    if ((field.kind === "optionalText" || field.kind === "optionalFloat" || field.kind === "optionalFloatTuple") && (value === undefined || value === null)) continue;
    members.push(`${{writeBlockJsonString(field.key)}}:${{writeBlockJsonValue(value, field)}}`);
  }}
  return `{{${{members.join(",")}}}}`;
}}

function writeBlockJsonValue(value: unknown, field: BlockJsonField): string {{
  switch (field.kind) {{
    case "text":
    case "optionalText":
      return writeBlockJsonString(String(value ?? ""));
    case "float":
    case "optionalFloat":
      return formatBlockFloat(Number(value ?? 0));
    case "bool":
      return value ? "true" : "false";
    case "textList":
      return `[${{((value ?? []) as string[]).map(writeBlockJsonString).join(",")}}]`;
    case "floatTuple":
    case "optionalFloatTuple":
      return `[${{((value ?? []) as number[]).map(formatBlockFloat).join(",")}}]`;
    case "record":
      return writeBlockJsonRecord((value ?? {{}}) as Record<string, unknown>, field.fields);
    case "table":
      return `[${{((value ?? []) as Record<string, unknown>[]).map((row) => writeBlockJsonRecord(row, field.fields)).join(",")}}]`;
  }}
}}
// #endregion ✍️Writer
'''

EXPORT_TS = '''/** 🚪️ block{dim} → json — TypeScript mirror of the sibling `🦀️.rs` leaf's `json_text`.
 *
 * A genuine second implementation, not a re-export: the field tables below restate the Rust
 * struct declaration order, the writer restates `serde_json`'s escaping and `pack::json`'s float
 * lexeme rule, and `🧪️tests/🟦️.ts` asserts the result is byte-identical to the JSON the Rust leaf
 * produced for every `📚️examples/**\\/🗣️.dsl.semio` fixture (`🧫️fixtures/*.json`).
 */

import type {{ {p}Snapshot }} from "{snapshot_import}";

{shared}
{per_subset_tables}
{writer}
// #region 🎯️Entrypoint
/** 🧵️ `{kind}@1/*` → `s.stdio.json@rfc8259/*` — the exact bytes the Rust `{p}IntoJson` writes. */
export function {fn}ToJsonText(snapshot: {p}Snapshot): string {{
  return writeBlockJsonRecord(snapshot as unknown as Record<string, unknown>, {SNAP}_FIELDS);
}}
// #endregion 🎯️Entrypoint
'''

IMPORT_TS = '''/** 🚪️ block{dim} ← json — TypeScript mirror of the sibling `🦀️.rs` leaf's `from_json_text`, the
 * exact inverse of `{fn}ToJsonText` in the `📤️export` leaf (whose field tables it reuses — one
 * declaration of the schema per subset, never two).
 */

import type {{ {p}Snapshot }} from "{snapshot_import}";
import {{ type BlockJsonField, {SNAP}_FIELDS, writeBlockJsonRecord }} from "{export_import}";

// #region 📥️Reader
/** 🧩️ Rebuilds one record from a parsed JSON object, dropping members the schema does not declare
 * and defaulting every absent non-optional member the way the Rust `FromValue` derive does. */
export function readBlockJsonRecord(value: unknown, fields: readonly BlockJsonField[]): Record<string, unknown> {{
  const source = (value ?? {{}}) as Record<string, unknown>;
  const record: Record<string, unknown> = {{}};
  for (const field of fields) {{
    const member = source[field.key];
    switch (field.kind) {{
      case "text":
        record[field.key] = typeof member === "string" ? member : "";
        break;
      case "optionalText":
        if (typeof member === "string") record[field.key] = member;
        break;
      case "float":
        record[field.key] = typeof member === "number" ? member : 0;
        break;
      case "optionalFloat":
        if (typeof member === "number") record[field.key] = member;
        break;
      case "bool":
        record[field.key] = member === true;
        break;
      case "textList":
        record[field.key] = Array.isArray(member) ? member.map(String) : [];
        break;
      case "floatTuple":
        record[field.key] = Array.isArray(member) ? member.map(Number) : [];
        break;
      case "optionalFloatTuple":
        if (Array.isArray(member)) record[field.key] = member.map(Number);
        break;
      case "record":
        record[field.key] = readBlockJsonRecord(member, field.fields);
        break;
      case "table":
        record[field.key] = (Array.isArray(member) ? member : []).map((row) => readBlockJsonRecord(row, field.fields));
        break;
    }}
  }}
  return record;
}}
// #endregion 📥️Reader

// #region 🎯️Entrypoint
/** 🧩️ `s.stdio.json@rfc8259/*` → `{kind}@1/*`. Throws on text that is not a JSON object. */
export function {fn}FromJsonText(text: string): {p}Snapshot {{
  const parsed: unknown = JSON.parse(text);
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {{
    throw new Error("json→block{dim}: expected a json object at the document root");
  }}
  const snapshot = readBlockJsonRecord(parsed, {SNAP}_FIELDS) as unknown as {p}Snapshot;
  if (!snapshot.schema) snapshot.schema = "{schema_value}";
  return snapshot;
}}

/** 🔁️ Canonical re-encoding of arbitrary `{kind}` json — the fixed point the parity test asserts. */
export function {fn}CanonicalJsonText(text: string): string {{
  return writeBlockJsonRecord({fn}FromJsonText(text) as unknown as Record<string, unknown>, {SNAP}_FIELDS);
}}
// #endregion 🎯️Entrypoint
'''

TEST_TS = '''/** 🧪️ block{dim} io — cross-language parity for the `🔣️json` and `🔤️txt` leaves.
 *
 * `🧫️fixtures/*.json` is the single shared oracle: the Rust `🚪️io/🦀️.rs` test
 * `json_matches_the_typescript_parity_fixture` asserts the same files from `{p}IntoJson`, so a
 * disagreement between the two implementations fails on both sides instead of drifting silently.
 */

import {{ describe, expect, test }} from "bun:test";
import {{ {fn}ToJsonText }} from "../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import {{ {fn}CanonicalJsonText }} from "../📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import {{ {fn}FromDslText }} from "../📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️";

const FIXTURES = [
{fixtures_ts}] as const;

async function read(path: string): Promise<string> {{
  return Bun.file(new URL(path, import.meta.url)).text();
}}

describe("block{dim} io", () => {{
  for (const {{ asset, example }} of FIXTURES) {{
    test(`${{asset}}: the TypeScript json writer is a fixed point on the Rust bytes`, async () => {{
      const expected = await read(`./🧫️fixtures/${{asset}}.json`);
      expect({fn}CanonicalJsonText(expected)).toBe(expected);
    }});

    test(`${{asset}}: the TypeScript dsl reader + json writer reproduce the Rust bytes`, async () => {{
      const expected = await read(`./🧫️fixtures/${{asset}}.json`);
      const dsl = await read(`../../📚️examples/${{example}}/🖼️assets/🧪️${{asset}}/🗣️.dsl.semio`);
      expect({fn}ToJsonText({fn}FromDslText(dsl))).toBe(expected);
    }});
  }}
}});
'''

SUBSETS = [
    dict(dir="◻️2d", dim="2d", p="Block2d", fn="block2d", snap="BLOCK2D_SNAPSHOT",
         schema_value="block.2d",
         kind="s.block.block2d",
         assets=[("hexagonal-cut-concrete-forest-left", "🎬️hexagonal-cut-concrete-forest-left"), ("hexagonal-cut-concrete-forest-right", "➡️hexagonal-cut-concrete-forest-right")],
         tables='''/** 🔵️ `Block2dPresentation`. */
export const BLOCK2D_PRESENTATION_FIELDS: readonly BlockJsonField[] = [
  { key: "shape", kind: "optionalText" },
  { key: "radius", kind: "optionalFloat" },
  { key: "width", kind: "optionalFloat" },
  { key: "height", kind: "optionalFloat" },
  { key: "color", kind: "optionalText" },
  { key: "iconKind", kind: "optionalText" },
];

/** 🔘️ `Block2dHandleKind`. */
export const BLOCK2D_HANDLE_KIND_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "label", kind: "text" },
  { key: "color", kind: "text" },
  { key: "defaultWireKind", kind: "text" },
];

/** 🌱️ `Block2dHandleTemplate`. */
export const BLOCK2D_HANDLE_TEMPLATE_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "handleKind", kind: "text" },
  { key: "angle", kind: "float" },
  { key: "radius", kind: "float" },
];

/** 📸️ `Block2dSnapshot` — the member order the Rust struct declares. */
export const BLOCK2D_SNAPSHOT_FIELDS: readonly BlockJsonField[] = [
  { key: "schema", kind: "text" },
  { key: "nodeKind", kind: "record", fields: BLOCK_KIND_IDENTITY_FIELDS },
  { key: "presentation", kind: "record", fields: BLOCK2D_PRESENTATION_FIELDS },
  { key: "handleKinds", kind: "table", fields: BLOCK2D_HANDLE_KIND_FIELDS },
  { key: "handles", kind: "table", fields: BLOCK2D_HANDLE_TEMPLATE_FIELDS },
  { key: "compatibility", kind: "table", fields: BLOCK_COMPATIBILITY_RULE_FIELDS },
  { key: "attributes", kind: "table", fields: BLOCK_ATTRIBUTE_FIELDS },
  { key: "authors", kind: "table", fields: BLOCK_AUTHOR_FIELDS },
  { key: "camera2d", kind: "record", fields: BLOCK_CAMERA_2D_FIELDS },
  { key: "meta", kind: "record", fields: BLOCK_META_FIELDS },
];'''),
    dict(dir="🖐️5d", dim="5d", p="Block5d", fn="block5d", snap="BLOCK5D_SNAPSHOT",
         schema_value="block.5d",
         kind="s.block.block5d",
         assets=[("hexagonal-cut-concrete-forest-left", "🎬️hexagonal-cut-concrete-forest-left"), ("nakagin-capsule", "🏢️nakagin-capsule")],
         tables='''/** 🔵️ `Block5dPart2d`. */
export const BLOCK5D_PART_2D_FIELDS: readonly BlockJsonField[] = [
  { key: "shape", kind: "optionalText" },
  { key: "radius", kind: "optionalFloat" },
  { key: "width", kind: "optionalFloat" },
  { key: "height", kind: "optionalFloat" },
  { key: "color", kind: "optionalText" },
  { key: "iconKind", kind: "optionalText" },
];

/** 🧱️ `Block5dPart3d`. */
export const BLOCK5D_PART_3D_FIELDS: readonly BlockJsonField[] = [
  { key: "orientation", kind: "optionalFloatTuple" },
  { key: "scale", kind: "optionalFloatTuple" },
];

/** 🔘️ `Block5dGripKind`. */
export const BLOCK5D_GRIP_KIND_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "name", kind: "text" },
  { key: "label", kind: "text" },
  { key: "color", kind: "text" },
  { key: "defaultRopeKind", kind: "text" },
];

/** 🌱️ `Block5dGripTemplate`. */
export const BLOCK5D_GRIP_TEMPLATE_FIELDS: readonly BlockJsonField[] = [
  { key: "id", kind: "text" },
  { key: "gripKind", kind: "text" },
  { key: "angle", kind: "float" },
  { key: "radius2d", kind: "float" },
  { key: "position", kind: "floatTuple" },
  { key: "direction", kind: "floatTuple" },
  { key: "radius3d", kind: "float" },
];

/** 📸️ `Block5dSnapshot` — the member order the Rust struct declares, including the two
 * `#[value(rename = "2d"/"3d")]` members whose json keys are bare dimension names. */
export const BLOCK5D_SNAPSHOT_FIELDS: readonly BlockJsonField[] = [
  { key: "schema", kind: "text" },
  { key: "partKind", kind: "record", fields: BLOCK_KIND_IDENTITY_FIELDS },
  { key: "2d", kind: "record", fields: BLOCK5D_PART_2D_FIELDS },
  { key: "3d", kind: "record", fields: BLOCK5D_PART_3D_FIELDS },
  { key: "representations", kind: "table", fields: BLOCK_REPRESENTATION_FIELDS },
  { key: "gripKinds", kind: "table", fields: BLOCK5D_GRIP_KIND_FIELDS },
  { key: "grips", kind: "table", fields: BLOCK5D_GRIP_TEMPLATE_FIELDS },
  { key: "compatibility", kind: "table", fields: BLOCK_COMPATIBILITY_RULE_FIELDS },
  { key: "attributes", kind: "table", fields: BLOCK_ATTRIBUTE_FIELDS },
  { key: "authors", kind: "table", fields: BLOCK_AUTHOR_FIELDS },
  { key: "camera2d", kind: "record", fields: BLOCK_CAMERA_2D_FIELDS },
  { key: "camera3d", kind: "record", fields: BLOCK_CAMERA_3D_FIELDS },
  { key: "meta", kind: "record", fields: BLOCK_META_FIELDS },
];'''),
]

SNAPSHOT_IMPORT = "../../../../../../../🧬️schema/📸️snapshot/🟦️"
EXPORT_FROM_IMPORT = "../../../../../../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️"


def main() -> None:
    for s in SUBSETS:
        base = BLOCK / s["dir"] / "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
        export_leaf = base / "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts"
        import_leaf = base / "📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts"
        export_leaf.write_text(EXPORT_TS.format(
            dim=s["dim"], p=s["p"], fn=s["fn"], SNAP=s["snap"], kind=s["kind"],
            snapshot_import=SNAPSHOT_IMPORT, shared=SHARED_TABLES,
            per_subset_tables="// #region 🧬️SubsetRecordTables\n" + s["tables"] + "\n// #endregion 🧬️SubsetRecordTables\n",
            writer=WRITER.format()), encoding="utf-8")
        print(f"wrote {export_leaf.relative_to(ROOT)}")
        import_leaf.write_text(IMPORT_TS.format(
            dim=s["dim"], p=s["p"], fn=s["fn"], SNAP=s["snap"], kind=s["kind"],
            schema_value=s["schema_value"], snapshot_import=SNAPSHOT_IMPORT,
            export_import=EXPORT_FROM_IMPORT), encoding="utf-8")
        print(f"wrote {import_leaf.relative_to(ROOT)}")

        tests = base / "🧪️tests"
        (tests / "🧫️fixtures").mkdir(parents=True, exist_ok=True)
        fixtures_ts = "".join(f'  {{ asset: "{a}", example: "{e}" }},\n' for a, e in s["assets"])
        (tests / "🟦️.ts").write_text(TEST_TS.format(dim=s["dim"], p=s["p"], fn=s["fn"], fixtures_ts=fixtures_ts), encoding="utf-8")
        print(f"wrote {(tests / '🟦️.ts').relative_to(ROOT)}")


if __name__ == "__main__":
    main()
