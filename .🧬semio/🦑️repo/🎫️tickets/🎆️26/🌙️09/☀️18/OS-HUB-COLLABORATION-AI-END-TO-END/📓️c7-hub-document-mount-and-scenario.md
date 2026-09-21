# C7 — the mounted hub document, then the two-user scenario

Slice C7 (session 7, 2026-09-21). Inherits C6 §3.4: *the artifact bootstrap's cold pair never reaches
`installColdPair`, so `refreshHostView`'s `if (!child || !this.coldApplied) return` fires — no mount,
no `resolveReady()`, the reservation self-retires at 55 s emitting nothing.* C6's exact next step was
"capture the `Welcome` frame's `bootstrap` variant on the document socket". That is done, and the
answer is bigger than the document.

## 0. HANDOFF

| field | value |
|---|---|
| **`Welcome.bootstrap` on the live hub document** | **`None`. Measured, not inferred** — decoded off the wire with the product's own `decodeServerFrame` on a real socket (`🗑️generated/c7-welcome-socket.txt`). The whole frame list for the connection is `["Welcome","Session"]`: no bootstrap chunks, no `RebootstrapRequired`, nothing else. |
| **why it is `None`** | **Not this document's data.** `Bootstrap::ArtifactBootstrap` is never constructed by any hub code path at all — §1.2. The socket's bootstrap plan is computed by the *database* replay (`🛢️db/🔄️sync/🦀️.rs:1293–1381`), whose only outcomes are `None` / `Tail` / `Snapshot`; the artifact canonical pair lives in a different subsystem the socket handler never asks. |
| **consequence** | **No browser client can ever mount a hub document, on any hub, for any document** — genesis-only or edited. The variant that would seed it is dead code. §1.3 |
| second, latent hub defect | the `Welcome`'s `server_frontier.document_id` is the **db composite key** `v1:36:41:<space><doc>`, not the document id. `validateArtifactBootstrapIdentity` (`🏪️store/👷️worker/🟦️.ts:3886`) throws `artifact bootstrap document mismatch` on exactly that, so an `ArtifactBootstrap` welcome would still be refused until the frontier is projected back to the scope's document id. §1.4 |
| scenario steps green | (filling) |
| gate wired | (filling) |
| infra reused, untouched | hub **7621** pid 48044 (`jc1-boot`, 03:42 `target-jc1` binary), serve `s` **6190**, serve `gis2d` **6191** pid 47392 |

## 1. The `Welcome` frame's `bootstrap` variant — decoded

### 1.1 The measurement

`🐍️c7-welcome-socket-probe.ts` (new, permanent) reaches the same hub handler the browser reaches —
`POST /auth/sessions` → `POST …/open-plan` → `POST …/socket-grants` → `GET …/socket/v1?surface=…`
upgraded with `Sec-WebSocket-Protocol: semio.socket.v1, <grant>` → `SocketHelloV1` — with **no
browser, no plugin and no 63 MB actor in the way**, and decodes every server frame with the
product's own `decodeServerFrame` (`📡️replication/🟦️.ts`).

C7's first attempt was the five-line CDP listener C6 proposed
(`🐍️c7-welcome-frame-probe.mjs`, kept). It does not work and should not be retried: the document
socket is opened by the **store worker**, so the page's CDP session never sees its frames, and
`Target.setAutoAttach` cannot hand Playwright a usable child session — with
`waitForDebuggerOnStart` the workers stay paused (that run's shell took 240 s to boot and never
signed in, `🗑️generated/c7-welcome-probe.txt`). Going straight at the hub is both cheaper and
stronger: it removes every client-side variable from the reading.

