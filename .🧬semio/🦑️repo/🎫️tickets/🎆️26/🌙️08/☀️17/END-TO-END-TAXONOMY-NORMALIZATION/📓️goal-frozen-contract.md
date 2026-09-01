# Frozen Coordinate Evidence Contracts — Exact Shape and Validation

Sources read (line numbers as of this ticket slice):
- Types + validators: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:634-712`
- Loader/enforcement: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:5000-5090` (violation call sites) and `:6671-6784` (`frozenCoordinateEvidenceCoordinates`, `markdownSourceCoordinateSpans`, `frozenMarkdownCoordinateEvidenceCoordinates`).

## 1. `frozenMarkdownCoordinateEvidenceContracts` (family for our slice — ticket/`.cursor` reports are Markdown)

Type (`FrozenMarkdownCoordinateEvidenceContract`):
```
{
  path: string,               // must end ".md", repo-relative, no opaque compose/temp-compose prefix, no "..", must be unique across all entries in this family
  grammar: "frozen-markdown-source-coordinates-v1",  // literal, only legal value
  sha256: string,              // /^[a-f0-9]{64}$/ — sha256 of the WHOLE document's exact UTF-8 bytes
  coordinates: [
    { start: number, end: number, kind: "source", form: "inline-code" | "path-list-item", valueSha256: string }
    // one or more, no overlaps
  ]
}
```
Object key discipline (`Object.keys(row).sort().join("\0")`): must be EXACTLY `coordinates\0grammar\0path\0sha256` — no extra/missing keys.
Each coordinate's keys must be EXACTLY `end\0form\0kind\0start\0valueSha256`.

Validation rules (`validateFrozenMarkdownCoordinateEvidenceContracts`):
- Contract id: `/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/` (lowercase kebab).
- `path`: ends with `.md`; no `\ : * ? " < > | ` + control chars; no empty/`.`/`..` path segments; not under `compose/` or `temp/compose/`; unique across the whole map.
- `sha256`: 64 lowercase hex chars — digest of the full document bytes.
- `coordinates`: nonempty array.
  - `kind` must be literal `"source"`.
  - `form` must be `"inline-code"` or `"path-list-item"`.
  - `start`/`end`: safe integers, `start >= 0`, `end > start` (nonempty span, UTF-16 code-unit offsets into the file content as a JS string).
  - `valueSha256`: 64 lowercase hex chars.
  - Spans across the whole contract must not overlap or duplicate (sorted by start, then checked `start < previous.end`).

Runtime resolution (`frozenMarkdownCoordinateEvidenceCoordinates`, called only when a `path` in the map matches the file under inspection):
1. Re-validates the whole contracts map (fails loud on any of the above).
2. `sha256(bytes) !== contract.sha256` -> fail "document digest does not match registered bytes". So `sha256` MUST be computed over the exact current file bytes.
3. Decodes bytes as UTF-8 and re-encodes; must round-trip exactly (rejects non-UTF-8 / lone surrogates).
4. Computes `markdownSourceCoordinateSpans(content)` — the ONLY spans the engine will ever recognize as "safe frozen" Markdown source coordinates. This is a hand-rolled scanner (not a full CommonMark parser) that finds, per line, outside fenced code blocks (``` or ~~~), outside `<!-- -->` HTML comments and raw HTML blocks, outside indented code blocks and blockquote/heading lines:
   - `path-list-item`: a line that is ONLY a `- `/`* `/`+ `/`1. ` list marker followed by one single "word" (no whitespace) and nothing else — the span covers just that word.
   - `inline-code`: text between a single-backtick pair on one line (multi-backtick fences on the same line for wider spans are NOT registered — only length-1 backtick runs), as long as the line doesn't also contain `[`, `]`, `<`, `>` outside the backtick-masked regions (guards against link/HTML syntax hiding a path).
5. For every declared `coordinate`, the exact `(form, start, end)` triple MUST be present in that computed span set, else fail "coordinate has no exact admitted Markdown source span" — i.e. you cannot pick arbitrary offsets, only offsets the scanner independently re-derives from the current file text.
6. The extracted `value = content.slice(start, end)` must be non-empty, contain no `\ : * ? " < > | ` backtick or control chars (0x00-0x20), must not start with `compose`/`temp/compose`, and no path segment may be empty/`.`/`..`.
7. `sha256(value) !== coordinate.valueSha256` -> fail "coordinate value digest differs from its exact source authority". So `valueSha256` = sha256 of the exact substring (the path token itself), NOT the whole file.
8. On success, returns coordinates as `{ pointer: "markdown:<form>@<start>", start, end, value, kind: "source" }`.

These become admitted "frozen" tokens: in `🧹️normalization/🟦️.ts` (`isFrozenSourceCoordinateToken`, `frozenEvidenceCoordinateAuthority`), any reference-scanner token whose `(start,end,value)` exactly matches one of these frozen coordinates is treated as already-authorized "leave alone" evidence and is exempted from `reference-syntax-unsupported` / `frozen-coordinate-evidence-unowned` complaints — it is never rewritten, and it is why registering the entry, rather than the taxonomy engine trying to auto-detect historical paths, is what silences the violation.

## 2. `frozenCoordinateEvidenceContracts` (JSON family — NOT used for our Markdown slice, documented for completeness/contrast)

Type (`FrozenCoordinateEvidenceContract`): `path` (`.json`, same path hygiene as above), `sha256` (whole-document digest), `schemaVersion: number|null`, optional `rootKind: "array"`, `coordinates: [{ pointer, kind } | ...with representation variants]`. Object keys must be exactly `coordinates\0path\0schemaVersion\0sha256` (or with `rootKind` inserted alphabetically for array roots). Coordinate `pointer` must match `/^(?:\/(?:\*|(?:[^/~*]|~[01])+))+$/` (RFC6901 JSON pointer segments, `*` allowed as an array wildcard), unique per contract, `kind` is `"source"|"destination"`. No `valueSha256` in this family — the JSON string's exact declared span from `jsonStringCoordinates` stands in for identity, and the whole-document `sha256` plus the pointer's resolved match against the live parse is the freezing mechanism (see optional `representation` variants: `recorded-repository-absolute`, `recorded-package-owner-identity`, `json-escaped-source-path`, each changing how the raw string is decoded relative to the required prefix — irrelevant to our Markdown rows).

## 3. What our generator must produce

For every targeted `.md` file (ticket reports under `.🧬semio/…/🎫️tickets/` or `.cursor/plans/`) that has one or more `reference-syntax-unsupported` unresolved rows whose token is a path-bearing Markdown source span:
1. Read the file's exact current bytes -> `sha256` for the contract's `sha256` field.
2. Independently recompute `markdownSourceCoordinateSpans` (same algorithm) over the file content to find the admissible `(start,end,form)` for each violating token's path text, OR reuse the plan's own reported coordinate if it already carries a byte offset — safest is to re-derive spans in the generator using the identical scanning rules so the offsets are guaranteed admissible, rather than trusting the plan message's embedded string.
3. For each admissible span whose slice equals the offending token value: emit `{ start, end, kind: "source", form, valueSha256: sha256(value) }`.
4. Assemble one contract entry per file: `{ path, grammar: "frozen-markdown-source-coordinates-v1", sha256, coordinates: [...sorted by start, non-overlapping] }`, with a descriptive kebab-case id.
5. Insert entries into `taxonomy.json`'s `frozenMarkdownCoordinateEvidenceContracts` map, preserving 2-space indent and existing key/entry ordering conventions (append after the last existing entry, keys in the order `path, grammar, sha256, coordinates`, coordinate keys in the order `start, end, kind, form, valueSha256`).
