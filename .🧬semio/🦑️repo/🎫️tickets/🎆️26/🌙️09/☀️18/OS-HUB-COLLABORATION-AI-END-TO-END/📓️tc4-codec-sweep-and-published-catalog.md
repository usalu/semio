# TC4 — the codec sweep green, the native twin closed, the disposer default un-fail-closed, and a PUBLISHED catalog

Slice TC4 of ticket 26/09/18, session 8 (started 2026-09-22 23:1x).
Predecessors: `📓️tc3e-three-package-hub-and-note-creation.md` (§7 gaps a/c/d/e),
`📓️hc1-fresh-component-genesis-and-creation.md` (§0, §6), `📓️tc3d-guest-genesis-and-note-creation.md` §3.

Brief, four items:
1. the three codec-sweep reds (`print-mirror` "lost the minted identity" for note + gis; stdio `pack-schema-hash`)
2. `store::ArtifactCodec::apply_ops_binary` — the native twin's owner-catalogue hole (TC3e §7d)
3. 229 of 298 `s` plugin apps at the fail-closed bounded-store disposer default (TC3e §7a)
4. rebuild + `trusted-catalog publish` into a proper boot root, hub **7683** on it

## 0. Inherited state

Read in full before editing: `📓️worker-preamble.md`, TC3e (all), HC1 §0/§1/§2/§6, TC3d §3, `📓️status.md`
from "### Session 8".

Two things had moved since TC3e wrote its gaps, and both change the slice:

1. **HC1 had already relaxed the print-mirror assertion rather than fixing it.** `git diff HEAD` on
   `🧰️framework/…/🔬️owned-instance-open/🦀️.rs` shows
   `assert!(mirror.dsl.contains(MINTED_DOCUMENT_ID))` replaced by
   `assert!(!mirror.dsl.is_empty() && mirror.dsl.contains(NOTE_DOCUMENT_SCHEMA))`, with a docstring
   arguing the identity is not in the mirror at all. §1 shows it is.
2. **`STAGED_CODEC_COMPONENTS` had been cut from three rows to one** (note only), for two stated
   reasons: the owned interpreter SIGKILLed on the 48 MB gis and 50 MB stdio components, and both
   would fail the close ladder anyway. §1 and §3 remove both reasons; the table is four rows now.

Machine at start: load 37.2 rising to 42.7, 22 cargo processes, 70 GiB free, wasm mutex held by the
peer `play` session since 22:17 with `gj1`/`fl3`/`px1` queued behind it.

## 1. The three codec-sweep reds

HC1 §1 measured, on the rebuilt three-package components:

```
test …owned_codec_answers_every_call_on_every_staged_component ... FAILED <1453.074s>
3 staged components swept, 3 failed:
semio:note: codec.print-mirror(note.document) lost the minted identity
semio:gis: codec.print-mirror(gis.map) lost the minted identity
semio:stdio: codec.pack-schema-hash(stdio.txt): Guest(Fault { … "artifact codec schema has no
  structural record specification" })
```

### 1.1 `print-mirror` "lost the minted identity" — the law was reading the wrong file

`codec.print-mirror` returns a PAIR of files, `(dsl, ops)`
(`🔌️plugin/🧬️schema/📜️.wit:1411`, `GuestDocumentMirror { dsl, ops }`), produced by
`store::print_document_text`:

* `dsl` is `envelope.vcs.initial_snapshot.print_dsl()` — the snapshot's own domain fields. Note's
  snapshot happens to have a field called `id`, which is why the rebuilt component answered
  `semio note.note.dsl v1\nschema=note.document id=empty …`;
* `ops` is `print_ops_log`, whose **very first line** is
  `OpsHeaderLine::Doc { id: envelope.id, schema: envelope.schema }`
  (`🏪️store/🦀️.rs`, `print_ops_log`) — the SERVER-MINTED identity, and the exact field
  `parse_document_text` reads back out on the inverse.

So nothing was ever lost. The law asserted `mirror.dsl.contains(MINTED_DOCUMENT_ID)` against the
file the identity is not in; the relaxation that followed (assert the dsl merely names the kind)
dropped the round-trip property altogether. **Root fix: assert the identity where the identity is.**
Both the single-component law and the sweep now require
`mirror.ops.contains(MINTED_DOCUMENT_ID) && mirror.ops.contains(schema)` plus a nonempty dsl, which
is a STRICTLY STRONGER oracle than either version before it — and it holds for note, gis and stdio
alike, because `print_ops_log` is framework code shared by every kind.

### 1.2 `codec.pack-schema-hash(stdio.txt)` — a real product hole, one line wide

