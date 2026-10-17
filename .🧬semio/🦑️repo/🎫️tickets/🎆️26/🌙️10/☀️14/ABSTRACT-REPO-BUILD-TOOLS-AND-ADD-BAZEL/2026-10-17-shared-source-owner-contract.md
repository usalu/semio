# Shared Installation Membership And Unique Source Ownership

This is the implementation contract proposed after the actual Probe refusal: duplicate=true, observed=297, maximum=4096, framework Rust package node_modules while processing mit-bestand. The old observer correctly refused two unresolved destinations. This feature must replace the model coherently, not remove that guard in isolation. Production is unchanged at this design checkpoint.

## One Physical Endpoint, Many Installation Memberships

The input is the complete fresh ALL physical Bun ownership custody. Subset requests must not derive a model from a subset observation. Every declared scope contributes its own root plus all currently selected package members. Existing authored members, projected w/index manifests and frozen lock workspace/package edges remain present.

Each declared scope owns its own physical root. For every other package, choose the deepest declared physical scope ancestor using path-component containment. The chosen owner must actually include that package in its own current installation membership. Absence of that membership refuses; there is no shallower fallback. Members are byte-sorted by their canonical scope-relative path before assigning stable w/index slots; canonical output rows may be byte-sorted by Source package without changing those slots. Scope duplicates, duplicate members within one scope, noncanonical paths, escaped roots, symlinked physical ancestors, and conflicting physical aliases refuse. Many scopes may legitimately contain the same canonical physical package.

The result separates scope install members and slots from unique Source binding rows. Only the designated owner receives the package's Source node_modules transition. A nonowner still stages and installs its own projected member, because its frozen lock and isolated package graph require that membership. Root slots and member slots stay explicit; excluding a Source binding must not renumber or omit the member.

A pure first-party DATA codec derives the relation under the existing install schema. It is not a Native grant. Source metadata observers enumerate the derived unique endpoints, rather than deduplicating their previous conflicting target list. Current physical parent/leaf observers still reject drift and aliases.

Bounds are explicit: 64 scopes from the existing stage protocol; 4096 unique Source endpoints; at most 65536 membership rows; 4 MiB canonical relation metadata; existing canonical UTF8 sorting, Source work and 256-path limits. Every input is closed/proxy/accessor-safe. Limits refuse before partial model publication.

## Generation Identity Without Circular References

Each scope has a local Source/material seed and a local owner-relation seed, including all install members, their slots, designated owners and owner slots. Build an edge B→A when B installs a Source package designated to A. Edges to self do not create an external requirement.

Collapse the at-most-64-node graph into strongly connected components. The private first-party SCC implementation stays in the existing storage file; importing the large crate authoring module would create an unrelated dependency edge and threaten the current Source closure. Byte-sort member scopes, dependency groups and all serialized rows. Process the condensation DAG dependency-first.

A group identity hashes the exact protocol, sorted local scope seeds/relations and sorted dependency-group identities. A scope generation hashes its group identity and scope. A cyclic group's member change invalidates every member and reverse dependent, with no recursive generation hash. A change in an unrelated component must not alter another generation.

The complete ALL observation hash remains fresh operation DATA. It is not inserted into every generation or required to remain equal to a stored receipt. A scope's receipt binds its semantically relevant local owner relation and required external-owner generations. A newly observed unrelated scope must not invalidate unrelated ready generations; complete ALL still proves that no deeper relevant owner was omitted.

The shared installer recipe, explicit isolated linker, backend, environment and layout caps remain unchanged. The new Source-owner identity protocol is a separate canonical generation input. No hash salt, old-generation fallback or old receipt reader is retained.

## Exact Ready Format And References

Replace the old binding receipt with one closed current format, version 2:

- scope and full generation;
- local owner-relation/group identities;
- every installation member with original Source package and stable projected slot;
- owned-only Source bindings, including the scope root;
- byte-sorted external-owner scope/full-generation references;
- a qualified execution-environment fingerprint.

External references use generation identities, not ready receipt hashes, so SCC cycles cannot create circular receipt hashes. Actual receipt FD/name/stamp/content fingerprints are independently observed and held during an invocation.

Ready validation compares every field against the freshly derived complete model, actual material and qualified execution observation. Unknown versions/fields, missing owners and stale references refuse. Existing version-1 payload stays physically retained; it is not parsed as current or upgraded.

The actual graph/current-binding authority includes the relevant owner relation, generation references and receipt fingerprints. It must not infer that each install member owns a Source transition. No new writable context or historical authority is derived from these records.

