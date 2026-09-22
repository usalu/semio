# WI1 — a user runs an inference on an artifact (wfc, end to end)

Slice owner: WI1, session 8 (2026-09-22, 17:2x → ). Spec: `📓️pz2-puzzle-describe-under-budget.md`
§5.2 (the design gap), `📓️jb1-builtin-jobs-in-production.md` (the bounded `semio.infer` machine),
`📓️ce3-four-mcp-gates-green.md` §1/§3.3, `📓️g19-ai-user-experience-audit.md` §1c.

Target: `inference_run`/`inference_submit` → progress → result, live, for a wasm guest inference;
`client-e2e` **37/37**; one live run from the `s` shell captured.

📏️ The denominator moved for a reason: PZ2's 37 was 36 rows + the ONE row the `else` branch
announces when `inference_run` mints no `jobId`. A run that mints one announces TWO (`job_get`,
`job_cancel`), so the gate is 38 rows now. **37/38** is therefore one red, not two — and it is not
the same red PZ2 left.

## 0. Headline

1. **The 240 s silence is gone and the payload contract is published.** `inference_run` on
   `s.wfc.bitmap.solve` answered nothing inside PZ2's 240 s budget; it now answers in **1–17 s**,
   and when it refuses it refuses BY FIELD (`INPUT_INVALID`, `details.field`). All five `🀄️wfc`
   inferences publish a contract in their committed descriptor — verified on disk after the
   describe: `payloadSchemaId`, a 1 107–1 110 B request JSON Schema, `progressUnit`, and
   `artifactBinding { document, artifact-pack-base64, required }`.
2. **`InferCommand` carries the artifact.** `inference_run { artifactKind, inferenceSchema,
   artifactId }` now binds that artifact's canonical `pack`/`spr` into the payload field the
   PLUGIN declared, and the guest decodes it into its own snapshot through one shared path. An
   agent no longer has to type 4 096 bitmap cells — which it could not.
3. **`client-e2e` 35/37 → 37/38** (`wi1-client-e2e-final.txt`, 21:3x, on this slice's own staged
   `semio-os-mcp`, serve 6200 up). Green now: the roster row WITH its published contract,
   `notifications/progress` for `_meta.progressToken`, `job_get`, `job_cancel`,
   `notifications/cancelled`. **Not 38/38**: `inference_run` itself is red.
4. **The one red is a guest-runtime trap, and it is not the contract.** With the artifact bound the
   guest traps inside the pack inflater; with a HAND-WRITTEN `payload.snapshot` that never touches
   the new carrier it traps in `alloc::Global::deallocate` while dropping a
   `wit_bindgen::rt::async_support::FutureState`. Same lane, two faces, and PZ2's "never answers"
   was its earlier one. §7.2 has the evidence that it is not this slice's code.
5. **The gis route is untouched**: `inference::` laws 51/51 (including
   `inference_run_dispatches_the_gis_map_oracle_through_the_infer_command`), and
   `inference-bridge-check --source` green (`ajv=14 hostile=29 errors=11 visibility=7 lifecycle=9
   routes=6 limits=4`).

## 1. Inherited state — what was measured before this slice

| source | measurement |
| --- | --- |
| `📓️ce1-…md` §3b | `inference_run` on `wfc`/`s.wfc.bitmap.solve`: **no answer within 900 000 ms** cold, then `wasmtime: failed to convert function to given type` in seconds warm — a component-AGE fault |
| `📓️jb1-…md` | every builtin `JobFn` was dropped under `#[cfg(not(test))]`; the bounded `InteractiveInferenceJob` replaced it (39/39 laws) — `semio.infer` is admitted in a shipped component since then |
| `📓️ce3-…md` §1/§3.3 | `source_dialect` fixed (capability-id grammar → identity); the refusal then became `job.infer.dispatch: tool factory 'semio.infer' rejected 's.wfc.bitmap.solve': bitmap-inference-wire-decode:missing field 'snapshot'`, and "no payload contract is published anywhere an agent can read it" |
| `📓️pz2-…md` §5 | `client-e2e` **35/37** at load 23; `inference_run` no longer refuses — it **does not answer inside 240 s**; §5.2: `InferCommand` carries no artifact binding, `ArtifactInferenceDescriptor` has no field for a contract |
| `📓️g19-…md` §1c | real inferences: `gis` ×1 (native, the only end-to-end one), `🀄️wfc` ×5, `cad-extension-aec-building` ×1 (not in the roster), `stdio` ×68 (no committed descriptor) |

