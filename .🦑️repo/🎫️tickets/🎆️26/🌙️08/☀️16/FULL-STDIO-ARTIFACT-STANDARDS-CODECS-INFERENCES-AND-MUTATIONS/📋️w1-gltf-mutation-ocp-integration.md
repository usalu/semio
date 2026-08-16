# glTF Mutation OCP Integration

Ticket: `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`

## Current Registry

The schema-owned immutable descriptor assembly currently registers exactly these compile candidates:

| Canonical ID | Version | Descriptor |
| --- | ---: | --- |
| `s.stdio.gltf.mutation.change-material-alpha-mode.v1` | 1 | `change-material-alpha-mode/🦀️component.rs::DESCRIPTOR` |
| `s.stdio.gltf.mutation.change-material-double-sided.v1` | 1 | `change-material-double-sided/🦀️component.rs::DESCRIPTOR` |

Both roots delegate decoding, derivation, inverse reconstruction, stale checks, apply, and concrete
touched-path recomputation to their own semantic leaves. The central runtime holds only immutable
descriptors and generic envelopes; it does not inspect command payload fields.

## Honest Coverage Status

- Registered compile candidates: **2 / 222**.
- Accepted leaves: **0 / 222** until the Rust registry/codec/vector gate executes successfully.
- Unregistered leaves: **220 / 222**.

`218 / 222` is not truthful for this worktree: it would require four more descriptors to be
registered. The four relation descriptors (`bind-node-child`, `unbind-node-child`,
`bind-scene-root-node`, `unbind-scene-root-node`) remain intentionally unmounted while their Rust
forged-path and vector gate is blocked. `create-scene` remains unmounted because its root still
uses a private adapter and lacks forged-path rejection. No legacy no-op, snapshot, generic
set/insert/remove, enum tag, alias, or switch is mounted.

## Runtime Boundary

- `GltfMutation` is a canonical `{ commandId, version, phase, payload }` envelope.
- `GltfMutationDiff` is a canonical diff-envelope sequence with concrete `touchedPaths`.
- The registry rejects duplicate IDs, unknown IDs, stale versions, malformed phases, and all
  command/payload/path/diff-count budget overruns.
- Diff and inverse apply derive concrete leaf paths and compare them exactly with the envelope
  before returning a snapshot. `try_apply` returns a typed registry error; the legacy infallible
  `MutationDiff::apply` bridge explicitly panics rather than silently accepting a rejected diff.
- Text and binary transports are generic canonical-envelope codecs in their own I/O facets.

## Added Gates

The mutation runtime includes duplicate/unknown/stale registry tests, forward/inverse planning and
stale-base checks, forged-path rejection, and forward-plus-inverse restoration. The GLTF I/O
conformance region now covers generic text/binary round trips, unknown/stale/budget rejection,
trailing binary rejection, and grammar/protocol parity without a legacy variant case list.

## Verification

`cargo check -p semio-s-plugin-stdio --lib` was run after mounting the two candidates. It did not
reach the stdio crate because `semio-framework-plugin` currently fails first: its `AppFrame`
construction misses `messages` and `report`, and it calls the removed
`ArtifactStore::snapshot_with_conflicts`. The complete captured output is
[`🧪️w1-gltf-mutation-ocp-candidate-check.txt`](./🧪️w1-gltf-mutation-ocp-candidate-check.txt).

No green Rust claim is made until that external framework-plugin failure is repaired and the
candidate and relation Rust gates run.
