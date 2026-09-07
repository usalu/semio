# 🧬️ W-D0 — mutation foundation custodian + generator owner

Lane report. Appended as work lands; every claim below is backed by pasted command output in the
same section or it is marked OPEN.

## 1. What the predecessor actually left on disk (verified 2026-09-06, this shift)

Verified by reading the files, not by trusting the earlier `📓️status.md` claims.

| Claim in `📓️status.md` | Disk truth |
|---|---|
| `♻️replace-model` deleted | ✅ gone; `🧬️mutations/` holds 17 leaf dirs + the 8 non-kind aggregate siblings |
| 17 kinds generated (12 model-root `1–12`, 5 zone `100–104`) | ✅ exactly those numbers in the generator spec table and on disk |
| 34 fixture cases | ✅ 34 `🧪️tests/<case>/` dirs, **but all 170 fixture JSON files are still the seeded `{}`** — the `SEMIO_ENERGY_WRITE_FIXTURES=1` pass has never run (the crate has not compiled yet) |
| `Model` gained `run_period` + `schedules` | ✅ |
| snapshot/diff gained the `weather_link` link slot | ✅ |
| diff's link slots became a typed `EnergyLinkSlotDelta` | ✅ |
| Python second implementation rewritten for all 17 | ✅ every spec row carries a `python=`/`python_invert=` body |
| "switch the snapshot link codecs off serde" (in progress when killed) | ⚠️ half done — see §3 |

Generator idempotency, measured (run twice, checksummed):

```
$ python3 T/🐍️generate-mutation-leaves.py
17 kinds, 34 fixture cases emitted
$ diff gen-before.txt gen-after.txt && echo IDEMPOTENT-MUTATIONS-TREE
IDEMPOTENT-MUTATIONS-TREE          # 309 files under 🧬️mutations/
$ ... second run, aggregate outputs outside 🧬️mutations/ ...
IDEMPOTENT-OTHER                   # crate mounts, 🔮️oracle/🔣️.json, 🥒️.feature, 🦀️.rs, 🐍️.py
```

So the generator was already idempotent and already sorted rows by ledger number
(`kind()` inserts into `KINDS` by `number`, never by declaration order), which is what makes four
groups appending into four disjoint regions produce the same files rather than a conflict.

## 2. Generator hardening (custodian work)

`audit()` previously checked three things (duplicate number, duplicate emoji, FE0F/NFC). It now runs
ten, all BEFORE the first file is written, and every one was probed by injecting a deliberately bad
row — output pasted verbatim:

```
baseline: clean
caught      duplicate ledger number: ledger number 1 is claimed by both rename-model and other-kind
caught      emoji without U+FE0F: other-kind: '🌟' needs exactly one trailing U+FE0F
caught      directory pattern (single word slug): otherkind: directory '🌟️otherkind' does not match taxonomy.json's mutationDirectoryPattern '^.+\\uFE0F[a-z][a-z0-9]*(?:-[a-z0-9]+)+$'
caught      ledger number out of range: other-kind: ledger number 1500 is outside every range in the ledger's §2
caught      non-NFC directory name: other-kind́: directory '🌟️other-kind́' does not match taxonomy.json's mutationDirectoryPattern
caught      missing refusal fixture: other-kind: needs at least one ✅️ happy and one ⛔️ refusal fixture, got ['✅️']
caught      case path over budget: other-kind/✅️xxx…: 300 UTF-16 units exceeds the 227-unit path budget
caught      non-kind sibling emoji: other-kind: 🦀️ is a non-kind sibling inside 🧬️mutations/
```