```
   3931 OPEN-PLAN schema=gis.map checkpoint={"checkpointId":"b2b7430b2f5ee17e195ceb956e1f1e42e2712828780bd0c0e7d8daa809123f1a",
          "descriptorDigestV1":"33f7b7df4064c83513e35810f0e9215c206f6426685c2e82e4da773a366226d4",
          "baselineFrontier":{"documentId":"artifact-0954e2d10d8fff9605f101b0dba34f3b","headEditOrdinal":0,"headEditId":"",
          "lastCommitSeq":0,"chainHash":[0,…,0]},"aggregateSha256":"de1cea598d67277b65f96f44296c50f2b151df377c692c9d9d49d540bc04f6e4"}
   3994 HELLO actor=hub.v1.850566c2702de962d59f20f5b5474055af33774cf48d22648d547ce58e827000
   4037 FRAME Welcome bootstrap=None 324B
   4042 FRAME Session 75B
  18996 WS-CLOSE code=1000
WELCOME.bootstrap None
SERVER-FRONTIER {"document_id":"v1:36:41:01a0c314-e41f-780d-a980-3adda40ca9f7artifact-0954e2d10d8fff9605f101b0dba34f3b",
                 "head_edit_ordinal":0,"head_edit_id":"","last_commit_seq":0}
FRAME-KINDS ["Welcome","Session"]
```

Two things in that capture matter beyond the tag. The **open plan carries a real, published
checkpoint** — a checkpoint id, a descriptor digest and an aggregate hash — so the hub *does* hold a
canonical artifact pair for this document; it simply never offers it on the socket. And the welcome
is 324 bytes: there is no truncation, no late frame, no race with the 8-second actor activation.
C6's "the hub sent `None`" reading was right; its caveat ("a genesis bootstrap of two CAS chunks
could also complete between two polls") is now closed.

### 1.2 Why — the variant is dead code, hub-wide

The socket's welcome is built by `state.db.hello(...)` (`🌎️hub/🏗️bootstrap/🦀️.rs:4993`) →
`take_welcome()` (`:4999`). That is the **database** sync hello
(`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:1293–1381`), and its bootstrap plan has
exactly three outcomes:

| condition | variant sent |
|---|---|
| replica is at or past the retained floor and no commands are missing | `Bootstrap::None` ← **this document** |
| replica is at or past the floor and commands are missing | `Bootstrap::Tail` |
| replica is behind the floor | `Bootstrap::Snapshot` |

None of the three is an artifact bootstrap, and the third is *refused by the client on purpose*
(`🏪️store/👷️worker/🟦️.ts:4074`, "database-private snapshot cannot seed an artifact client").

The fourth variant, `Bootstrap::ArtifactBootstrap` (`📡️replication/📡️wire/🦀️.rs:62`), is produced by
exactly one function in the repository — `VerifiedRebootstrapSource::load`
(`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:223`) — and **that function has no caller outside its own unit
test**. The hub's two live uses of `VerifiedRebootstrapSource` are `active_pair` (the HTTP route
`GET /spaces/{s}/documents/{d}/active-checkpoint/pair`, `🏗️bootstrap/🦀️.rs:3673`) and `control`
(the `RebootstrapRequired` notice, `:4663`). Grep-proof, non-test, whole repo:

- `ServerFrame::ArtifactBootstrapChunk` / `ArtifactBootstrapDone` — **zero senders** anywhere in `🌎️hub/`;
- `rebootstrap.load(` — **zero call sites**;
- `Bootstrap::ArtifactBootstrap` — constructed only inside `load`, matched only by the wire codec and by the two clients (`🏪️store/🔄️sync/🦀️.rs:2555, 3742` and the TS worker).

### 1.3 What that means

**No hub can seed a cold artifact client over its document socket.** This is not a property of
`artifact-0954e2…`, of C5's HTTP provisioning, or of a genesis-only ledger; a document with a
hundred edits would be answered with `Tail` and would be just as unmountable, because `Tail` carries
mutation envelopes to apply *on top of a pack the client does not have*. The client is complete on
its side: `startArtifactBootstrap` → `ArtifactBootstrapAssembler` → `installArtifactBootstrap` →
`VerifiedColdDocumentPair` → `reservation.installColdPair` → `refreshHostView` → `publishMountedIfReady`
is written, typed and reachable — it is simply never entered, because the frame that enters it is
never sent.