## Runtime Dependency And Peer Barrier

An export to a real Source package executes from that Source path and therefore uses its designated owner node_modules. Its requesting scope's staged w/index is not its execution environment. This semantic rule is explicit in the ready model and tested with actual Generated package imports.

Before any ready receipt or Source transition, the fixed producer observes each shared package's actual installed projected dependency environment and its designated owner's projected environment. The observer uses genuine funded readonly custody, bounded nofollow descriptors and exact current installer/lock/input lineage. It executes no package code. It resolves only declared development/runtime/optional/peer dependencies using the isolated produced directory/link graph; a workspace edge is resolved through the exact current Source-owner relation, never by following a stale Source node_modules.

The closed execution graph records package name/version, complete locked artifact identity/integrity/patch/platform selection, relevant package resolution metadata (including exports/imports/main/module/type), typed dependency edges and peer providers. Unknown package/lock forms, ambiguous providers, undeclared resolution requirements, foreign links, missing mandatory dependencies or unqualified export/bin subpaths refuse. This is not a name/version-only comparison.

The readiness claim is narrowly named declared-provider-identity. Every declared runtime/development/optional/peer edge relevant to shared Source execution must refer to the same canonical provider instance in both contexts. Equal versions, hardlinked bytes or equal complete artifact graphs in distinct module locations do not establish this. Artifact graph equality remains diagnostic DATA only. Optional absence must be explicit and consistent.

Peers use the same strict provider-instance rule, not merely equal versions or hardlinked bytes. Distinct same-version peer copies can create distinct singleton/module identities. A mismatched mandatory peer, one-sided optional peer, version mismatch or distinct peer provider refuses before readiness. Workspace peer providers can be the same exact designated Source package. No automatic dependency override or peer rewriting is introduced.

The supported closed resolution profile fingerprints actual Bun/Node-compat runtime, platform, architecture, supported exports conditions and exact resolved canonical owned links. It covers declared bare-package and admitted workspace edges only; dynamic paths, undeclared resolution requirements, unknown conditions and unsupported lock forms refuse rather than imply general Node compatibility. A distinct fixed fresh-provider canonicalization command could redirect equivalent newly produced links to one real provider, but it is not implemented or inferred by this model. The current runtime may safely refuse before readiness until such a gate is qualified.\n\nThe full graph/metadata is bounded by existing receipt bytes, installed-entry/depth/work/path limits. Cycles use finite graph identity rather than recursive expansion. Private physical observation fences remain distinct from the normalized semantic hash and are rechecked after every later producer phase. Installed path qualification alone does not prove these semantics.

Source tests must compare the actual first-party environment reader and comparator with independent SQLite graph decisions and real Generated Bun/Node package resolution/import behavior, including conflicting peer versions and equal-version distinct singleton providers. No real installer runs in those tests. Linux/Windows physical semantics remain independently unqualified.

## ALL And Requested Subsets

A request selects install scopes only after complete ALL relation derivation. Compute its transitive required owner closure.

For ALL sync, all selected scopes and referenced owners are produced together and every required generation is installed, exported, execution-qualified, path-qualified and durably ready before the first Source binding. SCC members are admitted as one readiness group.

For --scope B, an owner outside the requested set is not silently installed or rebound. It must already have an actual current qualified ready generation and current Source bindings, under the same actual whole funding/global binding custody and complete ALL/input observation. Missing or stale external readiness refuses before selected staging or Source effects. The current command must not claim external readiness from a stored hash alone.

Consumers acquire global binding SH and complete ALL ownership, derive all current plans, and hold the requested generation plus required owner closure through caller work, accepted work drain and final Source/receipt/input rechecks. Writers retain global EX. Existing genuine controls, cancellation, lifetime and lease ordering remain strict. Subset consumers cannot use an owner relation produced by a prior command as authority.

## Bounded Compositional Staging

Shallow lock header DATA suggests more than 512 projected named manifests across all scopes; it is not a fresh ALL count. Keep the existing 512-file per-Core-batch and 64 MiB total byte limits.

A distinct fixed Core group seam is needed under the same genuine Original, whole funding token, global EX and ALL lifetime: at most 64 disjoint canonical roots, at most 512 files per root/batch, total bytes across the group at most 64 MiB. Capture and validate the complete immutable Source model and every root's absence before the first effect. Each batch uses actual mapping/journal custody and privately issued nominal receipts. The group owns all receipts and accepts no caller writer/provider/callback. Installed work may begin only after every batch is freshly verified.