The `mutationDirectoryPattern` is **read out of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`**
at audit time, not copied into the generator — a copy would drift from the authority it claims to
enforce. `🔍️discovery/🟦️.ts:2783` applies exactly this regex plus an NFC check to every sibling of
`🧬️mutations/`, and a directory that fails it is INVISIBLE to discovery rather than reported.

Also added: **stale fixture-case pruning** (`prune_stale_case_directories`). A `🧪️tests/<case>/`
directory a spec row no longer declares is removed, so renaming a case leaves no orphan tracked path.
Verified live:

```
$ mkdir -p …/🏷️rename-model/🧪️tests/✅️stale-old-name/📸️snapshot && echo '{}' > …/🔣️.json
$ python3 T/🐍️generate-mutation-leaves.py
removed stale fixture case 🏷️rename-model/✅️stale-old-name
17 kinds, 34 fixture cases emitted
```

Kind directories with no spec row are **reported, never deleted** — a directory a group has not yet
added a row for is in-flight work, and deleting on a guess is how one worker destroys another's files.

### 2.1 FINDING — the ledger's 190-unit path budget was wrong, and unreachable

The ledger's §3 said the repo-wide sweep renames any `🧪️tests/` path over 190 UTF-16 units. Measured:

```
$ git ls-files | (count UTF-16 units)
tracked: 75887   over190: 1616   max: 227
```

So 190 is not a live gate — `🪟️shorten-long-paths.ts` is a one-off script in `26/04/08`'s own ticket
folder, and 1616 tracked paths already exceed it (`🧩️puzzle`'s OWN mutation fixtures reach 225). It is
also **unreachable for this vocabulary**: the prefix up to `🧬️mutations/` is 105 units and
`📐️change-zone-floor-area-participation` spends 39 more, leaving 5 units for a case name. 15 of the
predecessor's 34 case paths were already over it (worst 208).

The generator therefore enforces **227** — "never become the repository's new worst path" — which
still refuses the 40-character case name a group would otherwise invent. `📓️mutation-tag-ledger.md`
§3 was corrected to state the measured number instead of the stale one, and
`📓️mutation-kind-template.md` gained a §11 listing every refusal above.

### 2.2 The generator survives four groups appending at once

Re-run against the tree as the group workers were landing into it:

```
$ python3 T/🐍️generate-mutation-leaves.py
78 kinds, 156 fixture cases emitted
```

All 78 rows (G2/G3 families included) pass the ten validations, no orphan-directory NOTEs, and the
run is still idempotent. `kind()` inserts into `KINDS` by ledger number rather than by declaration
order, so the enum ordinal — which IS the binary tag — does not depend on which group's region was
parsed first.

## 3. GATE `mutation-outcome-law` — energy went 111 breaches → 0

The biggest finding of this shift, and it was invisible until the gate was actually run.

```
$ bun ./📜️script.ts verify mutation-outcome-law        # before
error: [verify mutation-outcome-law] 155 breach(es)
  … 111 of them under ✏️s/🔌️plugins/🔋️energy …
      54 × message code "mutation.duplicate"
      45 × message code "mutation.invalid-payload"
       8 × message code "mutation.target-in-use"
       4 × "…/🔺️diff/🦀️.rs" returns protocol::MutationOutcome<..> but never references one of the 7 frozen codes
```

`📋️contract-freeze.md` §C2 freezes SEVEN message codes repo-wide (`📜️script.ts`'s
`POLICY_MUTATION_FROZEN_CODES`, gate rules 1 and 2 at `📜️script.ts:29129`/`:29178`):

`mutation.target-missing` · `mutation.no-op` · `mutation.partial` · `mutation.clamped` ·
`mutation.duplicate-id` · `mutation.invariant` · `mutation.cascade`

There is **no per-plugin code, ever** — a refusal's specificity belongs in its message, not in a
private code. Three illegal codes had spread through the spec table as the groups landed (including
one of my own, `update-ground-temperature`). Swept centrally in the generator:
`mutation.duplicate`→`mutation.duplicate-id`, `mutation.invalid-payload`→`mutation.invariant`,
`mutation.target-in-use`→`mutation.invariant` (referential integrity IS a document invariant;
`mutation.cascade` is the INFO code for a cascade that HAPPENED, not for a RESTRICT refusal).

After:

```
$ bun ./📜️script.ts verify mutation-outcome-law        # after
error: [verify mutation-outcome-law] 44 breach(es)
$ grep -c '🔋️energy' …
0
```

The remaining 44 are pre-existing and owned elsewhere: `🗄️stdio` ×18 (pdf/step/ifc private codes),
`📸️remodel` ×16, `📋️forms` ×4, `🌍️gis` ×1, plus four framework ones — `🏪️store` ×3
(`mutation.duplicate`, `mutation.conflict`), `📡️spr/🧾️wire` missing all three `MergePolicy` variants,
and the two `🗣️dsl/✨️derive` copies having drifted out of byte-identity. **None of them is energy's**
and none is this ticket's to fix; they are recorded here so the next reader does not re-diagnose them.

### 3.1 So it cannot come back

`audit()` now refuses, at the spec row, any message code in a `diff=`/`inverse=` body that is not one
of the seven, and refuses a `diff=` that reports no frozen code at all (the gate's rule 1). The
generator stops before writing a single file, with the allowed set printed in the message. Probed:

```
caught  private message code in diff: create-x: diff reports message code 'mutation.invalid-payload',
        which is not one of the seven frozen codes ([…]) — `mutation-outcome-law` refuses it repo-wide
