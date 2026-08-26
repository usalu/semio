# Cross-language differential oracles — the working recipe

Date 2026-08-25. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
Grounded on one converted case: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-text`,
which went from `@no-oracle-semio-text-mutation-semantics` / 0 oracle-executed scenarios to a real
Python-versus-Rust differential over 22 scenarios. Everything below was executed, not designed.

**The path works end to end.** A second implementation in another language, written from a committed
specification, registered as a normal oracle, dispatched by the runner and compared against the Rust
subject under a comparison profile — all of it runs today with no framework change.

---

## 0. What the six sibling agents must copy

| Piece | Where it goes | Notes |
|---|---|---|
| The independent implementation + its adapter | `<case>/🐍️component.py` (or `🟦️component.ts`) | ONE file. The taxonomy rejects any other filename in a case directory. |
| Oracle registration | `<subset>/🧪️oracle/🔣️component.json` → `oracles[]` | `"ecosystem": "python"` \| `"javascript"` \| `"rust"` \| `"go"` \| `"dotnet"`. |
| Removal of the old decision | same file → delete the `noOracleDecisions` entry | Leaving it is a lie: it says no reference exists. |
| Feature tag | `@oracle-<id>` replacing `@no-oracle-<id>` | Exactly one of the two. |
| Scenario modes | `@mode-differential` where a second PRODUCER now exists | Keep `@mode-round-trip` for identity/carrier scenarios. |
| Subject adapter | `<case>/🦀️component.rs` | Drop its oracle registrations. Keeping them would put our answer on both sides. |

---

## 1. The adapter contract, per host

### Dispatch — how the runner picks the language

`oracleDecision` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:515`:

1. reads the feature's `@oracle-<id>`;
2. finds that id in the MERGED registry (every `<owner>/🧪️oracle/🔣️component.json` in the repo, so the
   entry can live in your subset — nothing central is edited);
3. maps `entry.ecosystem` → implementation, with `"javascript"` → `"typescript"` the only rename;
4. **requires the case to carry that implementation's adapter file**, else
   `oracle <id> needs a <impl> adapter to run in`.

The subject phase is unrelated: it runs every implementation the case has an adapter for.

### Python — `📦️packages/🐍️python/🐍️host.py`

The host installs itself as the importable module `semio_repo_test`, loads your file by path and
calls its module-level `adapter()`.

```python
from semio_repo_test import Adapter, Context, Outcome, digest

def handler(ctx: Context) -> Outcome:
    ...
    return Outcome(projection, raw=some_bytes)          # raw is optional

def adapter() -> Adapter:
    return Adapter("python").oracle("mutate-insert-run", handler)
```

* `Adapter(impl).oracle(scenario_id, handler)` / `.subject(...)` — **keyed by the FULL EXPANDED
  scenario id** (`mutate-insert-run`), never the outline's base id. A missing registration is an
  `errored` result, never a skip.
* `Outcome(projection, raw=None, diagnostics=None)`. `projection` is what parity compares; `raw` is
  hashed and written out but never compared.
* `ctx.fixture(uri)` / `ctx.fixture_bytes(uri)` / `ctx.copy_fixture(uri, name)` — an **undeclared URI
  raises**, so every path must appear somewhere in the feature.
* `ctx.scenario` is the raw plan dict: `{"id","name","level","mode","seed","steps":[…]}`. Steps are
  dicts with `keyword`, `text`, and optionally `docString` / `dataTable`. **There is no
  `ctx.doc_string()` in Python** — the Rust `Context` has one, the Python one does not. Read it
  yourself:
  ```python
  next(step["docString"] for step in ctx.scenario["steps"] if step.get("docString"))
  ```
* `raise AssertionError(...)` → `failed`; any other exception → `errored`. Both carry the traceback.
* `digest(bytes)` is the coordinator's sha256-hex-truncated-to-32 — the SAME function the Rust host
  exports, so a digest is directly comparable across the two languages.