The already-written lag path proves the hole rather than filling it: when the broadcast lags, the hub
sends `RebootstrapRequired` and closes (`🏗️bootstrap/🦀️.rs:5304`); the client sets
`artifactRebootstrapRequired` and reconnects expecting an `ArtifactBootstrap`; the hub answers `None`
again; and the client's own code then raises **`artifact rebootstrap returned no canonical pair`**
(`🏪️store/👷️worker/🟦️.ts:4050`). The hub asks for a rebootstrap it cannot serve.

### 1.4 The second defect on the same frame

`validateArtifactBootstrapIdentity` (`🏪️store/👷️worker/🟦️.ts:3884`) refuses a bootstrap whose
welcome frontier is not the document's:

```ts
if (bootstrap.baseline_frontier.document_id !== state.config.documentId ||
    bootstrap.required_tail_frontier.document_id !== state.config.documentId ||
    serverFrontier.document_id !== state.config.documentId)
  throw new Error("artifact bootstrap document mismatch");
```

The measured `server_frontier.document_id` is `v1:36:41:01a0c314-…artifact-0954e2…` — the hub's
internal db key (`db_artifact_id`, `🏗️bootstrap/🦀️.rs:664`), which the db engine stamps into every
frontier it returns and which `engine_frontier_to_wire` (`:4699`) passes through unchanged to `Ack`
as well. It is invisible today only because the `None` branch assigns the frontier without
validating it. Any fix that starts sending `ArtifactBootstrap` must project the frontier's document
id back to the scope's document id in the same change, or the client will refuse the pair it finally
receives.

## 2. The mount fix — five root defects, each measured by the next one appearing

The mount is a chain. Every hop below was dark before this slice; each fix made the NEXT refusal
visible, and each refusal is a real product defect, not a configuration mistake. All five are fixed
at the root. The probe tag is the capture in `🗑️generated/`.

| # | hop | what refused, verbatim | root cause | fix (file:line) | proven by |
|---|---|---|---|---|---|
| 1 | seeding | *nothing at all* — the mount silently never happened, the reservation self-retired at 55 s | the hub never sends a canonical pair on the socket (§1.2), and the browser client had no other source, although the hub's own `active-checkpoint/pair` route serves one (200, 81 873 B, measured) | `🏪️store/👷️worker/🟦️.ts` `seedColdPairFromCanonicalCheckpoint`; decoder `📇️directory/🧬️schema/🟦️.ts` `decodeCanonicalCheckpointPairV1` | `c7a`: stage `canonical-pair 81873/81873` |
| 2 | identity | `artifact bootstrap pack schema mismatch` | `validateArtifactBootstrapIdentity` REQUIRED `ArtifactActorConfig.packSchemaHash`, which only the wasm renderer's `document_pack_schema_hash` export supplies — **no React host has ever had one**, so a React host could never accept an artifact bootstrap, socket-delivered or not. The authority is the verified open plan mirrored onto the lease, and `documentOpenPlanAuthority` already treats the config value as an optional cross-check | `🏪️store/👷️worker/🟦️.ts:3891` | `c7b` got past it |
| 3 | ordering | `artifact bootstrap missing browser actor reservation` | `installArtifactBootstrap` hard-requires `state.browserActorReservation`, which is created only when the hub's **`Session`** frame arrives — and the hub sends `Session` strictly AFTER `Welcome` (its own contract §C7.3). A socket-delivered `ArtifactBootstrap` would have hit this too: the path was never once executed | seed moved into `activateDocumentBrowserActorAfterSession`, between `reserveDocumentBrowserActorChild` and `owner.activate(socket)` (`🏪️store/👷️worker/🟦️.ts:2617`) | `c7c`: the bootstrap installed, 81 038 B, and the actor loaded and activated on top of it |
| 4 | diagnosis | `document browser actor: invalid page receipt` | the cold-transfer loop refused a page receipt without saying what it got — `fault`, `backpressure`, `loading` and a wrong cursor are four different problems with one message | `coldPairIngressStatusDiagnostic`, `🏪️store/👷️worker/🟦️.ts:2291` | `c7d`: `invalid page receipt (page 1/2 answered fault page 1/2: cold-pair.frontier)` |
| 5 | **the guest** | guest fault `cold-pair.frontier` | `ColdDocumentPairHeader::validate` (`🎠️kernel/📥️cold-pair/🦀️.rs`) and `ColdDocumentPairFrontier::validate` (`🎭️actor/📥️cold-pair/🦀️.rs:56`) demand a **nonempty `head_edit_id` and a nonzero chain hash** — i.e. they refuse the GENESIS frontier that the hub's artifact authority publishes for every newly created document, and that the product's own `artifactFrontierIsGenesisForV1` defines as exact. **A freshly created hub document could never be mounted, by construction.** The host's TS twin `parseColdDocumentPairFrontier` refuses the same shape, so the guest's applied receipt would have failed identically one hop later | `🎭️actor/📥️cold-pair/🦀️.rs` — new `is_genesis`/`is_edited`, `validate` accepts either; `🎠️kernel/📥️cold-pair/🦀️.rs` now delegates to that ONE predicate instead of its own weaker copy; `🎭️actor/📥️cold-pair/🟦️.ts` host twin mirrored | `cargo check -p semio-framework-actor -p semio-framework` clean; **runtime proof needs the gis2d wasm actor rebuilt** — §3 |

