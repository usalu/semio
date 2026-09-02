# Shard B6 — Relocation Executed

95 of the 176 census-enumerated artifact-level cases were relocated from `<artifact>/🧪️tests/<case>`
to `<standard>/🪆️subsets/<subset>/🧪️tests/<case>`. Full per-case disposition (including the 81 left
in place, with reasons) is in `📓️b6-artifact-level-test-census.md`. Raw plan:
`🗑️generated/b6-relocate-log.txt` (95 `RELOCATE` lines). Extraction/relocation scripts:
`🔍️b6-extract-census.py`, `🔨️b6-relocate-cases.py`.

## What moved, and why these 95

Every relocated case passed two automated safety checks before the move (never asserted by hand):

1. **Target-subset ownership** — every `asset://🏅️standards/<ver>/🪆️subsets/<subset>/…` URI in the
   case already named the *same* `<ver>/<subset>` the case is moving to (0 mismatches across all 95).
2. **No sibling-case fixture collision** — every `shared://` fixture the case references was checked
   against every *other* case still living in the same artifact; none was found reused by a case
   targeting a different subset (0 conflicts), so moving the physical fixture file alongside its one
   case cannot break anyone else.

Where a case referenced a `shared://` fixture, the physical file was moved from
`<artifact>/🧫️fixtures/<name>` to `<subset>/🧫️fixtures/<name>` in the same operation (25 fixture
files moved — list below). `local://` fixtures needed no separate handling: they live inside the
case directory itself and move with it. Rust adapter `use` paths needed **no edits** — a background
investigation of `materializeRustHost` (`🧰️framework/…/🧪️test/📜️script.ts:419`) confirmed the
per-case Rust host crate is generated fresh from a live filesystem walk on every run (`#[path]` is
built from `discovered.adapters.rust` at generation time, never committed), so relocating the
directory is sufient by itself. No `#[path]`/`include_str!` needed fixing in any of the 95 cases.

### Breakdown by bucket

| Bucket | Count | Description |
|---|---|---|
| `pdf` | 4 | `create-minimal-pdf`, `edit-existing-pdf` → `1.7/✳️base`; `mutate-pdf-1-4`, `extract-text-pdf-1-4` → `1.4/✳️base` |
| `single-subset-artifact` | 74 | Every non-B3, non-pdf artifact whose `🏅️standards/*/🪆️subsets/` holds exactly **one** subset — relocation is unambiguous by cardinality alone |
| `stdio-multi-safe` | 17 | Individually verified cases inside a genuinely multi-subset stdio artifact (see below) |

`stdio-multi-safe` cases: `mutate-gif-87a` (87a/✳️any — 87a has only one subset), `mutate-svg-1-1-basic`
(1.1/✳️basic), `mutate-ifc-2x3-sav` (2x3/✳️sav), `mutate-xml-1-0` (1.0/✳️base), `mutate-xml-1-0-valid`
(1.0/✳️valid), `create-and-round-trip-tiff` (6.0/✳️document), `mutate-gltf-2-0` (2.0/✳️any),
`mutate-dwg-ac1018` (ac1018/✳️any), and nine `🧿️semio` cases — `mutate-semio-any`, `-drawing`,
`-graph`, `-image`, `-kit`, `-mesh`, `-object`, `-table`, `-text` (each `v1/✳️<name>`).

## The PDF worked example, in full

The developer's own example (`📄️pdf`) has **13** artifact-level cases. All target exactly one
subset (`SINGLE-SUBSET` — never `MULTI-SUBSET`/`ARTIFACT-WIDE`), but only **4 were safe to move**:

| Case | Target | Relocated? | Why |
|---|---|---|---|
| `create-minimal-pdf` | 1.7/✳️base | ✅ | No fixture at all — pure synthetic construction |
| `edit-existing-pdf` | 1.7/✳️base | ✅ | Its only fixture is `local://📄️two-pages.pdf`, inside the case dir |
| `mutate-pdf-1-4` | 1.4/✳️base | ✅ | Its only `asset://` fixture is already owned by 1.4/✳️base |
| `extract-text-pdf-1-4` | 1.4/✳️base | ✅ | Same — plus fixed a latent bug (below) |
| `mutate-pdf-1-4-a` | 1.4/✳️a | ❌ | Only fixture (`🎓️bachelor-thesis.pdf`) is owned by **1.4/✳️base**, a sibling subset |
| `mutate-pdf-1-4-x` | 1.4/✳️x | ❌ | Same blocker |
| `mutate-pdf-1-7`, `-a`, `-e`, `-h`, `-ua`, `-vt`, `-x` (7 cases) | 1.7/✳️base, ✳️a, ✳️e, ✳️h, ✳️ua, ✳️vt, ✳️x | ❌ | Same blocker — worse, these are in the **1.7** standard borrowing a fixture physically committed under **1.4** |

