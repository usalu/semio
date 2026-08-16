# glTF Mutation SCC Central Registrar

## Lease

The central registrar lease changed only these implementation paths:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
- `📜️script.ts`

No protected repo-library index, mutation leaf, manifest, AGENTS.md, compatibility alias, or generated output was edited.

## Precondition

The amended request was used after the source set was rehashed. The resolved source fingerprint is:

```text
068d5321f7a15c782540a83db5b1c207f5a62d8fa8a8a9f50fef4c2b0a35aca4
```

It matches the requested fingerprint. The mutation source tree has exactly 28 direct `🦀️component.rs` leaves.

## Applied Registration

The glTF schema mutation list now mounts exactly these direct command components:

```text
no-mutation             set-snapshot             set-asset
insert-scene            remove-scene             set-scene
insert-node             remove-node              set-node
transform-node          reparent-node            bind-node-mesh
insert-mesh             remove-mesh              set-mesh
insert-accessor         remove-accessor          set-accessor
insert-material         remove-material          set-material
bind-primitive-material insert-buffer            remove-buffer
set-buffer              insert-animation         remove-animation
set-animation
```

The former schema mutation root, planning, text/binary, and all nested event/diff/inverse facet mounts were removed. `mutation_dispatch` is mounted once at `schema::modules::mutation_dispatch`; it is colocated with the four previously registered subset modules because all migrated consumers import that canonical owner. The I/O boundary mounts one text and one binary mutations transport at `io::mutations`.

The root generator input replacement is exact: the former glTF mutation-root component entry is absent and `🔨️modules/🧭️mutation-dispatch/🦀️component.rs` occurs once.

## Static Validation

```text
direct commands:                         28
mutation_dispatch mounts:                 1
mutations text transport mounts:          1
mutations binary transport mounts:        1
former root/planning/text/binary mounts:  0
git diff --check:                         pass
```

`bun nx show project @semio-tech/stdio-plugin --json` exposes only `test`, `test-quick`, `test-long`, and `test-exhaustive`. It has no generation target, so there is no configured Nx generator to run for this registrar. No manual generated-output edit was made.

## Runtime Validation and Handoff

Command run:

```text
bun nx run @semio-tech/stdio-plugin:test-quick --skip-nx-cache
```

Result: **failed (exit 1)** after the registrar paths resolved. The remaining errors are source-contract omissions in the mutation-dispatch component: direct mutation leaves import `GltfSemanticMutation`, `check_index`, `reject`, `remap_references`, `remove_checked`, `shift_insert`, and `IndexFamily`, but `schema::modules::mutation_dispatch` does not export them yet. This is a mutation-SCC leaf/API completion lease, not a registry topology failure. No forwarding module or alias was added.

The prior transient inference import-depth failures were no longer present in the settled rerun. The Terra source owner can complete the mutation-dispatch public contract, then rerun the command above and the scoped taxonomy report.