`TxtSnapshot` (`✏️s/🔌️plugins/🗄️stdio/…/🧬️schema/📸️snapshot/🦀️.rs:45`) derives `dsl::DslRecord`,
so it HAS a generated `__dsl_spec()`, and that spec is already published to the DSL registry by
`register_schema_spec("stdio.txt", TxtSnapshot::__dsl_spec)` (`…/🚪️io/🦀️.rs:77`). But its
`ArtifactPack` impl is HAND-ROLLED (the pack is a raw line-joined body in a semio envelope), and a
hand-rolled `ArtifactPack` never picks up the `record_spec` override that `#[derive(DslArtifact)]`
installs. So `record_spec()` fell through to the trait's `None` — the documented "schema-agnostic"
opt-out — and:

* the guest export turns `None` into a fault: `plugin_artifact_pack_schema_hash`,
  `🔌️plugin/🦀️.rs:38052`, `"artifact codec schema has no structural record specification"`;
* the catalog builder refuses the zero fingerprint outright:
  `component_codec_rows`, `🖨️describe/🛂️descriptor-emission/🦀️.rs:507`.

**Fix:** `TxtSnapshot::record_spec()` returns `Some(Self::__dsl_spec())`, the same spec the DSL
registry already serves under the same schema id. The hash fingerprints the SNAPSHOT RECORD's
fields, not the pack container, so the raw body encoding is untouched. This is what makes
`s.stdio.txt` publishable as an open target at all.

### 1.3 The sweep law — three packages again, and affordable

`STAGED_CODEC_COMPONENTS` is four rows now — note under BOTH runtimes, gis and stdio under the JIT:

| row | component | runtime |
|---|---|---|
| `semio:note` / `s.note.note` / `note.document` | `semio_s_plugin_note.wasm` | owned interpreter (the runtime a hub arms) |
| `semio:note` | same | `GuestRuntimes::Wasmtime` |
| `semio:gis` / `s.gis.gismap` / `gis.map` | `semio_s_plugin_gis.wasm` | `GuestRuntimes::Wasmtime` |
| `semio:stdio` / `s.stdio.txt` / `stdio.txt` | `semio_s_plugin_stdio.wasm` | `GuestRuntimes::Wasmtime` |

The two reasons gis and stdio had been dropped are both answered rather than argued away. The owned
interpreter needs ≈ 840 s for ONE `codec.genesis` on the 47 969 539 B gis component (HC1 §1) and
took the whole test process to `SIGKILL` twice (TC3e); the JIT is the standing A/B of the same
exports (`wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter`) and the components are
pure functions of their bytes. A law nobody can afford to run proves nothing. The sweep also now
names its PROFILE (`wasm-release` only): a hub stages the release component, and `plugin_wasm`'s
newest-mtime search would otherwise grade a `wasm-dev` build carrying four times the code.

The second reason — "both would fail the close ladder anyway, neither gis viewer declares a bounded
document-store disposer" — is §3.

**Status: (filling — cargo check pending, machine load 42).**

## 2. The native `ArtifactCodec::apply_ops_binary` twin — TC3e §7d closed

TC3e left this open with the reasoning "that thunk is generic over `P`/`Mutation` with no app type
in scope, so it has no `build_document_store_owners()` to call … a design decision for whoever owns
`ArtifactCodec`". The decision is made here, and it is the only one available: **an owner catalogue
that needs no app is a FRAMEWORK catalogue.**

`apply_ops_binary_impl` (`🏪️store/🦀️.rs`) built its throwaway reduction store with a bare
`ArtifactStore::new(envelope)` and installed nothing, so the close cursor at the end of the thunk
answers `artifact store has no owner-supplied bounded disposer` for the first NONEMPTY batch — the
path stdio and gis take on a hub, because the hub links their Rust codecs and never asks their
components. The empty batch returns before a store exists, which is why every law over this thunk
passed while the real path was dead: exactly the guest twin's story one week earlier.

**Fix.** `🏪️store/🦀️.rs` gains `bounded_artifact_store_owners<P, Mutation>()` — the one-page
retirement catalogue (`BoundedArtifactRetirementFactory` × 3 + the store's own
`ArtifactStoreCursorDisposer`) — and `apply_ops_binary_impl` installs it on its reduction store.
`🔌️plugin`'s `bounded_config_store_owners` (and therefore `bounded_document_store_owners`, which is
that function) now DELEGATES to it instead of carrying a second copy of the same catalogue: the
implementation moved down to `store`, it was not duplicated.