caught  diff with no frozen code at all: create-x: diff returns a MutationOutcome but never reports
        one of the seven frozen codes — every verb family owes a real refusal/warning detection
```

This is a strictly better instrument than the gate itself for a group worker: the gate reads the
GENERATED `🔺️diff/🦀️.rs` files, so a bad code was only ever reported minutes later, repo-wide, and
usually in somebody else's run.

### 3.2 The audit caught a live cross-group collision the same hour

While the groups were landing, a re-run failed with:

```
AssertionError: emoji '📌️' is claimed by both create-setpoint-manager and create-constant-schedule
```

— G3's ledger 600 against G4's ledger 900, exactly the class of conflict the ledger exists to
prevent. G4 resolved it. Root cause is bookkeeping, not carelessness: `📓️mutation-tag-ledger.md` §5
still lists rows 900+ as `reserved | —` with no emoji, so a group reading the ledger sees a free
slot for one another group has already taken in the spec table. Logged for all four groups.

## 4. Other gates, run this shift

- `bun ./📜️script.ts verify taxonomy report` — **RED, and not energy's**:
  `Normalization requires an explicit repository-boundary decision before authored classification:
  ♻️mit-bestand/🔎️recherche`. That path is a **gitlink** (`git ls-files -s` → mode `160000`,
  `.gitmodules` → `https://github.com/usalu/recherche.git`) committed at `2026-09-06 20:48:37 +0200`
  by a concurrent peer — minutes before this shift started. `inventoryTaxonomyWithSourceParentPruning`
  refuses to classify anything while an unresolved repository boundary exists, so **every**
  taxonomy gate (report AND enforce) is blocked repo-wide for every lane until whoever added that
  submodule records the boundary decision. Energy contributes nothing to it. Recorded, not "fixed" —
  it is not this ticket's file.

## 5. Foundation code landed this shift

### 5.1 The snapshot/diff link slots are now completely off `serde`

The predecessor's in-progress item. On disk the two link slots (`referenced_model`, `weather_link`)
already went through `ToValue`/`FromValue` + `pack::json` in both codecs, but four `serde_json`
helpers (`enc_json`/`dec_json`/`write_json`/`read_json`) were left behind, dead, together with a
docstring still claiming "this ONE field keeps the `serde_json` round trip". Removed both; the
docstring now states what the file actually does. (A peer's sweep removed the `Serialize`/
`Deserialize` derives and the `#[serde(...)]` attributes from `EnergyModelSnapshot`,
`EnergyModelDiff` and `EnergyLinkSlotDelta` while this shift was running — with the dead helpers gone
those two files carry no `serde` reference at all any more.)

Codec-completeness proof strengthened, because the existing test only ever exercised ONE link slot:

- `structure_zones_and_both_link_slots_round_trip_through_text_and_binary` — the sample now carries a
  `weather_link` too, so the weather slot is actually covered by the round trip.
- `absent_link_slots_round_trip_as_none` — each slot absent on its own AND both absent, since a codec
  that reads the two slots in the wrong order still round-trips whenever they happen to agree.
- `the_two_link_slots_are_not_interchangeable_on_the_wire` — swapping their contents must change both
  the text and the binary form. Without this, a codec that wrote one slot twice passes everything else.

### 5.2 `next_entity_id` — one helper, at the artifact root, with the law as a test

`✏️editor/🦀️.rs` had a private `next_entity_id`. Every `create-*` kind in G1/G2/G3/G4's ranges needs
the same convention, so it now lives once at the artifact root
(`🗿️artifacts/🔋️model/🦀️.rs`, `//#region 🆔️EntityIdMinting`) as `pub fn next_entity_id`, with the
minting LAW stated as tests rather than as prose:

- an empty collection starts at **1**, never 0 — 0 stays the "no entity" sentinel every wire form
  already reads it as;