### TypeScript — `📦️packages/🟦️typescript/🏃️host.ts`

Same shape, run through `bun`. Bare specifiers resolve from the repository root `node_modules`
(one install, one lockfile); nothing is installed into a private tree.

### Rust — the subject side

`ctx.doc_string()` / `ctx.doc_json()` / `ctx.data_table()` exist here. `ctx.scenario.steps` is
`Vec<(keyword, text)>`. `Outcome::projection(Json)` / `Outcome::with_raw(Vec<u8>, Json)`.
The subject half must stay behind `#[cfg(feature = "sut")]` — the generated host only turns that
feature on for `role == "subject"`.

---

## 2. How the host resolves the language's packages

`oracleHostPackages[]` in a `🧪️oracle/🔣️component.json`, `path` present or absent:

* **`path` present** → local in-repo source. Rust links the crate by path; Python puts the directory
  on `PYTHONPATH`.
* **`path` absent** → an external distribution. Python builds a cache-local venv at
  `.🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-<digest>` with `--system-site-packages`, accepts a
  package only if it is importable **and** at the declared `version`, and `pip install`s it into the
  venv otherwise. TypeScript only VERIFIES that the package resolves from the repo root.
  An unprovisionable declaration stops the run before any scenario executes.

**Owner scoping is the trap.** `oracleHostPackagesFor` only collects contributions whose `owner` is
the case's owner or an ANCESTOR of it. A case owner is the directory containing `🧪️tests`. For
`mutate-semio-text` that is `…/🗿️artifacts/🧿️semio` — so an `oracleHostPackages` entry in the
`…/🪆️subsets/✳️text/🧪️oracle/🔣️component.json` (a DESCENDANT) would be **silently ignored**, while
the plugin-level `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️component.json` entries DO apply.
`oracles[]`, `noOracleDecisions[]`, `mutationCatalogs[]` and `comparisonProfiles[]` are merged
repo-wide and are NOT owner-scoped — only `oracleHostPackages` is.

**If your second implementation is written from a spec, you need no host package at all.** Keep the
whole implementation in the one adapter file and declare:

```json
{
  "id": "semio-text-python-independent",
  "ecosystem": "python",
  "package": "",
  "capabilities": ["semio-v1-text-mutate"],
  "comparisonProfiles": ["ordered-json-v1"],
  "license": "AGPL-3.0-only",
  "testOnly": true,
  "rationale": "… why a second implementation, and what it was written from …"
}
```

`package: ""` is load-bearing and deliberate: `oracleLinkedPackages` skips zero-length names, so the
entry contributes NOTHING to `🔒️dependencies.json` and nothing to the import-purity probe. Verified —
`bun ./📜️script.ts dependency` stayed at `entries=232 … test-oracle=30`, exit 0.

---

## 3. What counts as a second implementation for a semio-native carrier

`.dsl.semio` / `.pack.semio` has no third-party reader, so the reference is a second implementation
written from the format's own committed documents. For `s.stdio.semio.text` those were:

| Document | What it gave |
|---|---|
| `…/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` | `document = artifact-mark schema-line runs-line`, `run = "[" hex "," hex "," "[" mark-list? "]" "]"`, `mark-kind = "b" \| "i" \| "c" \| "l"` |
| `…/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` | `format u8`, then varint-length-prefixed UTF-8 `schema` |
| `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` | the seven verbs and their positional argument lists |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs` (envelope region) | `semio <plugin>.<artifact>.<component> v<n>` text preamble; `0x89 'S' 'E' 'M' 0D 0A 1A 0A` + u32-LE token length + token for binary |
| the committed `(before, mutation, after)` vectors | the JSON wire form of each verb |

**The protocol description stops short and says so** — the repeated `runs` record is declared an
opaque `payload` chain. That gap was closed by DERIVING the record layout from the committed
`🎒️example.pack.semio` bytes (field order from the grammar, `varint` = LEB128, mark ordinal = the
grammar's `b|i|c|l` position, confirmed by `bold → 0x00` and `link → 0x03` in the file). Say so in the
adapter docstring. The derivation is then *pinned*: the implementation re-encodes that committed file
byte for byte, which it could not do from a misreading.

**Do not read the Rust to write the Python.** Read the grammar. The evidence is only worth something
because the two were produced from one written specification by two hands. The one exception here was
deliberate and is named above: the semio ENVELOPE has no separate prose document, so its normative
statement is the framework carrier module `🧬️semio/🦀️component.rs` — framework carrier code, not the
subject codec. `📸️snapshot/🦀️component.rs`, `🧬️mutations/**` and the diff codec were opened only for
the *signatures* the Rust subject adapter calls; none of their bodies informed the Python.

---

## 4. The case shape that worked

22 scenarios, all executed in both roles:

| base id | mode | input | what each side asserts in role |
|---|---|---|---|
| `mutate-<kind>` ×7 | `@mode-differential` | the REAL committed note via `asset://`, parameters in the scenario's doc string | applies the verb; projection is the resulting snapshot |
| `inverse-<kind>` ×7 | `@mode-differential` | same | applies the verb, then its OWN computed inverse; requires the note back; projects `{mutated, restored}` |
| `spec-vector-<kind>` ×7 | `@mode-differential` | the committed handcrafted `(before, mutation, after)` triple | requires the committed after-snapshot |
| `identity-round-trip` | `@mode-round-trip` | the note's DSL **and** its pack twin | reproduces BOTH committed files byte for byte; projects the document plus digests and lengths of what it emitted |

Notes that matter:

* **`spec-vector-` was ADDED, not substituted.** The old case's only inputs were the handcrafted
  per-kind fixtures; the new one keeps them *and* adds the real artifact. Nothing was swapped away.
* The mutation parameters live in the feature (`Examples` + a doc string), and the specification
  vector paths live in the feature (`asset://` inside the step text, with `<dir>`/`<fixture>`
  columns). **Both adapters read them from the plan.** No adapter holds a transcription that could
  drift from what the other one read.
* `inverse-` projects `{mutated, restored}`, not just `restored`. Projecting only the restored
  document would make all seven rows project the same value and the differential would be vacuous.
* `identity-round-trip` projects the two digests. Byte-exactness is asserted in role on each side
  (Rust through `law::carrier_is_exact`), and the digests make the two sides' BYTES comparable across
  languages — which is what stops "the carrier reproduces itself" from being a self-comparison.

---

## 5. Every trap hit, in the order they bite

1. **`parity` runs the oracle adapter AS A SUBJECT, and it errors.**
   The subject phase iterates `Object.keys(discovered.adapters)`. A case with `🦀️component.rs` +
   `🐍️component.py` therefore runs a *python subject* pass, and an oracle-only Python adapter answers
   `adapter has no subject registration for scenario …` — `errored`, plus a `parity failed:` line per
   scenario (the null projection cannot match the oracle's).
   **This is pre-existing**, not something this conversion introduced: `parity exhaustive --case
   extract-text-pdf-1-4` (the wave-12 Python proof case, python adapter only) already reports
   `executed=4 passed=2 errored=2 parity=0/2`.
   **Workaround, and it must be in your report:** `parity … --implementation rust`.
   `--implementation` narrows the SUBJECT phase only; the oracle phase still dispatches to Python.
   **Do NOT "fix" this by registering the Python handlers as subjects too** — that makes the
   reference its own subject and manufactures a guaranteed-green self-comparison.
   The `@implementation-<impl>` scenario tag is parsed (`📦️index.ts:392`) and then never read by
   `planExecution`; it does not solve this. A framework fix is needed and is not a case's business.

2. **`asset://` cannot leave the artifact root.** It resolves against `discovered.owner` (the parent
   of `🧪️tests`) with an explicit `startsWith(guard + sep)` escape guard. The 8.7 MB
   `🎞️gif/…/🗣️example.dsl.semio` is therefore unreachable from a `🧿️semio` case. Use the richest
   document inside YOUR artifact and say in the feature description that you did — do not copy a
   multi-megabyte file into `🧫️fixtures` to get around it.

3. **`ecosystem` values are `rust|javascript|go|python|dotnet`.** `"typescript"` is not one of them;
   write `"javascript"` and the runner maps it. `"cargo"`, `"pip"`, `"npm"` are all rejected.

4. **The oracle entry's `capabilities` must contain the feature's `@capability-` exactly**, and its
   `comparisonProfiles` must contain the feature's `@comparison-` exactly. Otherwise
   `oracle-capability-mismatch` / `oracle-profile-mismatch`.

5. **Contract runs repo-wide even with `--case`.** `testing/discovery` breaches from other plugins
   (today: `🧰️framework 42 vs 35`, `✏️s 4 vs 1`, all `🎬️sequence` / `🎞️animate` `.test.ts`/`.test.js`
   files from concurrent sessions) make `contract` exit 1 no matter what your case does. Read the
   rule ids, not the exit code. Your case is clean iff no breach names its path.

6. **Deleting the `noOracleDecisions` entry is mandatory, and check nothing else points at it.**
   `grep -rn "<decision-id>"` — a second feature still tagging it becomes `unknown-no-oracle-decision`.

7. **Only one file may be added to the case directory.** `unknown-adapter-filename` fires on anything
   that is not `component.feature` or a taxonomy adapter name. The whole second implementation lives
   in that one adapter file; scratch drivers go in the ticket folder.

8. **Extra scenario families are allowed.** The mutation-coverage gate only requires
   `mutate-<kind>` + `inverse-<kind>` for every catalog kind, and only complains about STRAY ids that
   start with `mutate-`. `spec-vector-<kind>` passes freely.

9. **Doc strings and data tables are substituted per `Examples` row** (`📦️index.ts:393`), so
   `<mutation>` inside a `"""` block works. A cell may not contain a bare `|` (escape it `\|`).

10. **The Python venv is provisioned from an ANCESTOR manifest.** A `🧿️semio` case inherits
    `✏️s/🔌️plugins/🗄️stdio`'s `pypdf` and `simplejson` declarations, so the first Python run of ANY
    stdio case builds/validates that venv. Budget for it; it is cached afterwards.

11. **Emoji directory names are not uniform in their variation selectors** — `📥insert-run` has no
    VS16, `🗑️remove-run` does. Generate feature tables from `readdir` output rather than typing them.

12. **Python `Context` has no `doc_string()`.** See §1.

---

## 6. Verification — real output

Every command was run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; exit codes are the
tool's own, never a pipe's.

### Before

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-text      # exit 0
[test] not-exercised ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-text (recorded no-oracle decision semio-text-mutation-semantics — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1

$ bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-text     # exit 0
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0
```

### After

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-text               # exit 1
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery
  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

Both breaches are `testing/discovery` counts owned by other plugins (`🎞️animate`, `🎬️sequence`), present
before this work and unchanged in kind by it; **zero breaches name `mutate-semio-text`**, and the
`testing/contract`, `testing/oracle`, `testing/fixture` and `testing/taxonomy` families are all at zero.

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-text      # exit 0
[test] level=exhaustive cases=1 executed=22 passed=22 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts dependency                                                      # exit 0
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio                               # nothing else broken
[test] level=exhaustive cases=101 executed=1343 passed=1343 failed=0 errored=0 parity=0/0 not-exercised=24
```

`mutate-semio-text` is gone from the not-exercised list; the other 18 `mutate-semio-*` cases are
still on it, which is what the six sibling agents are for.

**Parity — the proof the two implementations agree.**

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-text --implementation rust
[test] level=exhaustive cases=1 executed=44 passed=44 failed=0 errored=0 parity=22/22
                                                                             # exit 0, 1:43 wall
```

44 executed = 22 scenarios in the Python ORACLE role and the same 22 in the Rust SUBJECT role.
`parity=22/22` is `evaluateParity` comparing each pair under `ordered-json-v1` — array order
significant, key order not, no `ignoreKeys`, no tolerance.

Without `--implementation rust`, trap 1 fires and the same run is:

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-text          # exit 1
[test] level=exhaustive cases=1 executed=66 passed=44 failed=0 errored=22 parity=22/66
[test] parity failed: …::mutate-insert-run::python::subject (1 differences)
  … 21 more, one per scenario …
[test] cross-subject parity failed: …::mutate-insert-run::python~rust (1 differences)
  … 21 more …
```

The same 44 results pass and the same 22 comparisons are equal; the extra 22 `errored` are the
python SUBJECT pass, which has no registrations, and the 44 red lines are that null projection being
compared twice.

The two `identity-round-trip` projections, read from
`.🧬semio/🦑️repo/⚡️cache/tests/results/…-{oracle-python,subject-rust}/`, are identical including the
digests of the bytes each language emitted:

```
{"document":{…},"dslDigest":"df28edec0b6978c6a38b6e8a687059e6",
 "packDigest":"983d00e56cc4ad465838326a1fb7d615","dslLength":203,"packLength":118}
```

Two implementations, two languages, the same 203 DSL bytes and the same 118 pack bytes.

### The negative control — the case fails when it should

`ReorderRuns` in the Python oracle was temporarily changed from move-semantics
(`runs.insert(target, runs.pop(source))`) to a swap, and the same command run again:

```
[test] level=exhaustive cases=1 executed=44 passed=43 failed=1 errored=0 parity=19/22
[test] parity failed: …::mutate-reorder-runs::rust::subject (8 differences)
[test] parity failed: …::inverse-reorder-runs::rust::subject (8 differences)
[test] parity failed: …::spec-vector-reorder-runs::rust::subject (1 differences)
                                                                             # exit 1
```

Three scenarios red, one of them (`spec-vector-`) red in role because the oracle's own committed
after-snapshot assertion caught it first. The edit was reverted and the run returns
`executed=44 passed=44 … parity=22/22`, exit 0.

---

## 7. The four files this conversion touched

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-text/🐍️component.py` | **new** — 498 lines: the independent implementation and its oracle adapter |
| `…/🧪️tests/mutate-semio-text/component.feature` | rewritten — `@oracle-…`, 22 scenarios, real artifact + parameters + specification-vector paths |
| `…/🧪️tests/mutate-semio-text/🦀️component.rs` | rewritten — subject only, reads its inputs from the plan, drops its oracle registrations |
| `…/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧪️oracle/🔣️component.json` | `noOracleDecisions` removed, `oracles[]` gains `semio-text-python-independent` |

Nothing else was edited: no framework file, no shared manifest, no `Cargo.toml`, no `🔒️dependencies.json`,
no fixture, no comparison profile, no `ignoreKeys`.

---

## 8. Honest limits of this conversion

* **The artifact is real but small.** The richest committed `s.stdio.semio.text` document in the
  `🧿️semio` artifact is the three-run note (203 bytes of DSL, 118 of pack). It does carry every field
  the subset has — two languages, an unmarked run, a `bold` mark and a `link` mark with a non-empty
  `href` — and the parameters were chosen to break a plausible wrong codec. But it is not the
  multi-megabyte document THE STANDARD's word "complex" evokes, and `asset://`'s owner-root guard is
  what prevents borrowing a bigger one. Stated, not hidden. Producing a genuinely large real
  `s.stdio.semio.text` example is a separate piece of work with its own provenance question.
* **`parity` needs `--implementation rust`** until the framework stops running an oracle-only adapter
  in the subject role (trap 1).
* **The pack `runs` layout was derived from committed bytes**, because the protocol document
  deliberately stops there. The derivation is pinned by byte-exact re-encoding but it is not a
  reading of a complete specification, and that boundary belongs in the record.