**Law.** `🏪️store/🧪️tests/🔬️unit/🦀️.rs` ·
`document_codec_apply_ops_binary_reduces_a_nonempty_batch_and_closes_its_store` — mirrors TC3e §6c's
guest-side law exactly: genesis pair → `encode_ops_vec` over one `OpBinary::encode_op` →
`(codec.apply_ops_binary)` → `decode_history` asserts the batch landed exactly one edit, with the
empty batch asserted first so the two paths are distinguished.

**Status: (filling — cargo check pending, machine load 42).**

## 3. The fail-closed disposer default across every shipped app

### 3.1 The measurement, reproduced

`✏️s/🔌️plugins/**`, every file with an `impl ArtifactEditor for …` / `impl ArtifactViewer for …`,
counted against each of the FIVE lanes `VcsArtifactApp`'s close ladder drives
(`drive_artifact_owned_disposer`, `🔌️plugin/🦀️.rs:30983-30987`):

| lane | declares a disposer | at the fail-closed default |
|---|---|---|
| `document-store` | 80 | **216** |
| `config-store` | 80 | **216** |
| `draft-store` | 53 | **243** |
| `presence-store` | 80 | **216** |
| `transient-store` | 80 | **216** |

over **296** editor/viewer impls (148 editors, 148 viewers). TC3e reported "229 of 298" for the
document lane alone; the counts differ by how multi-impl files are attributed, and the shape is the
same. **The draft lane is the worst and nobody had named it.**

### 3.2 The fix — one framework default per lane, on all three traits

`drive_artifact_owned_disposer`'s missing-disposer branch is
`interactive-job.close-owned-disposer-missing`, so an app that declares nothing cannot be CLOSED —
and `plugin_artifact_codec_app` constructs EVERY app of a bundle to read its schema and closes each
one it rejects, which is why one undeclared disposer in `🗒️note` killed the first three-package
bootstrap at 04:12:44 on 2026-09-22.

Every one of the five defaults is derivable from the app's own associated types — there is nothing
app-specific in any of them — and the precedent is already in the tree twice: S12's
`bounded_presence_root_retirement_factory` default, and the TEST-ONLY `BoundedViewerFixture`
(`🔌️plugin/🦀️.rs:7984-7999`), which wraps a viewer in exactly these `or_else` defaults. TC4 makes
them the product's defaults:

| lane | default now installed |
|---|---|
| `document-store` | `bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>()` |
| `config-store` | `bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>()` |
| `draft-store` | `bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>()` (`DraftStore` is an alias of `ArtifactStore`, `🏪️store/🦀️.rs:3942`) |
| `presence-store` | `crate::bounded_presence_store_disposer::<Self::Presence, Self::PresenceMutation>()` — **new**, §3.3 |
| `transient-store` | `bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>()` |

The bounds all fit without widening a single associated type, because
`protocol::Mutation<P>: Clone + ToValue + FromValue` already supplies what the bounded helpers ask of
a mutation (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:174`).

Landed on **all three** traits — `ArtifactApp` (the trait the ladder reads), `ArtifactEditor` and
`ArtifactViewer` — because `EditorApp<E>`/`ViewerApp<V>` FORWARD the authoring trait's answer, so a
`None` left on either authoring trait silently takes the framework default back out one layer up.
The same forwarding hole was live for `ArtifactViewer::build_presence_peer_retirement_factory`
(S12 defaulted `ArtifactApp`'s but not the viewer's, and `ViewerApp` forwards `V::`'s `None`) — fixed
in the same edit.

### 3.3 `bounded_presence_store_disposer` — the one lane with no generic form

`no_presence_store_disposer()` was `NoPresence`-typed, so a default could not use it for an app with
a real presence payload. `🔌️plugin/👥️presence/♻️retirement/🦀️.rs` gains the generic twin: the
terminal root a presence close installs is the payload's own `Default` and the emptiness predicate
is equality with it — a closing app holds no live presence by definition. `no_presence_store_disposer`
is that function at `P = NoPresence`.

### 3.4 The law

`🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs` ·
`the_framework_owns_every_bounded_close_lane_an_app_declares_nothing_for`. `Std1AnyEditor` and
`Std1AnyViewer` declare NOTHING about store ownership — they are exactly the shape 216 of the 296
shipped impls had — and the law asserts all ten lanes (five × two adapters) answer `Some` through
`ArtifactApp`, plus both presence-peer factories. It fails by NAMING the fail-closed lanes rather
than on the first one.

The product-level walk is the per-component codec sweep of §1: `plugin_artifact_codec_app`
constructs and closes every app of the bundle it is asked of, so a green sweep over the three staged
components is the standing proof for all of their apps at once.

**Status: (filling — cargo check pending, machine load 42).**

## 4. The published catalog and hub 7683 (filling)

## 5. Honest gaps (filling)

## 6. Files changed (filling)