No renewing Original, per-batch 64 MiB multiplication, staged member omission, existing-root adoption, hash variation or public reservation/image fabrication is permitted. The Domain must call an actual qualified Core group API; no placeholder/fake implementation can stand in while that seam is pending.

## Strict, Preserving, Unready And Retirement

Probe keeps metadata-only Source observation and no Source binding/ready claim. Its membership plans retain every member while endpoint observation becomes unique-owner based.

Strict completion still admits only absent/link Source leaves. Preserving completion still requires exact current leaf/ancestor/target/intent proof before its Generic capsule transition. Unready rebuilding still refuses any selected ready entry and any complete declared endpoint reference into a selected root. The complete relation now includes unique owner destinations and all external-owner references; noReady still does not mean inactive. These purposes share the one current model, never a weaker Probe model.

CurrentBindings requires actual qualified current owned bindings and every relevant owner receipt. A generation referenced by the current required owner closure cannot be treated as inactive merely because it has no borrowed-member Source alias. Selected retirement retains global EX, complete ALL/input/current ready proof, exact candidate payload CAS and whole charge. Old or unknown receipt formats remain unqualified; no migration/legacy reader or stale membership inference is added.

Partial failures retain every produced root, ready record, capsule and whole allocation. Reports distinguish completed accepted transitions, in-flight uncertainty and unknown outer prefix counts. Arbitrary undefined/frozen primaries and distinct close failures remain exact. No rollback, broad recapture or prefix-based adoption is introduced.

## Implementation Map And TDD

Production changes remain in existing files:

1. Dependencies storage install/bindings schemas and neutral corpora: owner relation, SCC/group identity, new exact ready format, environment graph, subset closure and completion/refusal cases.
2. Existing storage owner: pure relation/SCC/identity codecs; complete all-scope captured input plans; owned bindings and external references; actual shared environment observation; strict/probe/preserving/current/consumer/retirement readers.
3. Existing ordinary Dependencies owner: complete ALL authority for scope requests, external-ready preflight, current projection and fixed all-ready ordering; no new arbitrary privileged runner.
4. Existing explicit repository owner: all plan/Probe/strict/preserving/unready/retirement callsites adopt the same relation; fixed execution and group-readiness barriers; preserve unrelated Graph/Tools prefixes.
5. Existing Core explicit publication owner: separately coordinated bounded staging group, preserving each existing command's finite limits and actual authority.
6. Existing reading/main tests, runtime fixture helper and tooling diagnostic helper/schema: exact production-body tests, neutral Ajv/SQLite/graphlib/Python oracles, genuine controls and lifetime counterlaws; generic EN/DE launch route tests for all modes.

The first semantic RED must demonstrate that the current conflicting multi-scope projection cannot produce the new unique owner plan/receipt, not merely a fixture parser failure. Required countercases are in dependency-source-owner neutral inputs. Current old strict duplicate refusal is retained as historical evidence. No actual Native operation, installer or Source binding transition is performed by agents.

Implementation proceeds only after independent review of this complete contract, including peer semantics, compositional staging and lifetime.

## Prospective Inputs And Read-Only Checks

The ticket inputs dependency-source-owner/🧬️schema.json and dependency-source-owner/🔣️cases.json contain 16 owner/closure cases, 6 identity invalidation cases, 22 declared-resolution/provider cases and 23 orchestration/refusal cases. They are explicitly authority=false and executed=false. Ajv validated their structure. An independent SQLite recursive-closure/mutual-reachability oracle validated the 16 owner assignments, preserved membership counts, SCC partitions and requested owner closures; Generated/shared-source-owner-design-validation.json records that limited result. No production feature body was executed. The oracle's first two development failures were harness errors (Python dirname of a bare manifest and the special root-dot ordering), not semantic RED evidence.

Source-only shallow text lock headers currently report 121 root workspace rows, 296 mit-bestand, 300 hub, 292 s and 122 teaching. A guessed framework lock was absent. These are authored lock DATA, not current complete ALL membership or an installer result. They explain the compositional staging concern without authorizing any root or discarding any member. The current 512-file per-batch and 64 MiB total bounds remain unchanged.

Root accepted SCC-local identities and the conservative provider-instance barrier. Distinct copies with equal graphs remain unsupported, including ordinary dependencies; a separately qualified fixed fresh-provider canonicalization may be necessary before actual repository completion can pass. No current runtime pass is implied.
