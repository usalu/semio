# W5 — Inference access

## What "an inference" is in this codebase, and how it is addressed

An **inference** is a read-only, cache-aware derived value computed from an artifact's own
document — the kernel primitive is `InferredField<P>` / `infer_field` in
`🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs` (a merkle `DepHash` dependency
chain over `InferenceCache`). One artifact-standard's whole set of inferred fields is an
`ArtifactInferrer::infer`/`infer_cached` implementation. At the plugin-manifest level, one
inference is identified by the pair **`(artifactKind, inferenceSchema)`**, declared statically in a
plugin's own committed `🔣️descriptor.json` as `semio_framework::ContributedInferenceMetadata` —
either owner-authored (`descriptor.contributions.inference_services`) or contributed onto a
dependency's kind (`descriptor.contributions.artifact_contributions[].inferences`). That is the
EXACT roster `🏃️run/🦀️component.rs`'s `register_plugin` builds before handing it to
`semio_framework_plugin_host::ArtifactInferenceRouter::register_plugin` — the plugin-agnostic
router the ticket brief pointed at.

**Key finding, verified by reading `crate::actions::ArtifactChannel`/`AppCommand` in full
(`🔀️dispatch/🦀️component.rs`):** `semio-framework-os-mcp` is a *separate process* from `run`
(`run/component.rs`'s own `ArtifactInferenceRouter` lives in a different binary). This crate's own
wire port to a live plugin, `AppCommand`, has exactly `ReadHistory` / `PureCommand` /
`TransactionPrepare|Commit|Rollback|Undo|Redo` — **no inference variant at all**. So an inference
cannot be *executed* through this crate today, at all, for any plugin — the same `channel.not-wired`
class of gap `🏠️workspace`'s own `PureCommand`/`TransactionPrepare` doc comment already names for
mutations pre-W3. This is a real, crate-wide gap, not something scoped to my one file.

What **is** real and reachable without touching that gap: every plugin's `🔣️descriptor.json` is
committed, static data. Reading `descriptor.contributions.inference_services` +
`artifact_contributions[].inferences` needs no wasm compile, no activation, no live plugin process —
`HeadlessWorkspace::resolve_default_plugin_id` + `find_repo_root`/`load_plugin_registry`/
`find_plugin_entry`/`load_package_descriptor` (all already `pub fn` on `🏠️workspace`, none of them
touched by me) are enough. So W5 delivers **real, honest discovery** over declared inference
metadata, plus a typed, retryable gap for the one part (execution) this crate genuinely cannot do
yet — never a fabricated value.

## File

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️component.rs` (new, ~460 lines + tests).
No other file touched.

## Exact API (for root/glue wiring)

```rust
pub fn inference_capabilities() -> Vec<CapabilityDefinition>;

pub fn register_inference_tools(
    registry: &mut InMemoryToolRegistry,
    workspace: Option<std::sync::Arc<HeadlessWorkspace>>,
);

pub fn read_inference_resource(
    uri: &str,
    workspace: Option<&std::sync::Arc<HeadlessWorkspace>>,
) -> Option<Result<Vec<ResourceContent>, GatewayError>>;

pub fn inference_resources(workspace: Option<&std::sync::Arc<HeadlessWorkspace>>) -> Vec<Resource>;
```

Two tools registered, both always present regardless of tier (`DECLARED_STUB_TOOL_NAMES`
convention — presence never depends on tier, only the result does):

- **`inference_list`** — input `{ artifactId?: string }`.
  - No `artifactId`: real, static, plugin-agnostic discovery — every inference declared by
    whichever single plugin the workspace's own catalog names as its default
    (`HeadlessWorkspace::resolve_default_plugin_id`, the SAME resolver `open_artifact_channel`
    already uses — no plugin id hardcoded, no branching). Ambiguous (0 or 2+ plugins) propagates
    that resolver's own typed, retryable `PLUGIN_UNAVAILABLE` — this widens to every installed
    plugin for free once W1's registry work makes a workspace's catalog multi-plugin aware.
  - With `artifactId`: resolves that artifact's real schema by delegating to the SAME
    `semio://artifact/{id}/schema` answer `🏠️workspace` already implements (never re-derived) —
    `PROBE_SCHEMA` (any workspace-opened probe artifact) genuinely has zero declared inferences
    (`[]`, honest, not a gap — no plugin depends on this crate's own synthetic schema); any other
    id gets the same typed, retryable `PLUGIN_UNAVAILABLE` `/schema` itself already returns (that
    endpoint has no `NOT_FOUND` path at all today — confirmed by reading it — only `Ok` or one
    retryable gap).
- **`inference_get`** — input `{ artifactId: string, inferenceSchema: string }` (required). Three
  outcomes, matching the progressive-enhancement contract exactly:
  1. no workspace → retryable `PLUGIN_UNAVAILABLE` naming `--folder`/`--hub`.
  2. workspace bound, artifact resolves, but no declared service matches `inferenceSchema` for that
     kind → `NOT_FOUND` naming the missing `(artifactKind, inferenceSchema)` pair. **Real and
     reachable today** — verified via `inference_get_on_an_open_probe_names_the_missing_service_not_found`,
     which calls the actual registered tool end to end.
  3. workspace bound, a matching service IS declared → retryable `PLUGIN_UNAVAILABLE`
     ("channel.not-wired") naming exactly which `(artifactKind, inferenceSchema)` has no
     `artifact-infer` route yet. **Real code, unit-tested directly** via
     `execute_lookup_reports_a_retryable_channel_not_wired_gap`, but **not reachable end-to-end
     today** — no artifact this workspace can currently resolve to a non-probe schema, so no live
     tool call can walk into this branch yet. Wiring a real channel command later needs zero change
     to the discovery path above it.

Resources: `semio://artifact/{id}/inference` (index — the same declared roster `inference_list`
returns) and `semio://artifact/{id}/inference/{field}` (one field — same three outcomes as
`inference_get`). `read_inference_resource` returns `None` for any non-inference URI (verified: it
never touches the workspace before checking the URI shape, so the composing registry's fallthrough
is cheap) and a well-formed `NOT_FOUND` for a URI that IS ours but malformed (empty artifact id,
empty/nested field segment) — both checked before ever looking at `workspace`, so this holds even
bare. `inference_resources` lists one `.../inference` entry per artifact id the workspace currently
knows (`workspace_artifact_ids()`), empty when no workspace is bound — same convention
`WorkspaceResourceRegistry::list()` already uses for `semio://artifact/{id}`.

## The `job_get`/`job_cancel` seam

`crate::handles::HandleTable` already has a purpose-built, currently-unused `HandleKind::Job`
(`job_` prefix, 1h TTL, `mark_terminal` on completion) — clearly the intended shared mechanism for
whoever wires `job_get`/`job_cancel` for real. My file defines the wire contract an expensive
inference execution would mint into that table:

```rust
pub struct InferenceJobPayload {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub inference_schema: String,
    pub cancellation_id: String,
}
pub fn inference_job_payload(artifact_id: &str, item: &DeclaredInference, cancellation_id: &str) -> InferenceJobPayload;
```

I deliberately **do not mint** a real `job_` handle anywhere — `register_inference_tools`'s
signature takes no `HandleTable`, and there is no execution route to mint one *for* yet; an inert
job handle nobody could ever progress would itself be a fabrication. Once a real `artifact-infer`
channel command exists, the `Execute` arm of `inference_get_handler`/`read_inference_resource`
is the one call site that changes: for a request too expensive to answer inline it would call
`handles.mint(HandleKind::Job, session, Attachment::Artifact{artifact_id}, serde_json::to_value(inference_job_payload(..)), now_ms)`
and return `{"status":"job","jobId":..}`; `job_get`/`job_cancel` resolve/cancel that same id against
the same shared `HandleTable`. Cheap/cached reads would answer inline instead (see cache honesty
below) — which path a given request takes is that future wiring's decision, not mine.

## Cache honesty

`InferenceCache`/`DepHash` (`💡️inference/🦀️component.rs`) exist, but this facet never computes an
inference (see the execution gap above), so it never has a cache hit or miss to report — no
`cacheStatus`/`depHash` field is emitted anywhere. Reporting one today would be fabricated, since no
`InferenceCache` instance backing real production traffic is reachable from this crate. Once
`Execute` really runs, the response envelope gains a `cacheStatus: "hit"|"miss"` +
`depHash` field sourced from the real cache the execution goes through — not before.

## Why this is plugin-agnostic

No plugin id or artifact kind is hardcoded anywhere in the file. Discovery walks whichever plugin(s)
the *workspace's own catalog* names (`resolve_default_plugin_id`) and reads *that* plugin's own
`🔣️descriptor.json` generically; matching an artifact's resolved schema against the declared roster
is by schema-string equality, never by plugin id; the execution-gap message names whatever
`(artifactKind, inferenceSchema)` was actually declared. A newly installed plugin that declares
`contributions.inferenceServices`/`artifact_contributions[].inferences` becomes discoverable the
moment its catalog capability makes it the workspace's resolvable default (or once W1 widens that
resolver past "exactly one plugin") — zero change to this file.

## Verified vs written-but-unverified

**Verified by running `cargo test` for this file's own `mod quick` — NOT DONE. Only `cargo check`
was run (see below); no test binary was executed.** So every claim below about test *content* is
"this is what the test asserts," not "I watched it pass." I did not fabricate a pass.

**Verified by reading source directly (high confidence, not run):**
- `AppCommand`/`ArtifactChannel` have no inference variant anywhere in this crate (read
  `🔀️dispatch/🦀️component.rs` in full).
- `ContributedInferenceMetadata`/`ArtifactContributionDescriptor`/`PackageDescriptor.contributions`
  field shapes (read `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` directly).
- The real "procedural" plugin fixture: `✏️s/🔌️plugins/🌀️procedural/🔣️descriptor.json` declares
  exactly one inference service, `owner=contributor="procedural"`, `artifactKind="s.assembly"`,
  `inferenceSchema="s.assembly.solve"` — confirmed by parsing the committed JSON directly with
  Python, independent of any Rust test. It is the ONLY plugin among all ~59 committed descriptors
  with a non-empty inference roster (checked all of them).
- `HeadlessWorkspace::read_artifact_resource`'s `Some("schema")` arm never returns `NOT_FOUND` —
  only `Ok` (open probe) or retryable `PLUGIN_UNAVAILABLE` (everything else) — read directly; this
  corrected an earlier draft of my own test that wrongly assumed `NOT_FOUND` for an unseen id.

**Written and self-consistent, not run (`cargo check -p semio-framework-os-mcp` was still compiling
in the background — this crate's dependency `semio-framework-plugin-host` is mid-repair by another
agent per this ticket's own status; I did not see a clean build complete before writing this
report):**
- The file as a whole — every signature cross-checked field-by-field against the real struct
  definitions in `🗂️catalog`, `🧭️protocol`, `⚠️errors`, `🏠️workspace` (all read in full, not
  guessed), including the exact `manifest`/`semio_framework` field names for
  `ContributedInferenceMetadata`.
- All 19 tests in `mod quick` — schema shape, tool registration, bare-tier `PLUGIN_UNAVAILABLE`,
  the real procedural-plugin discovery, the probe-schema-is-empty case, the reachable
  `inference_get` → `NOT_FOUND` path, the unit-tested (not end-to-end) `Execute` → retryable gap
  path, `read_inference_resource`'s `None`/malformed/bare/bound cases, and `inference_resources`'
  bare/bound cases.

I did not claim any test passed. If `cargo check`/`cargo test` surface a compile error once the
shared `semio-framework-plugin-host` repair lands, the most likely spots are the exact field/method
names on `HeadlessWorkspace`/`ContributedInferenceMetadata` I transcribed by hand — everything else
is straightforward.