Measured by this slice at 17:5x, before any edit landed:

```
staged wasm-dev/semio_s_plugin_wfc.wasm  129 198 650 B  sha256 5981dfb5eb73b2f02d9395e0…
committed 🔣️.json  hashes.wasmSha256 = 5981dfb5eb73b2f02d9395e0…   (component and descriptor agree)
contributions.inferenceServices = 5 rows, `payload` present on 0 of them
```

So the freshness gate was green and the contract was the whole gap — exactly PZ2 §5.2's reading.

## 2. The design — payload contract, schema-first

Two declarations, one direction of travel: the PLUGIN says what its inference takes, the gateway
obeys that declaration, and neither side guesses.

### 2.1 The published contract

`semio_framework::InferencePayloadContract` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`, new):

| field | what it carries |
| --- | --- |
| `payloadSchemaId` | the id the guest's own `ToolJobFactory::payload_schema_id()` already returned and nothing published (`BITMAP_INFERENCE_PAYLOAD_SCHEMA` = `s.wfc.bitmap.inference.request.v1`) |
| `inputSchema` | the REQUEST JSON Schema, as text — the half that did not exist: the facet leaf `🔣️.json` beside each wfc inference is the RESULT schema |
| `outputSchema` | that result schema, carried verbatim (`include_str!("🔣️.json")`) |
| `progressUnit` | what the bounded job counts (`cells` / `slots`) |
| `artifactBinding` | `{ field, encoding, required }` — see 2.2 |

It rides on `ContributedInferenceMetadata.payload` (`#[serde(default, skip_serializing_if =
"Option::is_none")]`, so every descriptor committed before this slice still decodes, and a plugin
that declares no contract publishes no field at all). Chain: a plugin's `const fn
<kind>_inference_metadata()` → `ArtifactInferenceServiceMetadata.payload` → the guest's
`WireArtifactInferenceMetadata` roster → `describe`'s `plugin_inference_services` →
`contributions.inferenceServices[].payload` in the committed `🔣️.json` → `DeclaredInference.payload`
→ `inference_list` / `capabilities_describe`.

### 2.2 The artifact binding — why `InferCommand` needed one

PZ2 §5.2: `InferCommand` carried no artifact at all, so the guest had no document to read a
snapshot from and a generic client had no way to synthesise 4 096 bitmap cells. `InferCommand` now
carries `artifact_id` plus `artifact_document: Option<ArtifactDocumentBinding { pack, spr }>`, and
the contract says where it goes:

```
artifactBinding { field: "document", encoding: "artifact-pack-base64", required: true }
```

`bind_inference_document` (`🌉️mcp/🏠️workspace/🦀️.rs`) writes EXACTLY that field, in EXACTLY that
encoding, and refuses by name otherwise — an encoding it has no writer for, a payload that is not
an object, or a required binding with no artifact named AND an empty body. A caller who authored
the field themselves keeps it: the binding fills a gap, it never overwrites.

`required` means "this inference needs SOME body and a caller cannot type one", not "this field or
nothing": `🀄️wfc`'s own request schema is `oneOf [document, snapshot]`, and a caller who states a
`snapshot` has already answered the requirement. Refusing an authored body was a real defect this
slice shipped and then measured out (21:2x, `wi1-snapshot-payload-probe.txt` first run) — the
gateway now refuses only an EMPTY body, and the guest, which owns the schema, judges the rest.

The guest half is one shared decode: `ArtifactDocumentPayload::settled_snapshot::<P, Mutation>()`
(plugin SDK) → `artifact_pair_snapshot` → `store::parse_document_pack(...).into_snapshot()`.
`into_snapshot` is new and is the mirror of the existing `into_envelope`: it takes the LIVE replayed
projection and retires the history beside it through the same bounded path a refused parse uses —
letting the envelope drop aborts the process, which is why a reader could not do this before.

### 2.3 `INPUT_INVALID`, with the field, before any guest runs

`validate_inference_request` (`🌉️mcp/💡️inference/🦀️.rs`) runs before routing and answers
`INPUT_INVALID` with `details.field`:

| case | field named |
| --- | --- |
| artifact-bound inference, no `artifactId`, no `payload` | `artifactId` |
| caller payload missing a `required` name from the contract's `inputSchema` | that field |
| no published contract AND no payload | `payload` (names the plugin that must declare one) |
| `artifactId` given for an inference that publishes no binding | `artifactId` |

Only the contract's TOP-LEVEL `required` list is enforced host-side; the guest that owns the schema
stays the authority on the rest. That is the difference between "answers in milliseconds naming the
field" and PZ2's 240 s of silence.

## 3. Implementation

| # | file | change |
| --- | --- | --- |
| 1 | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `InferencePayloadContract`, `InferenceArtifactBinding`, `INFERENCE_ARTIFACT_PACK_BASE64`; `ContributedInferenceMetadata.payload` (optional, skipped when absent) |
| 2 | `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs` | TS projection rows for both new types + `payload?` on `ContributedInferenceMetadata` |
| 3 | `🔌️plugin/🦀️.rs` | `ArtifactInferencePayloadContract` / `ArtifactInferenceDocumentBinding` (`&'static str`, `Copy`, const-authorable) on `ArtifactInferenceServiceMetadata.payload`; wire twins `WireInferencePayloadContract`/`WireInferenceArtifactBinding` on `WireArtifactInferenceMetadata.payload`; `ArtifactDocumentPayload` (`from_pair`/`pair`/`snapshot`/`settled_snapshot`); `artifact_pair_snapshot`; `validate_wire_request_metadata` now compares the IDENTITY projection (a request names an inference, it never re-states the contract) |
| 4 | `🔌️plugin/🛂️describe/🦀️.rs` | `plugin_inference_services` carries `payload` into the committed descriptor |
| 5 | `🏪️store/🦀️.rs` | `ParsedDocumentText::into_snapshot()` — the mirror of `into_envelope()`; a reader keeps the live projection and the history is retired through `discard_parsed_envelope` instead of aborting the process on drop |
| 6 | `🌉️mcp/🔀️dispatch/🦀️.rs` | `InferCommand.artifact_id` + `.artifact_document: Option<ArtifactDocumentBinding{pack,spr}>` |
| 7 | `🌉️mcp/🏠️workspace/🦀️.rs` | `bind_inference_document` — writes the declared field in the declared encoding, refuses everything else by name; `infer_real` builds its canonical payload through it |
| 8 | `🌉️mcp/💡️inference/🦀️.rs` | `DeclaredInference.payload` (published by `inference_list`); `validate_inference_request` + `contract_required_fields` + `inference_field_invalid` + `inference_error_field`; `resolve_inference_artifact_document` reads the named artifact's pair with `read_artifact_bytes`; the handler mints the job BEFORE validating; `execution_not_wired_error` (what `inference_get` answers) now carries the contract and names `inference_run` + the bound field |
| 8b | `🔌️plugin/🖥️host/🦀️.rs` | `GuestArtifactInferenceMetadata.payload` — the router decodes the committed roster, so it must accept the field the descriptor now carries (§5.1) |
| 9 | `✏️s/🔌️plugins/🀄️wfc/…/💡️inferences/🦀️.rs` ×5 | `<KIND>_INFERENCE_REQUEST_SCHEMA` (published request JSON Schema) + `<KIND>_INFERENCE_CONTRACT` + `payload: Some(...)` on each `<kind>_inference_metadata()`; `<P>InferenceRequest { snapshot: Option, document: Option<ArtifactDocumentPayload>, checkpoint }` + `resolve_snapshot()` — ONE shared decode for all five kinds |
| 10 | `🌉️mcp/🟦️.ts` (`client-e2e`) | the journey creates an `s.wfc.bitmap` artifact and runs the inference ON it; the `inference_list` row now also requires a PUBLISHED contract |
| 11 | `🌍️gis`, `🗄️stdio`, `📐️cad` extension + 6 framework fixtures | `payload: None` — the 22 `ArtifactInferenceServiceMetadata` literals in the tree; no behaviour change |