**Why the 9 are not a partial job, but a hard architectural stop.** `resolveFixtures`
(`🟦️.ts:962`) resolves `asset://` strictly against `discovered.owner` with a path-escape guard
(`resolve(abs).startsWith(guard + sep)`) — a case can **never** reach a file outside its own owner's
directory subtree via `asset://`, by design (there is no `..` allowed). The one real 6.3 MB bachelor
thesis PDF that exercises every PDF/A and PDF/UA conformance-class mutation catalog is committed once,
under `1.4/🪆️subsets/✳️base/📚️examples/`. Moving `mutate-pdf-1-7-a` to owner `1.7/✳️a` would make
that file *unreachable* — the fixture reference would resolve to nothing, i.e. exactly the
`missing-fixture` regression the brief says must never happen. The framework's own docstring on
`resolveFixtures` explains why duplicating this real-world document per subset was never done:
"copying a multi-megabyte document into a fixtures directory would duplicate history for no gain."
Untangling this — either duplicating the fixture per subset/version, or deciding these 9 cases are
legitimately artifact-owned — is exactly the `📄️pdf` slice of Wave 2's "give subsets their own
`🚪️io`/`🧬️schema`" item, which is out of B6's mandate. Left in place, fully documented in the census.

**Bug fixed in passing, in the file being moved.** `extract-text-pdf-1-4`'s Python oracle
(`🐍️.py`) hard-coded `DOCUMENT = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/…"` — `✳️any` does not
exist under `1.4` (only `✳️a`/`✳️base`/`✳️x` do); the feature file itself correctly said `✳️base`.
Since `Context.fixture_bytes` looks the URI up by exact string match against the plan's
feature-declared fixture list (`🧪️test/📦️packages/🐍️python/🐍️.py:106-116`), this mismatch meant
the case raised `KeyError` on every real run — a pre-existing, silent breakage this static
`test contract` gate cannot see (it only checks the feature's own declared URIs, not a stray
constant buried in adapter code). Corrected to `asset://📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf`
(the post-relocation, owner-relative form) while the file was already open for the move.

## URI rewrites — the general pattern

For every relocated case whose owner *is* the subset that already owned the referenced asset, the
`🏅️standards/<ver>/🪆️subsets/<subset>/` prefix was mechanically stripped from every `asset://` URI
in the moved `🥒️.feature`, `🦀️.rs`, `🐍️.py` and `🟦️.ts` files (script: `relocate()` in
`🔨️b6-relocate-cases.py`). Example (`mutate-pdf-1-4`):

```
- asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
+ asset://📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
```

`shared://` and `local://` URIs never needed text edits (the scheme name inside the feature stays
identical); only the physical fixture file's location changed for `shared://`.

### Shared fixture files moved (25)

```
☁️ply     🧪️pattern-sphere/🧊️.ply                              → 1.0/✳️any
🌐️html    🧪️zukunft-bau-entwerfen-mit-bestand/🌐️.html            → 5/✳️any
🎥️mp4     🎬️.mp4                                                → isobmff/✳️any
🎵️mp3     🔊️.mp3                                                → mpeg1-layer3/✳️any
💾️binary  🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg           → raw/✳️any
📄txt     📄️interview-transkript.tex, 🔤️.txt                    → utf-8/✳️any
📊️csv     🧪️reuse-marketplaces/📊️.csv                           → rfc4180/✳️any
📑️tsv     reuse-marketplaces.tsv                                → iana/✳️any
📝️md      📄️readme.md                                           → commonmark/✳️any
📷️png     🧪️rathaus-ahlen-grundriss/🖼️.png                      → 1.2/✳️any
🔊️wav     🧪️bauen-mit-bestand-ausschnitt/🔊️.wav                 → riff-pcm/✳️any
🖼️bmp     🧪️rathaus-ahlen-grundriss/🖼️.bmp                      → v3/✳️any
🗜️deflate 📄️readme-level1.zz, 📄️readme-level9.zz                → rfc1950/✳️any
🟪️stl     🧪️hexagonal-cut-concrete-forest-left/🧊️.stl           → ascii/✳️any
🎞️gif     🧪️dancing-87a-large/🖼️.gif, 🧪️dancing-87a/🖼️.gif       → 87a/✳️any
🎨️svg     mouse.svg, 🎨️semio-brand-and-onboarding.svg            → 1.1/✳️basic
🏗️ifc     🧪️wellness-center-sama-structural-seed/🏗️.ifc          → 2x3/✳️sav
📰xml     🏷️.xml, 🧪️ooxml-readme-document/🏷️.xml                 → 1.0/✳️base
📰xml     🧪️macos-uttype-plist/🏷️.xml, 🧪️reuse-marketplaces-plist/🏷️.xml → 1.0/✳️valid
```