Defect 5 is guest Rust compiled into the 63 MB browser actor, so it only takes effect after a
`wasm32` rebuild and re-activation of the gis2d component.

## 3. The ten-step scenario

(filling)

## 4. Files changed

Product source:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` — `decodeCanonicalCheckpointPairV1` + the pair media type/limits (new region), `canonical-pair` progress stage
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` — `seedColdPairFromCanonicalCheckpoint`, pack-schema authority (defect 2), seed ordering (defect 3), `coldPairIngressStatusDiagnostic` (defect 4)
- `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🦀️.rs` — `is_genesis`/`is_edited`, `validate` (defect 5)
- `🧰️framework/🔨️modules/🎠️kernel/📥️cold-pair/🦀️.rs` — delegates to the one frontier predicate (defect 5)
- `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts` — host twin of the same predicate (defect 5)

Ticket folder:
- `🐍️c7-welcome-socket-probe.ts` — new, permanent: decodes the hub's `Welcome` on a real document socket, with no browser
- `🐍️c7-welcome-frame-probe.mjs` — new: the CDP route C6 proposed, kept as the record that it cannot work (§1.1)
- `📓️c7-hub-document-mount-and-scenario.md` — this report

`tsc -p 🧰️framework/🛍️products/💻️os/tsconfig.json --noEmit`: **63 errors, none in any file this slice
changed** — byte-identical to C6's baseline count.
`cargo check -p semio-framework-actor -p semio-framework` (private `target-c7`): **clean**, warnings
pre-existing and in other crates.

Laws added to the store worker's vitest corpus
(`🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`, `describe("canonical
checkpoint pair")`), **3 passed**:

1. *decodes the hub's own framing and accepts the genesis baseline a new document's first checkpoint
   carries* — a body built byte-for-byte the way `append_canonical_pair_{header,data,terminal}`
   builds one, for both a genesis and an edited baseline;
2. *names every refusal and never half-accepts a body* — truncation, a non-`Complete` terminal, an
   ordinal-0 frontier with a positive head, an edited frontier with a zero chain hash, and a record
   whose part byte breaks the pack-then-SPR order;
3. *accepts a genesis applied receipt from the guest, which the host twin used to refuse* — the TS
   half of defect 5.

Suite state (`bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts`): **359 passed, 4 failed**. The
four are in `backbone-worker offline resilience` and are **pre-existing** — running the same pattern
with this slice's seed call disabled and re-enabled gives the identical `3 failed | 28 passed`, so
they are not this slice's (`🗑️generated/` captures of both runs).

## 5. Honest gaps

(filling)