### 3.1 The job is minted before the contract is checked

Measured, not assumed. With validation first (20:56 run, `wi1-client-e2e-before.txt`) `inference_run` refused in milliseconds — the 240 s hang was already gone — but the refusal minted **no job**, so `notifications/progress` carried nothing and `job_get` had no id: **33/37**. Moving `JobRegistry::begin` above the validation (it is what binds the id to this call's `_meta.progressToken`, `📣️notify/🦀️.rs:358`) and failing the job with the typed error turned three rows green in one edit: **36/38** (`wi1-client-e2e-2.txt`, 20:59). A refused inference now reports progress, carries its `jobId` in `details`, and is readable with `job_get`/`job_cancel` — which is exactly when a client needs those two most.

## 4. Laws

`cargo test -p semio-framework-os-mcp --lib` in `⚡️cache/cargo/target-wi1`, `CARGO_INCREMENTAL=0`
(`🗑️generated/wi1-test-mcp-laws.txt`, `wi1-test-mcp-inference.txt`):

| law | what it pins | file |
| --- | --- | --- |
| `an_artifact_bound_inference_without_an_artifact_is_input_invalid_naming_artifact_id` | the PZ2 §5.2 refusal: `INPUT_INVALID`, `details.field == "artifactId"`, message names `payload.document` | `💡️inference/🧪️tests/🔬️quick` |
| `an_artifact_bound_inference_with_an_artifact_is_admitted` | …and the same call WITH an artifact passes — a gate, not a wall | ″ |
| `a_payload_missing_a_contract_required_field_names_that_field` | a caller payload is checked against the contract's own `required` list | ″ |
| `an_unpublished_contract_refuses_by_naming_payload_and_its_owner` | no contract + no payload → names `payload` and the plugin that must declare one | ″ |
| `a_committed_contract_reaches_the_published_roster_row` | descriptor → `DeclaredInference` → the JSON `inference_list` publishes (`payloadSchemaId`, `artifactBinding.field`, `progressUnit`) | ″ |
| `a_named_artifact_is_bound_under_the_field_the_contract_declares` | the pair lands under `document`, base64, and what the host wrote is what a guest decodes back | `🏠️workspace/🧪️tests/🔬️quick` |
| `a_caller_authored_binding_field_is_never_overwritten` | the binding fills a gap, never overwrites | ″ |
| `a_required_binding_without_an_artifact_refuses_by_name` | host-side twin of the gateway refusal | ″ |
| `an_unknown_binding_encoding_is_refused_rather_than_guessed` | an encoding with no writer is named, never written in the one shape this host knows | ″ |
| `an_inference_without_a_published_contract_passes_its_payload_through` | host-opaque behaviour is preserved for every plugin that publishes nothing | ″ |
| `a_bound_document_round_trips_through_the_guest_json_decoder` | 405 B pack + 211 B spr survive `serde_json` → the GUEST's own DSL JSON decoder → base64, byte for byte — the host↔guest carrier, proven without a component (§7.2) | ″ |
| `a_read_of_an_artifact_bound_inference_answers_with_its_contract` | `inference_get`'s refusal is now an INSTRUCTION: it names `inference_run`, names the bound field, and carries the whole contract in `details.payload` | `💡️inference/🧪️tests/🔬️quick` |
| `a_read_of_an_unbound_inference_keeps_its_plain_refusal` | …and nothing is claimed for a plugin that declared nothing | ″ |

**Final run 21:4x: `inference::` filter 51 passed / 0 failed; the six binding laws 6 passed / 0
failed** (`wi1-test-mcp-inference-final.txt`, `wi1-test-mcp-binding-final.txt`). The pre-existing
suite under the `inference::` filter is green — `inference_run_dispatches_the_gis_map_oracle_through_the_infer_command`
and `…a_previously_not_wired_service_identically` included, so the gis route is unchanged.

Three more laws are authored in `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/…/💡️inferences/🧪️tests/🔬️unit/🦀️.rs`
(`the_published_contract_binds_the_field_the_request_decodes`,
`a_request_with_neither_carrier_is_refused_by_name`, `a_stated_snapshot_is_resolved_unchanged`).
They are **not yet run**: a native `cargo check` of the wfc artifact crates pulls
`semio-framework-artifact-flow-flow`, which a peer had red at 17:51 and 20:5x
(`owner_backing_payload` / `FlowRetirement::root_backing_credit`, `🌊️flow/🧵️retained/🦀️.rs:530-615`),
and `🌊️flow` is out of this slice's scope. They compile under `wasm32-wasip2` inside the mutex hold
(§5), where `cargo check --target wasm32-wasip2` does not link the native flow crate.

## 5. Component rebuild + describe (ordered wasm mutex)

ONE hold, ticket stamp `20260922172000`, slice `wi1`, script
`📜️wi1-wfc-check-and-describe.sh` — it type-checks the whole guest for `wasm32-wasip2` FIRST and
only describes if that is green, so a compile break never costs a describe-length hold.

| attempt | what happened |
| --- | --- |
| 17:52 → 17:56 (pid 68856) | reached the lock, then `error: none of the selected packages contains this feature: semio-s-plugin-wfc/component-app-assembly` — the feature lives on the five ARTIFACT crates, not on the plugin crate. `DESCRIBE SKIPPED`, rc 101, hold released in 1 s. The fleet was cut at ~18:00 while the fix was being written |
| 20:52 re-queued (pid 96769), lock at **20:58:42** | `cargo check --target wasm32-wasip2 -p semio-s-plugin-wfc -p semio-s-artifact-wfc-{bitmap,2d,3d,grid2d,grid3d}` → **exit 0 in 1 m 07 s** (`wi1-wasm-check.txt`). This is also the only way this slice's wfc code can be type-checked at all: a NATIVE check links `semio-framework-artifact-flow-flow`, which a peer had red both times it was tried (§4) |

**The describe, 20:59:50 → 21:04:45, rc 0 in 293 s** (`wi1-describe-ledger.txt`):

| | before | after |
| --- | ---: | ---: |
| `🔣️.json` | 1 033 420 B | **1 046 722 B** |
| `🛂️.descriptor.semio` | 255 125 B | **266 391 B** |
| staged `wasm-dev/semio_s_plugin_wfc.wasm` | 129 198 650 B | **129 379 508 B** |
| `sha256` | `5981dfb5eb73b2f0…` | **`90db1834dba54fa2…`** |

Guest execute: **152 939 ms, 488 131 106 fuel** — a third of `🧩️puzzle`'s 8 G budget, no cliff.
`core_wasm_sha256 = 88ffe1859dc6753c…`. The freshness step of every run after this reads
`wfc: … 129 379 508 B, sha256 90db1834dba5… matches its committed descriptor`.

Committed result, read straight off `✏️s/🔌️plugins/🀄️wfc/🔣️.json`:

```
s.wfc.bitmap.solve -> s.wfc.bitmap.inference.request.v1 | binds document / artifact-pack-base64 | unit cells  | 1110 B input schema
s.wfc.grid2d.solve -> s.wfc.grid2d.inference.request.v1 | binds document / artifact-pack-base64 | unit cells  | 1110 B
s.wfc.grid3d.solve -> s.wfc.grid3d.inference.request.v1 | binds document / artifact-pack-base64 | unit cells  | 1110 B
s.wfc.wfc2d.solve  -> s.wfc.wfc2d.inference.request.v1  | binds document / artifact-pack-base64 | unit slots  | 1107 B
s.wfc.wfc3d.solve  -> s.wfc.wfc3d.inference.request.v1  | binds document / artifact-pack-base64 | unit slots  | 1107 B
```

### 5.1 One more host fix the committed contract forced

The first run after the describe was **37/38 with `INTERNAL: inference route registration: json:
0.unknown field 'payload'`**. `ArtifactInferenceRouter::register_plugin` decodes the roster —
which IS `contributions.inferenceServices` — into `GuestArtifactInferenceMetadata`, a
`deny_unknown_fields` struct in `🔌️plugin/🖥️host/🦀️.rs`. A row it cannot decode is a row it cannot
route. The field is now carried there too (additive + defaulted, so a pre-contract descriptor still
decodes), and `InferencePayloadContract`/`InferenceArtifactBinding` gained `Eq` for that struct's
own derive.

## 6. `client-e2e`

Every run below is on THIS slice's own staged `semio-os-mcp`
(`🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp`, rebuilt by this slice at 20:55 and 20:58),
folder mode, with serve **6200** up (`📜️wi1-serve.sh s 6200`, pid 97157, `/` → 200) and hub 7621 /
7651 untouched.

| run | tally | what moved |
| --- | ---: | --- |
| `📓️pz2-…md` §5.1 (inherited, 2026-09-22 ~17:10) | **35 / 37** | `inference_run` did not answer inside 240 s; no `jobId`, so the cancel row never ran |
| `wi1-client-e2e-before.txt` (20:56) | **33 / 37** | the 240 s hang is GONE — `inference_run` answers in milliseconds with `INPUT_INVALID … details.field = "payload"`. But validation ran BEFORE the job was minted, so `notifications/progress` and `job_get` had nothing |
| `wi1-client-e2e-2.txt` (20:59) | **36 / 38** | §3.1's re-ordering: progress, `job_get` and `job_cancel` all green on a REFUSED inference. Two reds left, both the same fact |
| `wi1-client-e2e-3.txt` (21:07, after the describe) | **37 / 38** | the contract row went green; the new red was `INTERNAL: inference route registration: json: 0.unknown field 'payload'` — §5.1 |
| `wi1-client-e2e-final.txt` (21:3x) | **37 / 38** | roster, contract, progress, `job_get`, `job_cancel`, `notifications/cancelled` all green; the single red is the guest trap (§7.2) |

The two remaining reds at 20:59, verbatim:

```
FAIL  os: inference_list declares the pinned service — 74 declared inference(s), including
      s.wfc.bitmap/s.wfc.bitmap.solve by wfc, publishing NO payload contract — a client cannot
      know what to send
FAIL  os: inference_run — {"code":"INPUT_INVALID","details":{"artifactId":"mcp-client-e2e-…-inference",
      "artifactKind":"s.wfc.bitmap","cancellationId":"…-cancel","field":"payload",
      "inferenceSchema":"s.wfc.bitmap.solve","jobId":"job_f2682c94de4df8f55b982891e3","pluginId":"wfc"},
      "message":"`s.wfc.bitmap/s.wfc.bitmap.solve` publishes no payload contract …"}
```

Both say one thing: the CONTRACT is authored in source (§3 row 9) and is not yet in `🀄️wfc`'s
committed `🔣️.json`, because that needs the describe in §5. Two things worth naming from these runs
even before it lands:

- **`artifact_create` of `s.wfc.bitmap` succeeds**, inside the journey's own budget — the artifact
  the inference runs on is real and its id reaches the refusal's `details.artifactId`.
- **`inference_list` now declares 74 services**, not the 6 CE1 measured (`gis ×1 + wfc ×5`): M10's
  `artifact_contributions` walk landed in between, so `cad-extension-aec-building` and the rest are
  in the roster. That is not this slice's work; it is the denominator this slice's contract check
  now runs against.

## 7. Live proof in the `s` shell — and where the last red really is

### 7.1 What the live shell proved

Serve **6200** is this slice's own (`📜️wi1-serve.sh s 6200`, pid 97157, `/` → 200, serve-only on
the staged tree — no re-activation). `🐍️wi1-shell-inference-probe.ts` drives the real `.mcp.json`
`semio` server against it with a headless browser on the page, and the gateway attached to the LIVE
shell rather than the folder:

```
context_resolve :: channel="shell" principal=agent:local locale=en
                   scopes=[registry.query, artifacts.read, artifacts.write, jobs.spawn,
                           shell.observe, shell.control, ui.window, ui.dialog, shell.navigate, shell.converse]
inference_list  :: 74 declared; pinned row payload = {"artifactBinding":{"encoding":"artifact-pack-base64",
                   "field":"document","required":true},"inputSchema":"{…1110 B…}", …}
inference_run   :: 3s, progressRows=3
                   progress 0     inference_run
                   progress 0.05  checking `s.wfc.bitmap.solve` against its published payload contract
                   progress 0.15  binding artifact `wi1-…` into the request body
job_get         :: status=FAILED progress=0.15 message="binding artifact `wi1-…` into the request body"
                   error.field="artifactId"
```

So over the SHELL channel an agent reads the contract, the run reports progress live, and
`job_get` answers a typed, field-named outcome. Screenshot: `🗑️generated/wi1-shell-before-inference.png`.

**What this screenshot does NOT show, and why.** The `s` shell was on its home screen ("No studios
yet. Create one from the navbar"), so no `🀄️wfc` surface was open, and in shell mode
`artifact_create` refuses by design: *"this artifact handle names plugin `wfc`, which has no open
instance in the attached shell — open one there first"*. Opening one means driving the shell's own
studio UI (`space…home.createStudio` → `space…studio.spawnApp`, both of which take an EMPTY input
schema, so they are not addressable to a kind from MCP — it is a browser-click journey). That
journey was not run. **No screenshot of a solved bitmap is offered, and none is claimed.**

### 7.2 The last red, root-caused — and it is not the payload contract

`client-e2e`'s `os: inference_run` is red with `SIDE_EFFECT_REJECTED … guest trapped`. Two probes
separate the causes:

| probe | payload | result |
| --- | --- | --- |
| `🐍️wi1-inference-probe.ts` (`wi1-inference-probe-{2,3}.txt`) | `artifactId` → the gateway binds `document` | trap, innermost frame `<semio_framework_deflate::component::Inflater>::advance`, via `artifact_pair_snapshot` → `parse_document_pack` → `decode_pack` → `codec_decompress` |
| `🐍️wi1-snapshot-payload-probe.ts` (`wi1-snapshot-payload-probe.txt`) | a HAND-WRITTEN `payload.snapshot` (the crate's own 4×4 stripes fixture) — never touches the new carrier | trap, innermost frame `<alloc::Global as Allocator>::deallocate`, via `drop_glue::<wit_bindgen::rt::async_support::FutureState>` → `Arc<FutureWaker>::drop_slow` |

The second probe is the one that matters: **it reproduces the trap with a payload that never goes
near `ArtifactDocumentPayload`, `artifact_pair_snapshot` or `into_snapshot`.** The lane itself
traps. Three more facts pin it to the guest and not to the bound bytes:

1. **The bytes are right.** The bound pack was captured off the wire (405 B pack, 211 B spr) and
   inspected: it carries the `\x89SEM` container, the inner `\x89SPK` pack, and valid raw-DEFLATE
   segments that Python's `zlib` inflates (offset 92 → 426 B, beginning `\r\x00\x06\x00…AAACAAAAAAIA…`,
   the bitmap's own base64 pixels). Nothing is truncated or corrupt.
2. **The seam is byte-exact.** New law
   `a_bound_document_round_trips_through_the_guest_json_decoder` encodes a 405 B pack + 211 B spr
   through `bind_inference_document`, re-reads the result with the GUEST's own DSL JSON decoder
   (`semio_framework_os_kernel::os_pack::json::from_json_str`) and base64-decodes it: byte-identical.
   The host↔guest carrier is proven, natively, without a component.
3. **The trap address moves between runs** (`0x338e0c4`, `0x338e0e9`, `0x338e124` inside `advance`),
   which is the signature of a corrupted allocator / interrupted runtime, not of a deterministic
   decode failure on fixed input.

This is PZ2 §5.1's "does not come back" with the silence removed: the same `semio.infer` lane, now
failing in 1–17 s with a backtrace instead of hanging for 240 s. Root-causing it belongs to the
guest job lane (`🔌️plugin/⚛️reactor/💼️jobs` + `wit_bindgen` async support), not to the contract
packet, and every attempt costs a ~25-minute mutex hold — so it is handed over, precisely located,
rather than guessed at here.

## 8. Honest gaps

1. **The three `🀄️wfc` laws are authored, not yet run** (§4). A native `cargo check` of the wfc
   artifact crates links `semio-framework-artifact-flow-flow`, which a peer had red at 17:51 and
   again at 20:5x; `🌊️flow` is explicitly out of this slice's scope. They are proven to COMPILE for
   `wasm32-wasip2` (§5) but no test binary has executed them.
2. **`inference_submit` was not extended.** It is the HUB-bound GIS Map route
   (`hub_inference_route_for`), not the plugin-declared one, and it carries no `payload` argument at
   all — so there is nothing to validate against a contract there yet. The brief named it; this is
   the honest reason it is unchanged.
3. **Host-side validation enforces the contract's TOP-LEVEL `required` list only.** A payload that
   satisfies every required name but violates a nested type still reaches the guest, which refuses
   it by its own decode fault. A full JSON Schema evaluator in the gateway would duplicate the
   authority the guest already has; the split is deliberate, but it is a split.
4. **`job_cancel` does not reach a running guest.** `inference_run` is one synchronous call into the
   guest: the handler checks the host cancel flag before dispatch and after it, and the
   `cancellationId` travels on the wire for the guest's own `semio.infer` loop to poll — but a
   `job_cancel` issued while the guest is mid-solve is only observed when that guest polls. Inherited
   shape, not introduced here.
5. **Only `🀄️wfc` publishes a contract.** `🌍️gis` (1), `📐️cad`'s extension (1) and `🗄️stdio` (68)
   carry `payload: None`, so `inference_run` on them behaves exactly as before when a payload is
   supplied, and now refuses BY NAME when one is not. That refusal is a behaviour change for those
   plugins: previously such a call dispatched `{}` into the guest.
6. **The last `client-e2e` red is NOT closed** (§7.2). `inference_run` reaches the guest and the
   guest traps. The contract, the binding, the progress lane and the job lane are all green around
   it; the solve itself does not complete, so **no inference result has been produced end to end by
   this slice** and none is claimed.
7. **Two pre-existing `workspace::quick` reds** were seen while running the suite —
   `authenticated_hub_workspace_resources_are_snapshot_only_scoped_and_fail_closed_when_stale`
   (expects an `Err` where a hub `…/schema` read now answers `Ok`, which is M10's change) and
   `read_resource_artifact_returns_real_bytes_after_a_commit` (store drop witness). Both fail in
   ISOLATION on this tree and neither touches inference; consistent with TS1's "os-mcp 45/50".
   Not introduced here, not fixed here.
8. **`hub-agent-participant-check` has no inference rows to report.** Grepping the whole gate
   (`🌉️mcp/🧪️tests/🤖️hub-agent-participant/**`) finds `inference.execute` only inside the scope
   string it requests — no `inference_run`/`inference_list` step exists there. Nothing was run
   against hub 7651 for this slice, and nothing is claimed.

## 9. Files changed

**Framework**
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — `InferencePayloadContract`, `InferenceArtifactBinding`, `INFERENCE_ARTIFACT_PACK_BASE64`, `ContributedInferenceMetadata.payload`
- `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs` — TS projection rows for both new types
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️plugin-dependency/🦀️.rs` — fixture field
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — `ParsedDocumentText::into_snapshot()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — SDK contract types, `ArtifactDocumentPayload`, `artifact_pair_snapshot`, wire twins, identity-projection metadata compare, exports
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs` — the contract reaches the committed descriptor
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/{🧪️tests/🔬️app-artifact-inference-wire,🧪️tests/🔬️app-artifact-inference-service,🛂️describe/🧪️tests/🔬️unit,🏗️builder/🧪️tests/🔬️schema-stamping,⚛️reactor/💼️jobs/💡️infer/🧪️tests/🔬️unit}/🦀️.rs` — `payload: None` in six fixtures

**Gateway (`🌉️mcp`)**
- `🔀️dispatch/🦀️.rs` — `InferCommand.artifact_id` / `.artifact_document`, `ArtifactDocumentBinding`
- `🏠️workspace/🦀️.rs` — `bind_inference_document`; `infer_real` routes its payload through it
- `💡️inference/🦀️.rs` — `DeclaredInference.payload`, `validate_inference_request`, `contract_required_fields`, `inference_field_invalid`, `inference_error_field`, `resolve_inference_artifact_document`, handler re-ordering (§3.1)
- `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`, `💡️inference/🧪️tests/🔬️quick/🦀️.rs` — 10 new laws
- `🟦️.ts` — `client-e2e` binds the inference to its own artifact and requires a published contract
- `🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs` — `InferCommand` fixture fields

**Plugins**
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/{🖼️bitmap,🔲️grid2d,◻️2d,🧊️3d,🧱️grid3d}/…/🧬️schema/💡️inferences/🦀️.rs` — request schema, contract, `document` carrier, `resolve_snapshot()`
- the same five `…/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` — request-literal updates; `🖼️bitmap`'s also gains 3 contract laws
- `✏️s/🔌️plugins/🀄️wfc/{🔣️.json,🛂️.descriptor.semio}` — recommitted by the plugin's own `describe` (§5)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs`, `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs` — `payload: None` (no behaviour change; they publish no contract yet)

**Ticket files** — `📜️wi1-wfc-check-and-describe.sh`, `📜️wi1-serve.sh`, `🐍️wi1-shell-inference-probe.ts`, this report.
Captures: `🗑️generated/wi1-{check-framework,check-plugin,check-plugin-all-targets,check-mcp,check-mcp-2,check-wfc-artifacts,test-mcp-laws,test-mcp-inference,mcp-build,client-e2e-before,client-e2e-2,wasm-check,describe-wfc,describe-ledger,inference-bridge-source,serve-s-6200}.txt`.

**Not touched**: `✏️s/🔌️plugins/🌊️flow/**`, `🌎️hub/**`.