- a populated one continues past its **maximum**, not its length;
- a hole left by a delete is **never refilled** — reusing a deleted id is exactly how a dangling
  reference silently re-points at a different entity;
- minting is **pure**: it does not advance until the entity actually lands in the collection;
- `u32::MAX` **saturates** rather than wrapping to 0, so exhaustion surfaces as a detectable
  collision instead of as a silent alias of the sentinel.

I did NOT edit `✏️editor/🦀️.rs` to delete its private copy — W-E is actively editing that file this
shift. Logged for them in `📓️status.md`.

### 5.3 Whole-document load: `ArtifactStore::reset`, and where it actually lives

The brief said "provide the `ArtifactStore::reset` wiring; W-E calls it". Reading the framework
first changed the shape of that deliverable, so recording the reasoning:

**A guest plugin never holds an `ArtifactStore` to reset.** `ArtifactStore::reset` is the store's sole
public reload API (`🏪️store/🦀️.rs:14228`), but the caller is the framework:
`🔌️plugin/🦀️.rs:25289`'s `load_document_pack` parses the pack+spr, honours a persisted cursor, and
calls `self.store.reset(...)`; that is reached from `AppCommand::LoadDocument`
(`🔌️plugin/🦀️.rs:32374`), which the host loops back after the app emits
`kernel::Effect::LoadDocument`. So a plugin-side `reset(store, …)` helper would have been unreachable
by construction. What the plugin owes is the genesis `(pack, spr)` pair.

W-E had already landed exactly that as a private `load_document_effect`, matching `📐️cad`'s
`reset_document_effect`. Two real gaps in it, both closed at the artifact root instead of duplicating
a third copy for W-F's epJSON import:

1. the document id was the hardcoded literal `"model"`;
2. **both link slots were silently dropped** — it builds through `energy_snapshot_with_state`, which
   hardcodes `weather_link: None` and has no parameter for it, so re-opening a document unbinds its
   weather file without saying so.

Landed in `//#region ♻️WholeDocumentLoad`: `EnergyModelLinkSlots { referenced_model, weather_link }`
(a struct, not two positional `Option<ArtifactLink>` that would type-check when swapped) with
`EnergyModelLinkSlots::of(&snapshot)`, `energy_snapshot_with_links`, and
`energy_model_load_document_effect(document_id, model, &links)`. Two tests: the slots survive a load
when carried and clear when not, and the effect's pack decodes back to exactly the model it was given.
W-E and W-F asked in `📓️status.md` to call it rather than keep/add a private copy.

## 6. OPEN — everything that needs the crate to compile

`cargo check -p semio-s-plugin-energy --lib --tests` has NOT produced a result this shift, and the
reason is host-level, not energy-level. **Swap is exhausted** — `vm.swapusage: used = 70014.50M,
free = 641.50M` of 70656 M, with ~68 MB of free RAM on a 32 GB machine — so `cc` is being OOM-killed
and peer logs (`w1-check-1.txt`, `wc-check-1.txt`) end in `error: linking with `cc` failed` on
trivial proc-macro crates (`futures-macro`, `tokio-macros`, `thiserror-impl`, `displaydoc`). A bare
`cc hello.c` links fine, so the toolchain is healthy; the box is not. Census at the time: 26 cargo,
15 rustc, 28 bun, 16 node. Blocked behind that, and NOT claimed as done:

- **Fixture materialization.** All committed fixture JSON is still the seeded `{}`
  (170 files at 17 kinds; 552 cases now). `SEMIO_ENERGY_WRITE_FIXTURES=1` runs inside the crate's own
  test binary, so it cannot run until the crate compiles. Until then the eight fixture laws, the
  `🥒️.feature` rows and the Python second implementation all have nothing to compare against.
- **The crate tests** for the model-root group's twelve kinds, and the three new artifact-root tests
  and three snapshot codec tests added above.
- **`verify-taxonomy-report` / `verify-taxonomy-enforce`**, blocked on the unrelated
  `♻️mit-bestand/🔎️recherche` gitlink (§4).

## 7. Ledger drift is now reported on every run

`report_ledger_drift()` parses `📓️mutation-tag-ledger.md` §5's allocation tables and, for each kind
in the spec table, names the row that disagrees: missing entirely, pointing at a different slug,
carrying a different emoji, or still marked `reserved` for a kind that has landed. NOTE lines, never
an assert — a stale ledger row is a bookkeeping debt owed by the group that landed the kind, and
blocking every other group's run on it would be the worse trade.

