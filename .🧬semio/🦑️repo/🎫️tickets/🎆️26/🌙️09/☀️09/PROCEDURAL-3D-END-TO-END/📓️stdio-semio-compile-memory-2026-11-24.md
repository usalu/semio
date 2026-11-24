# stdio-semio compile memory (2026-11-24)

## Root cause
`semio-s-artifact-stdio-semio` has a source closure of only 48 KB (lib root 24 KB, base io 23 KB, one example), so the size gate never saw it. Its compile memory comes from one macro invocation: `space_members!` in the lib root, with 18 rows. Every row instantiates in the same compilation unit:
- `ArtifactStore<Snapshot, Mutation>` as a boxed variant, and the `SpaceMember` (about 60 methods, about 20 of them `async`), `ReferenceSpaceMember` and `MemberVisit` delegation over all 18 stores, so every async delegating future is an enum over 18 distinct generic store futures;
- `InitialMemberStoreOpen<Members, Snapshot, Mutation, Producer>` per row (the open machine of about 50 KB of source), its `size_of` queries (layout of each open state) and the `begin_open` / `step` / `close_step` arms;
- `create_member_store` and `open_member_store` async fns per row.
Cost is linear in the rows and lives in one rustc process. The hub build logged the unit at an admission estimate of 5311 to 6352 MB and SIGKILLs at 1.78, 2.62, 2.84 and 3.08 GiB resident, still growing.

## Split
The rows moved into six member group crates (three rows each), every group a plain `space_members!` expansion of its own subsets, below the artifact crate:
| crate | rows |
|---|---|
| `semio-s-artifact-stdio-semio-members-animation-brep` | animation, audio, brep |
| `semio-s-artifact-stdio-semio-members-cad-drawing` | cad, document, drawing |
| `semio-s-artifact-stdio-semio-members-flow-image` | flow, graph, image |
| `semio-s-artifact-stdio-semio-members-kit-model` | kit, mesh, model |
| `semio-s-artifact-stdio-semio-members-object-table` | object, presentation, table |
| `semio-s-artifact-stdio-semio-members-text-video` | text, value, video |
They live at `✉️base/👥️members/🧩️<group>/` (lib `🦀️.rs`, manifest in `📦️packages/🦀️rust`). Groups are contiguous in the alphabetical row order so the order of `OPEN_DECLARATIONS` (pinned by the member-factory-identity fixture) is unchanged.

The framework macro file `🏪️store/🎲️space-members-macro/🦀️.rs` gained:
- `space_member_delegation!`: the `SpaceMember` / `ReferenceSpaceMember` delegation, cut out of `space_members!` so both macros share it;
- `space_member_groups!`: composes group enums into one member enum. It only routes: delegation, `Open` as an enum of the group opens (inline, so accounting of the retained open state is the group's own), `begin_open` / `create` / `open` / demand queries picked by dialect through `space_member_group_owns!`, and `OPEN_DECLARATIONS` as a const concatenation of the group tables;
- `semio_s_artifact_stdio_semio_base` owns `SEMIO_MEMBER_HISTORY_LIMITS` and `SEMIO_MEMBER_HISTORY_ALLOCATION_BYTES` once, instead of a literal per group.

`SemioMembers` keeps its name and the `create_semio_member` / `open_semio_member` entry points; its variants are now the six groups (`SemioMembers::FlowImage(group)` then `SemioMembersFlowImage::Flow(_)`).

## Other files
- `Cargo.toml` (workspace members and dependencies) and `📦️packages/🦀️rust/Cargo.toml` of the artifact crate;
- `📋️build-owners.json` (six owners);
- 45 tracked `Cargo.lock` files that contain the artifact crate (six packages added, six deps added to the artifact package), so `--locked` builds keep working;
- `🏭️member-factory-identity/🟦️.ts` reads the 18 rows from the group sources;
- flow snapshot binary test: the `Flow` match is nested.

## Measurement (hub-14, 2026-11-24)
All seven crates (six groups and the artifact crate) compiled in hub-14 without an error; the pass went on to `semio_hub_flow`. Peak RSS of the rustc process, sampled with ps every 4 s (so a lower bound; the hub log reports only the aggregate):
| unit | peak RSS |
|---|---|
| `members-object-table` | 2568 MB |
| `members-kit-model` | 2478 MB |
| `members-animation-brep` | 2247 MB |
| `members-text-video` | at least 925 MB (sampled partially) |
| `members-flow-image` | at least 176 MB (sampled partially) |
| `members-cad-drawing` | at least 125 MB (sampled partially) |
| `semio-s-artifact-stdio-semio` (routing, io) | about 133 MB sampled, compiled in seconds |
Before: one unit estimated at 5311 to 6352 MB, SIGKILLed at 1.78 to 3.08 GiB while still growing. So the worst unit fell from more than 3 GiB (unbounded) to about 2.6 GiB, and no unit competes with the others for one huge reservation, but the target of well under 2 GiB is not met for three groups: a row costs roughly 0.8 GB, so three rows cost 2.2 to 2.6 GB.

## Next step if the 2 GiB target stays
Split the groups into pairs (about 1.7 GB each) or single rows (about 0.8 GB each). It is mechanical: the group sources, manifests, workspace entries, build-owners rows and the 45 lock files are all generated from the same row table, and `space_member_groups!` takes any number of groups. Do it after the running hub build, because it changes the graph.

## Final layout: one crate per subset row (applied after hub-14 exited)
Decision after the hub-14 numbers (about 0.8 GB per row): the six groups are gone, no leftovers. Each of the 18 subsets owns its member store at `🪆️subsets/<subset>/👥️member/` (lib `🦀️.rs`, manifest in `📦️packages/🦀️rust`), crate `semio-s-artifact-stdio-semio-<subset>-member`, type `Semio<Subset>Member`. `SemioMembers` now has one variant per subset (`SemioMembers::Flow(group)` then `SemioFlowMember::Flow(_)`). `space_member_groups!` is unchanged. Applied by `apply-stdio-semio-member-crates.py` (kept in this folder): removes `✉️base/👥️members`, writes the 18 crates, updates the workspace and artifact manifests, `📋️build-owners.json`, the 45 lock files, the flow snapshot test and the member-factory-identity check (it now reads the 18 `👥️member` sources and sorts the rows by subset).

## Result of the final layout
- `repo cargo ... check -p semio-s-artifact-stdio-semio --lib -j 1` (35 min 36 s, cold) finished: all 18 member crates and the artifact crate check without an error. Peak RSS of the largest process of the whole run, dependencies included: 1.43 GB (`time -l`). So no member crate in check mode exceeds 1.43 GB; check does not run the code generation, which is where most of the monomorphization cost sits.
- Per-crate code generation peaks still need one hub run: the expected value is about 0.8 GB per member crate from the hub-14 rows (2.2 to 2.6 GB for three rows).
- hub-14 itself failed on a different unit: `semio-hub-flow-component` (cdylib) SIGKILLed at 3.33 GiB. All stdio-semio units compiled before that.
- Oracle: see the line below.
- Oracle: the repo script `member-factory-identity-check --oracle-only` was started (log not kept) but was still reading the project graph after about 40 minutes (host saturated), so it did not report. Its only input that changed is the declaration extraction, which I replayed directly: the 18 rows read from the `👥️member` sources, sorted by subset, equal `fixture.declarations` exactly, and the `SemioMembers` variant order equals the same order. Re-run the script when the host is quieter.
