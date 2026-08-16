# Stdio Executable-Registration Contract SCC Audit

## Decision

**No-go for a registry-only or glTF-only repair.** The failure is one contract
SCC spanning the stdio source schema, its Rust registry, the root TypeScript
structural validator, and framework typed capability registration. No source
files were changed by this audit.

The proposed Terra leaf patch is intentionally withheld. A leaf could make the
JSON parse, but cannot truthfully establish that the declared codecs,
mutations, or inferences have concrete executable registrations.

## Evidence

### Current ownership and dirty isolation

| Owner | Path | State | Finding |
| --- | --- | --- | --- |
| Stdio registry | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` | `MM` | Rust source requires `runtime_capabilities` and checks executable rows. |
| glTF artifact definition | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json` | `MM` | Active working roster is 6 codecs, 18 mutations, 15 inferences; all current rows are unimplemented and unregistered. |
| Framework plugin capability owner | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | `MM` | Capability declarations are claimless; runtime declarations require exact matching claims. |
| Root validator and type owner | `📜️script.ts` | protected central | The exact-field validator does not yet allow `runtime_capabilities`. |

`MM` means the index and worktree already differ. These owners must stay
quarantined until one coordinator-owned SCC lease starts; this audit did not
rewrite, stage, or reconcile any of them.

### Reproducible gate failure

Run on the current worktree:

```text
bun nx run workspace:stdio-quick --skip-nx-cache
```

Result: failed before Rust registry compilation with:

```text
[stdio] artifact definition has unknown fields runtime_capabilities.
at 📜️script.ts:1346
```

Thus the root TypeScript validator rejects a field the Rust `Source` schema
requires. This is a source-contract mismatch, not a glTF data-only error.

### Rust registry inspection

`Source` owns `codecs`, `mutations`, `inferences`, and
`runtime_capabilities`. Its validator currently rejects each codec or
executable leaf unless `executable_registration` is true and `status` equals
`implemented`. The diagnostic then unconditionally returns the typed-mapping
error for the first item. Consequently any non-empty planned ledger fails,
even when its status is correctly unimplemented.

`Registry::build` converts standards, profiles, dialects, representations,
resources, localizations, and conformance suites into the framework artifact
definition. It currently emits no capability evidence for the codec,
mutation, or inference ledger, so a boolean in a JSON row is not a typed
runtime registration.

### Framework capability inspection

The framework builder exposes codec, mutation, and inference declaration
methods, but their current stdio-created capabilities do not carry the claims
needed by the runtime declaration APIs. Runtime declaration registration checks
for an exact kind-and-claims capability. The active glTF declaration currently
registers a document codec and aggregate geometric-analysis inference service;
it does not establish independent runtime entries for the 6/18/15 leaf roster.

The framework `ArtifactDeclaration` also lacks the query surface necessary for
the registry to verify the concrete runtime target represented by a ledger row.
Changing only the registry would therefore either make an unverifiable claim or
weaken validation, both prohibited by the ticket.

## Required Atomic SCC

The next implementation lease must include these semantic owners and no direct
generated-output writes:

1. Root stdio schema validation (`📜️script.ts`) — admit and type the canonical
   field in the exact schema contract at the same time as Rust.
2. Stdio registry/schema (`📇️registry/🦀️component.rs`) — make ledger validation
   status-aware and validate an owned typed runtime binding, not a boolean.
3. Framework plugin capability owner — provide a repository-owned typed
   capability-to-runtime-registration contract and the query/build API needed
   to verify it. This must remain behind framework types; no external-library
   type may cross the boundary.
4. Every active authored stdio artifact definition that participates in the
   shared schema (the independent audit found 36 definitions needing the Rust
   field contract), via the owning schema/generator path. Do not hand-edit a
   generated mirror.
5. Stdio plugin integration tests/runtime surface, to prove the registry build
   actually binds the declared capability records.

The glTF definition is one consumer of this contract, not a special-case
exception. Its canonical roster is **6 codecs / 18 mutations / 15 inferences**
in the active working tree. The older 6/28/15 observation is not a basis for
creating records; the SCC must derive its rows from the actual registered
component/mount graph and record any intentional roster correction atomically.

## Required Contract Shape

The SCC must select and enforce one schema-first owned representation, with
these invariants:

1. Each ledger item has a canonical semantic-component ID, a kind
   (`codec`, `mutation`, or `inference`), and an explicit concrete runtime
   registration target. `executable_registration` cannot remain an
   unverified boolean.
2. A runtime binding is accepted only when the framework declaration contains
   exactly one typed capability with the same ID, kind, and claims/target.
   Missing, stale, duplicate, or cross-kind mappings fail.
3. Status and registration are distinct facts. An unimplemented item cannot
   claim a runtime binding; an implemented item must be backed by one. The
   validator must not reject a non-empty planned ledger merely for being
   planned.
4. The Rust parser, TypeScript exact-field/type validator, artifact schema
   source, registry builder, and framework validation use the same canonical
   representation in the same change.
5. No glTF-specific branch, compatibility alias, baseline, allowlist, or
   direct generated edit is permitted.

The current generic document-codec and aggregate-inference registrations may
only satisfy leaf records if the new typed contract explicitly represents the
many-to-one target and framework semantics accept that relationship. Otherwise
the leaf ledger remains unregistered until each real runtime registration
exists. That decision belongs in the SCC design; it cannot be inferred from a
source import.

## Lease Boundary and Handoff

This is a **coordinator-owned joint framework/stdio contract lease**, not a
Terra leaf lease. It conflicts with currently protected root script and dirty
framework/registry/artifact-definition owners. Before writing, the coordinator
must rehash each owner and acquire one shared lease; if the framework API would
need a semantic change beyond this owned capability contract, stop and record
the API decision before changing schema data.

No taxonomy, root Cargo, launch configuration, repository-library index, or
direct generated path belongs to the initial implementation write set.

## Acceptance Matrix

After the atomic change, run through Nx and retain the exact output in a
follow-up ticket record:

```text
bun nx run workspace:stdio-quick --skip-nx-cache
bun nx run @semio-tech/stdio-js:test-quick --skip-nx-cache
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
bun ./📜️script.ts verify taxonomy enforce --scope s.stdio.gltf
```

Add source-owned tests that prove: a planned leaf is permitted but unbound; an
implemented leaf without a typed registration fails; valid codec/mutation/
inference bindings pass; a stale, duplicate, wrong-kind, or wrong-target
binding fails; and the registry build exposes each validated capability to the
framework runtime surface. Finally exercise the registered stdio plugin
artifact assembly, not a fixture-only parser.

## Commands and Result

| Command | Result |
| --- | --- |
| `git status --short` on the four contract owners | All Rust/schema/framework owners already `MM`; central root remains protected. |
| `bun nx run workspace:stdio-quick --skip-nx-cache` | Failed at the root TypeScript unknown-field gate, as quoted above. |
| Static source audit of registry and framework builder/declaration paths | Confirmed the missing typed mapping and exact-claims gap. |

## Next Action

Reserve a combined framework-plugin/stdio-registry/root-schema SCC only after
the protected writers release their current changes. Do not begin glTF leaf
edits or registry-only validation changes before that lease is active.
