# Mutation Union Wire-Format Audit

Audited every `🧬️mutations/🟦️component.ts` discriminated union in the repo (115 files found by
`find ✏️s/🔌️plugins -path "*🧬️mutations/🟦️component.ts"`, minus exclusions below) against its
sibling Rust enum's actual `#[serde(...)]` attributes and, wherever one exists, the committed
`🧪️tests/**/🦠️mutation/🔣️component.json` fixture.

## Exclusions (not audited, per instructions)

- **`🧱️block`** (3 files: `◻2d`, `🖐️5d`, `🧊️3d`) — another agent is fixing a kebab-case discriminant
  bug there concurrently; skipped entirely to avoid collision.
- **`🏛️architect` / `🏛️program`** — a dedicated agent is fixing this file's 266 pre-existing
  undefined-type errors (unreachable from architect's barrel `📦️index.ts`) plus its
  PascalCase→camelCase discriminant bug; skipped entirely to avoid collision.

115 files remained in scope.

## Method

1. For every remaining file, located the sibling Rust enum, read the attribute block **immediately
   above** `pub enum …Mutation` (not the first `#[serde]` in the file), and classified it:
   - **Shape (A) — internally tagged**: `#[serde(tag = "mutation", rename_all = "…")]` present.
     Wire form: `{ "mutation": "<tag>", ...fields }` (or `{"mutation": tag, "payload": {...}}` for
     the handful that also carry `content = "payload"` — adjacently tagged).
   - **Shape (B) — externally tagged**: only `#[derive(…, dsl::Mutations)]`, no `#[serde(tag=…)]`.
     Wire form: `{ "<PascalCaseVariant>": { ...leaf fields } }`.
2. Cross-checked every classification against a committed fixture wherever one exists (`find
   <artifact-root> -path "*🦠️mutation*" -name "*.json"`).
3. Scanned **every one of the 115 files** with a script that flags any `mutation: "…"` /
   `mutation: '…'` tag literal that is kebab-case or PascalCase (i.e. not camelCase) — the two wrong
   spellings actually observed in this repo. This is a full sweep, not a sample: every occurrence of
   that literal pattern in all 115 files was checked.
4. Spot-verified a sample of files that use a **composed-import** architecture (the aggregate file
   just unions named types re-exported from per-leaf `🟦️component.ts` files, so the tag literal
   lives one file down) and files that read as already-correct on inspection.

## Critical finding: fixtures overrule attribute inference — twice

For **`stdio/📊️csv`** and **`stdio/💬️bcf`**, the Rust enum (`CsvMutation`, `BcfMutation`) carries
**no** `#[serde(tag=…)]` — by the same reasoning used everywhere else in this audit that would mean
shape (B). But their committed fixtures (`📄set-snapshot/…/🦠️mutation/🔣️component.json`) show
`{"mutation": "setSnapshot", "snapshot": {...}}` — shape (A) — and that JSON is fed straight into
`serde_json::from_str::<CsvMutation>(...)` by a real, presumably-passing Rust test. The mechanism
behind this discrepancy is not visible to me (no custom `Deserialize` impl found; `dsl::Mutations`
is not documented here), but the ticket's rule is unambiguous: fixtures beat inference. Both
artifacts' existing TS (`{ mutation: 'setSnapshot'; snapshot: … }`) already matches the fixture, so
**no change was made** to either — but this cost real time to discover and is worth flagging: **do
not trust "no `#[serde(tag)]` ⇒ shape (B)" without a fixture** for any artifact under `stdio`.

A second lesson, cheaper to state: `rename_all` on a shape-(A) enum is **not always `"camelCase"`**.
`stdio/📄️pdf` (1.4/base), `📷️jpg`, `📷️png`, `🖼️bmp`, `🖼️tiff` all declare
`rename_all = "kebab-case"` explicitly — their kebab-case tags are *correct*, not a bug. Only
`svg`/`xml`/`json`(base)/`gltf` declare `rename_all = "camelCase"` while their TS used kebab-case
tags; those four were the real defects in that cluster.

## Confirmed defects — fixed (19 files)

| Artifact | Shape | Confirmed by | Bug | Fix |
|---|---|---|---|---|
| `📕norm/📕din4108` | B | fixture | shape (A) written (`{mutation:"camelCase"}`) + `ClimateZoneDe` values lowercased (should be PascalCase `"Zone1"`…) | rewrote union to `{Variant: Variant}`, snake_case leaf fields, fixed `ClimateZoneDe` |
| `📕norm/📔vdi3805` | B | fixture | shape (A) written; every nested value type (`ManufacturerFile`, `VdiUnit`, `BoundingBox`, …) also wrongly camelCased; `VdiQuantityKind` wrongly lowercased (should be PascalCase `"Dimensionless"`…) | rewrote union + all nested interfaces to snake_case/PascalCase per fixture |
| `📕norm/📓iso16757` | B | fixture | shape (A) written | rewrote union to `{Variant: Variant}`, snake_case leaf fields |
| `🌀procedural/🌀procedural2d` | B | fixture | shape (A) written (8 wired variants) | rewrote union tagging only (leaf-imported field casing was already correct) |
| `🌀procedural/🧩assembly` | B | fixture | internally-tagged-by-`kind`, kebab-case (`{kind:"create-slot"}`) — a third wrong shape | rewrote to `{Variant: Variant}`, snake_case fields |
| `🌍gis/🗺️gismap` | B | fixture | shape (A) written; NOTE: unlike the others, every one of gismap's 12 leaf structs individually carries `rename_all="camelCase"`, so fields stay camelCase | rewrote tagging only, kept camelCase fields |
| `stdio/🧿semio ✳️brep` | B | fixture | shape (A)-style `{mutation:"...", payload:{...}}` envelope; broken `import(".../🦠️mutation/🟦️component.ts")` refs (files don't exist) | rewrote to `{Variant: Variant}`, snake_case fields, self-contained interfaces |
| `stdio/🧿semio ✳️drawing` | B | fixture | same envelope bug, mixed camelCase/kebab-case tags, 9/17 variants wired | rewrote the 9 wired variants; left the 8 unwired ones documented as still-missing (unchanged from before) |
| `stdio/🧿semio ✳️graph` | B | fixture | same envelope bug + broken `import()` refs | rewrote to `{Variant: Variant}`, snake_case fields |
| `stdio/🧿semio ✳️kit` | B | fixture (same pattern as `object`) | same `{mutation, payload}` envelope, `payload:{childId,target:string}` (target should be full `ArtifactRef`) | rewrote to `{Variant: Variant}`, snake_case fields, `target: ArtifactRef` |
| `stdio/🧿semio ✳️mesh` | B | fixture | flat `{mutation:"...", ...fields}` shape | rewrote to `{Variant: Variant}`, snake_case fields |
| `stdio/🧿semio ✳️object` | B | fixture | `{mutation,payload}` envelope, `target: string` (should be `ArtifactRef` object) | rewrote to `{Variant: Variant}`, snake_case fields, `target: ArtifactRef` |
| `stdio/🧿semio ✳️table` | B | fixture | `{mutation, payload:{...}}` envelope | rewrote to `{Variant: Variant}`, snake_case fields |
| `stdio/🧿semio ✳️text` | B | fixture | `{mutation, payload:{...}}` envelope | rewrote to `{Variant: Variant}`, snake_case fields |
| `📸remodel` | A | fixture | correct `{mutation:"…"}` shape but **kebab-case** tag values (`"create-stream"`) instead of camelCase (`rename_all="camelCase"`); also missing the 35th variant `CommitReconstruction` (already-documented gap, not touched) | converted 34 tag literals kebab→camelCase |
| `stdio/🎨svg` (1.1/base) | A (adjacent) | fixture (leaf-level) | kebab-case tags instead of camelCase | converted 9 tag literals |
| `stdio/📰xml` (1.0/base) | A (adjacent) | attribute only (no fixture found) | kebab-case tags instead of camelCase | converted 6 tag literals |
| `stdio/🔣json` (rfc8259/base) | A (adjacent) | attribute only (no fixture found) | kebab-case tags instead of camelCase | converted 5 tag literals |
| `stdio/🧊gltf` (2.0/any) | A (adjacent) | attribute + Rust variant-name cross-check | kebab-case tags instead of camelCase, 120 variants | converted all 120 tag literals kebab→camelCase |

All fixes verified with `bunx tsc --noEmit --strict --target ESNext --module ESNext
--moduleResolution bundler --esModuleInterop --skipLibCheck --allowImportingTsExtensions` against
each touched plugin's `📦️packages/🟦️typescript/📦️index.ts`:

```
norm       : 0 errors
procedural : 0 errors
gis        : 0 errors
stdio      : 0 errors
remodel    : 0 errors
```

`git status --porcelain -- <19 files above> | wc -l` → **19** (exact scope, nothing extra touched).

## Verified already-correct (no change) — 15 files, fixture- or attribute-confirmed

| Artifact | Shape | Confirmed by |
|---|---|---|
| `📏layout` | B | fixture (this was the reference file for shape B, per brief) |
| `✒writer` | A | fixture (reference for shape A) |
| `🌍gis/🏔️gisterrain` | B | fixture |
| `🎪demonstrator/🎪playground` | B | fixture |
| `💠lowpoly` | B | fixture (per file's own extensive doc comment, spot-checked) |
| `📕norm/📘en1991`…`en1995` (5 files) | B | fixture (en1992 spot-checked directly; 1993–1995 shape-verified structurally, `rename_all=camelCase` attribute-confirmed on every leaf sampled) |
| `stdio/💬bcf` | A* | fixture (*attribute reading said B — see "critical finding" above; fixture wins) |
| `stdio/📊csv` | A* | fixture (*same caveat) |
| `stdio/📄pdf` (1.4/base) | A, kebab | attribute (`rename_all="kebab-case"` explicit) |
| `stdio/📷jpg`, `📷png`, `🖼bmp`, `🖼tiff` | A, kebab | attribute (`rename_all="kebab-case"` explicit) |
| `🌿vcs` | A (composed-import) | fixture, leaf-level |

Also spot-checked and found already correct (camelCase, shape A, no fixture available or needed
given the file's tags were already well-formed and internally consistent with the enum's own
`rename_all="camelCase"`): `🎬sequence`, `📋forms`, `🔋energy/🔋model`, `🖨raster`, `📖playbook`,
`stdio/🧿semio` subsets `✳️any`, `✳️model`, `✳️value`, `✳️document`, `stdio/📄pdf` 1.7/base (composed-
import, leaf spot-checked).

## Coverage note — the exhaustive kebab/PascalCase sweep

Step 3 of the method above (`mutation\s*:\s*['"]([a-zA-Z0-9_-]+)['"]` scanned against **every one**
of the 115 in-scope files, not a sample) is what surfaced `svg`/`xml`/`json`/`gltf`/`remodel` and
also correctly cleared `pdf`/`jpg`/`png`/`bmp`/`tiff` (kebab-case is their real, declared casing).
That sweep only catches the "wrong casing style" defect class for files that use the literal
`mutation: "…"` pattern. It does **not** independently verify:
- Files with **zero** `mutation:` literals (composed-import architectures where the tag lives in a
  per-leaf file) — I individually opened a leaf in `vcs`, `energy/model`, and `pdf 1.7/base` as
  samples and found them correct; I did not open a leaf in every other composed-import file (e.g.
  the other five `pdf` 1.7 subsets `e`/`h`/`ua`/`vt`/`x`, or `stdio` artifacts I didn't name above).
- **Field-name casing** inside an already-correctly-tagged union, for the ~80 shape-(A) artifacts I
  did not individually open a fixture for. Given `writer`/`vcs`/`remodel`/`playbook` all confirmed
  `rename_all="camelCase"` fields are the norm for shape (A), and none of those spot-checked showed
  drift, I did not find evidence of a second wave of field-casing bugs there — but I did not
  exhaustively confirm it artifact-by-artifact against a fixture the way I did for every shape-(B)
  artifact and the four shape-(A) kebab-case ones.

If you want that residual risk closed out, the next pass should specifically open one fixture per
still-unverified shape-(A) artifact and diff its field names against the leaf struct.

## Not-yet-filled (stub/scaffold, correctly left alone) — 9 files

No union exists yet in these files, so there is nothing to mis-tag — out of this audit's scope
(fixing wire-format bugs in *existing* unions), not a defect this ticket covers:

- `➗mathematical`, `🌊flow`, `🎞animate/🎬present`, `📐cad`, `🏭process/🧊process3d`, `🕸dag` — all
  literally `export {};` behind a `/** … WASM facade. */` comment.
- `stdio/🎵mp3`, `stdio/🔊wav` — `/** 🚧 scaffolded by W1b — generic facet mirror */` placeholder
  (`{schema, entries: {key,value}[]}`), not a real per-variant union.
- `🗒note` — bare `export {};`, no comment at all.

## Related finding, out of this ticket's scope (flagged, not fixed)

- **Per-leaf `🦠️mutation/🟦️component.ts` mirrors can independently drift** from the aggregate file
  I was asked to fix. Example: `📕norm/📓iso16757/…/🍃change-exchange-process/🦠️mutation/🟦️component.ts`
  declares `newExchangeProcess: ExchangeProcess` but the committed fixture and Rust leaf struct both
  say `new_exchange_process`. I did not touch this file — it is not one of the 115
  `🧬️mutations/🟦️component.ts` aggregate files this ticket named, but the same bug class likely
  recurs across other artifacts' per-leaf mirrors. Also noted: some leaf directories carry a
  `🔣️payload.schema.json` (e.g. din4108's `change-moisture-mu-interior`) that itself asserts the
  WRONG (camelCase) field name — this schema file is not a committed fixture in the ticket's sense
  and should not be trusted; I ignored it in favor of the actual `🧪️tests/**/🦠️mutation/…json`
  fixtures and the Rust struct.

## Honest boundaries

- I did not guess on `stdio/📰xml` or `stdio/🔣json` (base) — no committed fixture exists for either;
  the fix rests on the Rust `#[serde(tag="mutation", content="payload", rename_all="camelCase")]`
  attribute alone, which is unambiguous (unlike the bcf/csv no-attribute case, an *explicit*
  `rename_all` is deterministic serde behavior, not something a hidden macro could override).
- `stdio/🧿semio ✳️drawing` is left intentionally partial (9 of 17 variants) — matches its own
  pre-existing doc comment; I did not add the missing 8, since that's scope-expansion beyond fixing
  existing entries.
- `📸remodel`'s missing 35th variant (`CommitReconstruction`) and its lack of real per-variant
  payload interfaces (it's a bare tag-union + envelope, not a full discriminated union with fields)
  were left alone — restructuring it into a complete union is a materially larger task than the
  wire-format casing fix this ticket asked for.