Full source→destination list: `🗑️generated/b6-relocate-log.txt`.

## Discover evidence

`bun ./📜️script.ts test discover` after the moves: **191 test case(s)** total (same total as before —
nothing lost or duplicated, only relocated). Every relocated case's Nx project id now carries
`subsets-<name>` in place of the bare artifact segment, confirming the SUBSET is the discovered
owner:

```
test-s-plugins-stdio-artifacts-pdf-standards-14-subsets-base-4bbcbc-extract-text-pdf-1-4   …/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🧪️tests/extract-text-pdf-1-4
test-s-plugins-stdio-artifacts-pdf-standards-17-subsets-base-869a2f-create-minimal-pdf      …/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧪️tests/create-minimal-pdf
test-s-plugins-stdio-artifacts-pdf-standards-17-subsets-base-869a2f-edit-existing-pdf       …/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🧪️tests/edit-existing-pdf
test-s-plugins-stdio-artifacts-semio-standards-v1-subsets-object-fc3bba-mutate-semio-object …/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧪️tests/mutate-semio-object
test-s-plugins-stdio-artifacts-svg-standards-11-subsets-basic-72e8ab-mutate-svg-1-1-basic   …/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🧪️tests/mutate-svg-1-1-basic
test-s-plugins-stdio-artifacts-gltf-standards-20-subsets-any-f486e7-mutate-gltf-2-0         …/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️tests/mutate-gltf-2-0
test-s-plugins-writer-artifacts-writer-standards-1-subsets-any-99af0c-mutate-writer-1       …/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-writer-1
```
(full 191-line dump: `🗑️generated/b6-discover-after.txt`; all 95 relocated cases individually
grepped and confirmed carrying their target subset in both project id and path.)

## Gate: `bun ./📜️script.ts test contract`

Ran in the foreground before and after, per the brief; it exits non-zero either way (expected —
that is not the signal). Counted from `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`:

| Breach id | Before | After | Δ |
|---|---|---|---|
| `missing-fixture` | 0 | 0 | **0 — did not rise** |
| `case-in-language-package` | 0 | 0 | 0 |
| `case-slug` | 0 | 0 | 0 |
| `unknown-mutation-catalog` | 5 | 4 | -1 (unrelated to B6 — all 4 remaining hits are in B3's territory: `🖍️draw`, `🏗️fem/◻2d`, `🏗️fem/🧊️3d`, `🗒️note`; none of my paths appear before or after) |
| `mutation-catalog-unclaimed` | 30 | 30 | 0 |
| `mutation-catalog-capability-mismatch` | 0 | 0 | 0 |
| `no-adapter` | 0 | 0 | 0 |
| `unknown-case-child` | 0 | 0 | 0 |
| `unknown-adapter-filename` | 0 | 0 | 0 |
| `unsplit-artifact-subset` | 942 | 0 | **-942** |
| **Total breach count** | **2977** | **2034** | **-943** |

The 943-breach drop is essentially the entire `unsplit-artifact-subset` bucket clearing (942), plus
one unrelated `unknown-mutation-catalog` hit that disappeared from a concurrent edit elsewhere in the
tree (not touched by B6 — none of the 4 remaining hits, nor the one that vanished, are under any path
this shard wrote to). No breach class B6 could plausibly cause (`missing-fixture`,
`case-in-language-package`, `case-slug`, `no-adapter`, `unknown-case-child`,
`unknown-adapter-filename`) moved at all.

## What's left, and why (see the census for the full per-case table)

176 census-enumerated cases − 95 relocated − 6 B3 territory = **75 left in place**, every one
individually attributed to exactly one of three reasons (no "unaccounted" remainder):

| Reason | Count |
|---|---|
| **Blocked** — only fixture is committed under a sibling subset or sibling standard version | 65 |
| **Anomaly** — the tag names a subset (usually a leftover `✳️any`/`✳️base` catch-all) that no longer exists for this case's own version | 7 |
| **Artifact-wide** — genuinely a container/grammar round trip, no subset-scoped mutation | 3 |

- **6 cases** in `🎬️sequence`, `➗️mathematical`, `🏗️fem/◻2d`, `🏗️fem/🧊️3d`, `🖍️draw`, `🗒️note` —
  shard B3's territory, not touched (one, `mutate-sequence-1`, had already been moved/removed by B3
  mid-census; the stale row is called out in the census rather than silently dropped).