```
NOTE ledger drift: 402 change-people-gain-zone: ledger says —, spec table says 🚶️
…
NOTE: 61 of 276 kinds have a stale or missing row in …/📓️mutation-tag-ledger.md
      — the group that landed each one owes that line
276 kinds, 552 fixture cases emitted
```

This is the same gap that produced the `📌️` collision in §3.2: a group reading the ledger for a free
emoji sees `— | reserved` on a row another group has already shipped. Model root `1`–`12` is clean.

## 8. Summary of the generator's contract, as it now stands

One spec table → 4712 files under `🧬️mutations/` plus the crate mounts, the subset oracle catalog,
the `🥒️.feature` tables, the Rust adapter and the Python second implementation. Re-running is a
no-op; running it after four groups have appended to four disjoint regions produces one file, not a
conflict, because `KINDS` is ordered by ledger number rather than by parse order and that order IS
the binary ordinal. Before writing anything it refuses eleven classes of mistake (§2, §3.1), and
after writing it reports ledger drift (§7) and any kind directory with no spec row (§2).

## 9. Cross-registry set-equality, verified without cargo

`cargo check` cannot run on this host right now (§6), but the failure modes that actually bite a
276-kind fan-out — a leaf directory with no mount, a `DIRECTORIES` row for a directory that no longer
exists, an enum whose ordinal order stops matching the ledger (the ordinal IS the binary tag), a
duplicate protobuf field number — are all checkable from the emitted files directly. Run against the
tree as it stands, all four registries agree exactly:

```
leaf dirs on disk : 276
DIRECTORIES rows  : 276      disk − table: []      table − disk: []
crate mounts      : 278      mounts − disk: ['💾️binary', '📝️text']   disk − mounts: []
enum variants     : 276      enum order == KINDS order: True
proto oneof       : 276      unique numbers: 276   ascending: True
```

The two extra crate mounts are the aggregate's own `💾️binary`/`📝️text` siblings, which are not kinds.
`enum order == KINDS order` is the load-bearing one: it is what makes a ledger number and a binary
ordinal the same fact, and it holds even though four groups appended into four regions in whatever
order they happened to finish.

## 10. Where this lane stands at end of shift

Done and evidenced above:

1. Predecessor's disk state verified item by item (§1) — one claim was overstated (fixtures are all
   still `{}`) and one was half-finished (the serde removal), both now recorded or fixed.
2. Generator is idempotent, prunes stale fixture cases, refuses eleven classes of bad spec row before
   writing anything, and reports ledger drift after (§2, §3.1, §7).
3. `mutation-outcome-law`: energy 111 breaches → **0** (§3). This was invisible until the gate was
   run and would have failed the ticket's own definition of done.
4. Snapshot/diff link slots fully off `serde`, with a codec proof that actually distinguishes the two
   slots (§5.1).
5. `next_entity_id` promoted to a tested artifact-root helper with its minting law as tests (§5.2).
6. Whole-document load consolidated at the artifact root, with the document id parameterized and both
   link slots preserved — two real bugs in the private copy it replaces (§5.3).
7. Cross-registry set-equality across disk / `DIRECTORIES` / crate mounts / enum ordinals / protobuf
   field numbers, at 276 kinds (§9).

Not done, and honestly blocked, not forgotten (§6): every fixture JSON is still `{}`, no crate test
has run, and the taxonomy gates are red on an unrelated gitlink. The blocker is the host — swap is
exhausted and `cc` is being OOM-killed mid-link — not the energy sources. A `cargo check` is queued
behind a peer's, which after 49 minutes of wall clock has accumulated 9m44s of CPU (≈13 % efficiency
under thrash); it is progressing, not hung.

The first thing the next shift on this lane should do, once one `cargo check` comes back green:

```
SEMIO_ENERGY_WRITE_FIXTURES=1 RUSTC_WRAPPER= CARGO_TARGET_DIR=…/target-energy-e2e \
  cargo test -p semio-s-plugin-energy --lib
```

then re-run the generator (it seeds `{}` only when a file is ABSENT, so it can never wipe a
materialized vector), then `bun ./📜️script.ts verify mutation-outcome-law` to confirm energy is still
at zero.