- **65 blocked cases**: `📄️pdf` contributes 9 (detailed above — including `mutate-pdf-1-7`, whose
  fixture is committed under the *sibling version* `1.4/✳️base`, not `1.7/✳️base` itself). The same
  `asset://`/`shared://`-escape-guard problem recurs across `☁️las` (`-points`, `-vlr`; 2 of 3 — the
  base case is its own anomaly, see below), `🎒️zip` (2), `🎞️gif` (`-89a-application`, `-89a-comment`,
  `-89a-graphic-control`; 3 of 4 — only `-87a` was safe, and plain `-89a` is its own anomaly),
  `🎞️pptx` (3), `🎨️svg` (2 of 3 — only `-basic` was safe), `🏗️ifc` (6 of 7 — only `-sav` was safe),
  `💬️bcf` (`-snapshot`/`-viewpoint`, 2 of 3 — the base case is its own anomaly), `📐️step` (7),
  `📕️xlsx` (3), `📜️docx` (3), `📷️jpg` (2), `📼️avi` (`-idx1`/`-movi`, 2 of 3 — the base case is its
  own anomaly), `🔣️json` (2), `🖊️dwg` (`mutate-dwg-ac1024`, sibling-version fixture), `🖊️dxf`
  (`-blocks`/`-entities`/`-tables`, 3 of 4 — plain `-r12` is its own anomaly), `🖼️tiff` (2 of 3),
  `🧊️obj` (`mutate-obj-3-0-material`, 1 of 3 — the other two are their own anomaly), `🧿️semio`
  (10 of 19 — `animation`, `audio`, `brep`, `cad`, `document`, `flow`, `model`, `presentation`,
  `value`, `video`; each references `✳️any`'s shared example pack instead of its own). In every case:
  one committed real-world example or handcrafted vector is exercised by every subset's mutation
  catalog in that artifact, so only the ONE sibling case whose target *is* the fixture's owning
  subset/version can ever be relocated without either duplicating the fixture or accepting
  `missing-fixture`.
- **7 anomalies** (`☁️las`/`mutate-las-1-0`, `💬️bcf`/`mutate-bcf-2-1`, `📼️avi`/`mutate-avi-1-0`,
  `🧊️obj`/`create-and-round-trip-obj` + `mutate-obj-3-0`, `🖊️dxf`/`mutate-dxf-r12`,
  `🎞️gif`/`mutate-gif-89a`) — the Rust adapter imports `standards::v<X>::subsets::any`, or the
  feature tags `@mutations-<x>-any`, but no `✳️any` subset directory exists any more under that
  artifact/version (a catch-all left behind when its siblings — `✳️header`/`✳️points`/`✳️vlr`,
  `✳️markup`/`✳️snapshot`/`✳️viewpoint`, `✳️hdrl`/`✳️idx1`/`✳️movi`, `✳️geometry`/`✳️material`,
  `✳️blocks`/`✳️entities`/`✳️header`/`✳️tables`, `✳️application`/`✳️base`/`✳️comment`/`✳️graphic-control`
  respectively — were split out from under it). These need a decision (restore an `✳️any` subset, or
  redistribute the catalog across the real subsets) before they can be relocated or even compile
  cleanly; flagged, not guessed at.
- **3 `ARTIFACT-WIDE` cases** (`🎒️zip`/`create-and-edit-archive`, `🎞️gif`/`create-and-round-trip-gif`,
  `📷️jpg`/`create-and-read-jpeg`) — no `@mutations-` tag, no subset-scoped fixture: genuinely a
  container-grammar round trip, correctly staying at artifact level.

Everything left in place is fully attributed, case by case, with the specific blocking URI or
anomaly named, in `📓️b6-artifact-level-test-census.md`.
