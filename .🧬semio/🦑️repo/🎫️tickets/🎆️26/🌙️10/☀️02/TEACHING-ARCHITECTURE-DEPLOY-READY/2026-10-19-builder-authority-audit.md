# Container Builder Authority Audit

Read-only source audit, 2026-10-19. No Docker invocation, image pull, allocation, installer, compiler, Git mutation or source edit was performed. The release worker owns bounded container probing.

## Conclusion

The macOS ARM64 host alone does not establish that a genuine Debian 12 x64 container route is forbidden. No Rosetta/QEMU/emulation restriction was found in the reviewed process/native-storage or repository build contracts. An x64 Bun process actually executing inside Debian 12 would report the facts that preflight tests; no architecture override is necessary or authorized. This is conditional source admission, not evidence that the current Docker setup passes or that a deliverable can be built.

The current storage figures supplied by the coordinator do not prove sufficient backing: guest free 766 GiB, Docker raw physical free approximately 55 GiB and host free approximately 194 GiB describe different layers. In particular, virtual guest free space must not be presented as already available physical backing. The audited Linux guards see the guest filesystem and actual loop file allocation, but do not inspect the enclosing macOS Docker.raw file or APFS host capacity. This limitation prevents a claim that preflight success alone certifies all backing layers. No capacity metadata should be altered to make the route appear admitted.

## Authored Admission

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🟦️.ts:29–54` compares actual `process.platform` and `process.arch`, `/etc/os-release` ID/version, `/proc/self/status` CapEff bit 21 (CAP_SYS_ADMIN), workspace `statfs.bavail*bsize`, writable character `/dev/loop-control`, and at least seven writable unused block loop devices observed through sysfs. Executable discovery checks actual PATH files with X_OK.

The course project metadata requires Linux x64, Debian 12, 77,309,411,328 available bytes (72 GiB), seven free loops, and fallocate/mkfs.ext4/mount/umount/losetup/cc/pkg-config/docker. Preflight explicitly describes observations as insufficient to grant compiler authority. A passing bounded probe would establish these observations only.

`🏗️build/📥️caller/🟦️.ts` issues the actual CLI's original invocation, workspace identity, selected task/arguments, native artifact directory, transport/child/network grants and mode. A real managed CLI must create and retain those grants in the guest; host-issued runtime objects cannot simply be relabeled. No reviewed branch rejects a container or translated x64 process by virtualization identity.

## Exact Linux Backing

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:171,212–222` uses actual fallocate, mkfs.ext4 and an ext4 loop mount with nodev/nosuid. Preparation independently requires actual CAP_SYS_ADMIN and parent filesystem free space above maximum physical image bytes plus 64 MiB. A fresh backing must have exact authored logical size and allocated `blocks*512 >= size`, bounded above by maximum physical bytes. A sparse replacement fails this check.

Observation requires a regular, non-symlink, single-link backing file, matching size/allocation and actual mountinfo/sysfs loop mapping: real backing path equality and zero offset/sizelimit. Receiver checks additionally retain file device/inode/size, mount/root identity, filesystem geometry and immutable receipt identity (lines 110,125). These rules require real usable loop devices and mounts inside the selected namespace; an environment variable or detached receipt does not supply them.

The guards can therefore admit real fixed ext4 image storage inside a correctly configured container. They do not themselves prove that Docker Desktop's enclosing sparse disk can fulfill the required allocation. Given the explicit prohibition on large allocations in this audit, that question remains unverified.

## Source and Binding Isolation

`⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:148,472` includes actual OS/architecture/runtime in dependency generation and runtime identity. macOS ARM64 ready dependencies cannot be assumed to certify Linux x64 execution.

The normal managed dependency producer captures each Source endpoint and ancestry, then publishes exact dependency directory bindings under writer custody (`:399–405,444–455`). Thus a read-write bind of the shared checkout is not harmless merely because compilation occurs in Docker: normal bootstrap may replace its node_modules endpoints with guest-specific targets. A read-only shared checkout prevents those writes but does not, by itself, establish a working managed dependency route.

An admissible route must expose exact current source to the managed CLI while making its writable Source dependency bindings, cache/leases and artifacts genuinely isolated from the active checkout. It must preserve actual source provenance, satisfy current ownership checks and stay within the 256-character path requirement. This audit did not establish an existing authored isolated source facility or validate an isolated container filesystem view; it does not propose bypassing endpoint checks, sharing host node_modules, changing source capabilities or copying source into the ticket.

## Remaining Evidence

The bounded guest probe can settle actual x64 runtime/ABI, capabilities, devices, tools and guest filesystem observations without compiling. Separately establish honest usable backing capacity and an authorized isolated Source/binding view before any managed preparation. The current evidence supports further bounded investigation, not an assertion that Docker is impossible, and not permission to launch a large allocation or compiler.

## Followup: Existing Authored Source Facilities

There **is** an authored physical Source snapshot mechanism; the earlier statement that no isolated view was established must not be read as saying the repository has no such API. `🏗️build/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts` exports `withSelectedNativeSnapshotV1(workspace, request, original, phase, body)`. Its closed request is a complete finite list of relative path/digest/byte triples. It copies exact current no-follow, single-link source files into a graph-store-funded `ns/<slot>/s` root, fsyncs files and metadata, retains a slot writer and source reader, and rechecks all original source inputs before callback settlement. The policy bounds this to 16 MiB, 8,192 files and 256-character paths. Cleanup removes its owned snapshot after the child settles; this is an existing authored source copy outside the ticket, not a suggested ticket copy.

Its receiving API `withOriginalSelectedNativeSnapshotV1` authenticates the original invocation, graph allocation, digest/grant, snapshot initialization marker, actual source and snapshot bytes, reader lease and callback lifetime. `selectedNativeWorkspaceRootV1`, `selectedNativeWorkspaceInventoryV1` and `requireSelectedNativeFileCoverageV1` expose only that admitted Nx inventory. Setting `SEMIO_NX_NATIVE_SOURCE_SNAPSHOT` alone is insufficient: receipt/allocation/current-byte checks still run. This is real provenance enforcement, not a detached manifest-only mechanism.

The existing orchestrating caller is `🏗️build/📥️inputs/🎯️selected/🦀️rust/🟦️.ts:29–34`. It requires the already published pinned graph, supported run/run-many/affected selection, no watch/daemon, literal supported Rust unit inputs and no unsupported task. It falls back to the full route for unsuitable inventories and paths. This facility serves the selected native Nx child; it is not a bootstrap workspace producer or general container launcher.

The decisive ordering is `🏗️build/📜️script.ts:129`: `ensureBuildEnvironmentV1` runs before graph loading/selection at `:177`, hence before this selected snapshot can be constructed. Initial dependency sync therefore still observes and publishes Source endpoints in the CLI's actual original workspace. The snapshot root hook is used by Nx authorizer/inventory integration, not by the JavaScript dependency producer's workspace argument. No reviewed existing API redirects original-entry dependency bindings into the selected snapshot.

The other relevant mechanisms do not bridge that gap:

- `🗂️workspaces/🟦️bun/🟦️.ts` exposes `withCurrentBunWorkspaceOwnershipV1` and `withPublishedBunWorkspaceOwnershipV1`: controlled current membership/topology custody and rechecks, not workspace materialization or namespace isolation.
- `🏗️build/📥️inputs/📂️membership/🟦️.ts` exposes `withBuildFileMembershipV1`: no-follow current directory inventory and identity rechecks, not copied contents or a writable private root.
- `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:128–148,304–308,365` already stages exact installer manifests, locks, configuration and patches via the immutable artifact input-group contract. It maps package manifests into `w/<slot>` and strips install-only fields; this is intentionally an installer input projection, not the complete source tree or compiler workspace. Its binding plans still set `sourceDirectory = resolve(workspace, source.path, "node_modules")`, then publish there (`:306,454`).
- `⚡️caching/🦀️cargo/📦️units/📥️inputs/🔏️snapshot/🟦️.ts` compares complete expected and observed path/digest/byte rosters; it does not materialize or launch a workspace. Native Rust Source admission reads current closure/cfg under storage custody, likewise without producing an isolated bootstrap root.
- Normalization's repository-capture API inventories tracked/untracked candidates and retained source bytes for its mutation/analysis owner; it is not a managed build workspace launcher.

Accordingly, existing code supplies exact reusable selected-compiler snapshot and installer-input staging contracts, but this audit establishes **no existing end-to-end original-entry route** that starts managed guest dependency preparation on a private view while keeping shared checkout bindings untouched. This is a bounded source finding, not proof that an actual isolated filesystem view cannot satisfy the contracts. A proposed overlay/private view still needs its real current-file provenance, workspace discovery, Source endpoint writes, leases and backing verified; neither the existing snapshot token nor membership data automatically grants that route.

## Bounded Early-Preflight Entry Finding

For the release worker's source-streaming investigation, the smaller existing authentic entry is `🏗️build/📜️script.ts` itself: its `import.meta.main` branch calls `runBuildCliV1(findWorkspaceRoot(import.meta.dir), process.argv.slice(2))`. Passing `repo preflight --scope @teaching/architecture-course` enters its existing Repo router without a custom wrapper or fabricated invocation. It creates its own original envelope/capabilities and issues normal repository providers. This avoids loading the root router's unrelated lazy declarations while retaining authored entry semantics.

`runOwnedBuildInvocationV1:91` returns through preflight **before** its leases/diagnostics stores are opened at `:92`, and before `ensureBuildEnvironmentV1` in the later build route. Therefore a source-only, isolated early-preflight probe need not install dependencies or allocate fixed stores if the actual entry import/runtime closure can be supplied intact.

Root discovery requires authentic paired root `📋️project.json` and `package.json`, with the real package workspaces declaration (`process/🧭️routing/🟦️.ts:94–108`). Preflight additionally reads its authored registry/schema and the registry's actual architecture-course owning project metadata. CLI policy, caller schemas, provider designation/schema and imported framework/repository source remain part of the real entry closure; do not replace them with reduced stand-ins. Dynamic selected-route imports and runtime JSON reads must be included as well as static runtime imports/reexports. Type-only imports do not themselves constitute executed dependencies.

The existing Flow cold-bootstrap test uses TypeScript's parser to examine that specific owner's top-level import declarations and then loads the real synchronous owner. It proves that owner's deliberately tiny system/JSON closure; it is **not** an existing generic transitive CLI copier or container workspace launcher. This audit does not certify a complete preflight file roster or execute a streamed guest probe. A bounded manifest of actual file hashes plus direct streaming can establish the worker's exact imported inputs without ticket source copies, but a partial-closure refusal remains a probe failure, not permission to fabricate missing metadata or original capabilities.

Even success would establish only real early preflight observations in the private reduced workspace. It would not establish complete course source membership, compiler provenance, full-workspace dependency binding isolation or enclosing physical storage admission for a later build.

## Streamed Probe Implementation Review

Read the ticket's current `📜️script.ts`, stream input and manifest after the transport correction was authored. No concrete blocking defect found in the reviewed bounded early-preflight design. No Docker command or probe was executed by this auditor.

The script obtains the entry's closure through existing `cacheInternals.relativeScriptInputs(..., requiredSources=true)`, explicitly includes actual root markers and the course owning project, and retains a path/size/SHA-256 manifest (817 files, 7,051,164 bytes at inspection). Source files must be ordinary non-symlink files with bounded paths, with 8,192-file/32-MiB capture limits. Before and after the guest attempt it rechecks current source identity while reading and compares every file's content digest to the capture.

The corrected packer streams actual source directly through Python tarfile; it writes no source-copy tree or archive into the ticket. A local streaming tar inspection requires an exact unique regular-file roster and matching size/digest for every source, the Bun binary and manifest before Docker is started. The guest source hash gate checks every captured source before CLI execution. Precisely, the binary hash is checked by local archive validation; the guest hash loop covers source rows, not a separate binary digest assertion.

The actual package.json repo script invokes the canonical build entry with `repo`, so `./bun repo preflight --scope @teaching/architecture-course` uses the real authored entry, original capability creation and early branch reviewed above. No fake invocation, platform/capacity override, dependency command or compiler command is present. Guest debug observations use actual process platform/architecture/version, os-release and statfs. No host bind is passed: source/executable enter `/repo` in the disposable container's writable layer, `/tmp` is a private 16-MiB tmpfs and the transpiler cache stays there. Cache/lease paths, if unexpectedly touched before refusal, remain container-private. Networking is disabled, image pulling refused, and resource/time limits plus forced named-container cleanup are supplied.

The retained plan enforces conservative input 181,370,615 bytes and temporary 356,633,687 bytes against 256/512-MiB bounds. These are authored accounting bounds, not a new measured host physical reservation. The one earlier bind attempt failed before guest entry (Docker125); the first stream transport failed before managed entry. Corrected source/transport inspection here is not evidence that its next invocation ran or reached preflight. The release worker owns local archive execution and the final actual guest outcome. Early preflight remains a bounded environment observation, not a release artifact or full compiler admission.

## Host and Guest Capacity Followup

The release worker subsequently reports actual Debian12/x64/Bun1.4.2 original preflight execution, refusing only cc/pkg-config/docker. Its updated source capture is 924 files/7,303,397 bytes, locally validated as 926 archive entries; guest source and post-host checks passed. Those execution claims belong to `2026-10-19-builder.md`; the preceding implementation-review counts are dated inspection values, not the final roster. This auditor did not run that probe or provision the newly authorized private OS tools.

The precise native capacity contract is scoped to a real enforcing backing, not a whole-machine free-space number. `storage/📜️journal/🟦️.ts:45–51` defines NativeStorageBackingV1 as identity/root/maximumPhysicalBytes; reserved, committed and retiring allocations all count equally. Its reservation transition refuses a charge exceeding remaining capacity (`:60–62`). This journal is meaningful only with the actual live storage port, identity checks and mounted finite image. A host statfs sample is not itself a reservation or that port.

`storage/🌐️platform/🟦️.ts:21–35` selects the creator from actual process.platform and privately registers its canonical image in a WeakMap. The funded-child API `:64` requires that real registered parent and its issued control, computes the finite funding charge, and dispatches to the **same invoking platform's** creator. The funding DATA reader (`storage/🌱️birth/🟦️.ts:25–32`) explicitly does not grant provider authority. These are parent/child allocation mechanisms within retained physical custody; they do not serialize a macOS physical-image capability into a Docker/Linux worker.

No existing macOS APFS/Docker.raw whole-host reservation API or authentic Mac-parent-to-Linux-private-worker funding handoff was found in the reviewed native storage, platform and funding owners. The macOS image creator uses actual hdiutil/mounted-device identity and its own physical owner; the Linux creator uses actual guest parent statfs, fallocate and exact ext4 loop identity. Merely creating a macOS reservation and passing its JSON into Linux would not satisfy either platform's private registration/custody checks. No such workaround is proposed.

This is also a limit on what can be called an authored requirement: the reviewed Linux guard does **not** explicitly require a second reservation of the entire Docker.raw/APFS outer layer. It enforces guest filesystem headroom and fully allocated inner backing as described above. Absence of a cross-host API does not establish a policy ban on virtualized Linux, and an outer reservation must not be invented as an existing source condition. An actual Linux fixed Store could be admitted by the existing guard if its real creation/mapping checks pass; no such allocation has been authorized or demonstrated by this audit.

The supplied host free approximately194GiB, Docker.raw approximately55GiB physically allocated/926GiB logical, and guest free818,640,089,088 bytes therefore support only bounded headroom observations at different layers. In particular, 55GiB is the raw file's allocated size, **not 55GiB of physical free capacity**; the earlier approximate 'raw physical free' wording above must be read with this correction. Guest capacity cannot be used as proof of outer host reservation. Conversely, the194GiB host sample cannot prove an exclusive future guarantee against other writers.

For the requested stronger whole-host physical-backing proof, existing guest guards plus host measurement alone cannot issue an authentic reserved cross-layer capacity witness. That specific guarantee is unsupported by the reviewed handoff APIs. The source-supported path remains genuine guest-native finite Store creation under its original Linux owner and unchanged checks, with honest independent host headroom accounting; claiming an exclusive outer reservation would require an actual supported mechanism that this review has not found. No allocation, Docker setting, compiler, dependency install or new authority was created here.

### Exact Repository Peak Envelope

The current `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` declares disjoint physical partition envelopes: rust16GiB, tools8GiB, graph4GiB, dependencies8GiB, products24GiB, diagnostics2GiB, nx1.5GiB and leases0.0625GiB. Their sum is **63.5625GiB (68,249,714,688 bytes)**. The repository ceiling is64GiB (68,719,476,736 bytes), with1MiB bootstrap control allowance and468,713,472 bytes remaining policy headroom. `readRepositoryCacheBudgetV1` validates the total and disjoint paths; `filesystemBytes` is each physical capacity minus1MiB. Logical publication limits, including tools64GiB, are observation limits and must not be added as separate physical backing demands.

This is the declared whole-repository managed Store envelope, so it is a conservative upper bound for these eight stores even when a particular course command opens fewer. Child allocations funded inside these parent images are accounted within parent capacity and must not be counted a second time as independent host space. The policy does not cover unrelated Docker images/layers, a full private source tree, OS tools, exported release artifacts or unrelated host writers; those require separate actual accounting for a full attempt.

Against the supplied approximately194GiB host free sample, the entire64GiB ceiling leaves approximately130GiB before those other demands. This arithmetic supports available headroom at the sample time; it is neither an exclusive reservation nor a full-build measurement. The earlier72GiB preflight threshold includes more margin than the eight-store sum, but does not prove external host accounting itself.

A new actual Linux original can fund its **own** guest-native stores through the existing admission path. It need not borrow a macOS parent allocation and therefore needs no Mac-to-Linux handoff. No explicit outer-Docker.raw reservation condition was found or is imposed by this report. The outstanding limit is honest cross-layer capacity observation/accounting, alongside the actual unchanged guest create/mount/custody checks—not an unsupported extra blocker or virtualization prohibition.

### Current Course/Proctor Demand Table

The latest host sample supplied by root is191,592,064KiB =196,190,273,536 bytes =182.716430664GiB. It supersedes the earlier194GiB observation.

| Top-level backing | Maximum physical GiB | Relationship |
| --- | ---: | --- |
| dependencies `d` |8|Installer generations live inside this image|
| tools `t` |8|Rust/Node/tool SDKs share this image|
| graph `g` |4|Graph and selected-source snapshots inside|
| diagnostics `o` |2|Logs/captures inside|
| products `p` |24|Published artifacts/state and funded child volumes inside|
| leases `l` |0.0625|Retained alongside other partitions|
| Nx `n` |1.5|Nx state/outputs inside|
| Rust |16|Compiler workspace/artifact capacity inside|
| Total |63.5625|All may persist simultaneously; use64GiB policy ceiling conservatively|

`⚡️caching/🔣️policy.json:495–501` declares Rust16GiB physical,16GiB-minus1MiB filesystem,4GiB maximum compiler workspace and1GiB maximum artifact charge. `repositoryCompilerReservationV1` in the compiler owner charges/reuses one4GiB compiler-workspace allocation per build mode inside Rust16; it is not another4GiB top-level image. `withRepositoryCompilerStoreV1` explicitly retains tools8 alongside Rust16. Artifact child volumes are funded within their actual parent partition and cannot exceed remaining parent budget. The exact selected child roster is computed from selected artifact policies at execution; this audit does not invent a fixed extra child peak or claim every child is allocated sequentially.

Serializing commands reduces overlapping compiler work and cold growth, but does not reduce the63.5625GiB sum of already-created persistent top-level images. The managed compiler writer serializes cold growth/maintenance under its live writer lease. Sequential course/proctor gates may reuse those images and charges. Consequently the safe accounting upper envelope for these declared stores remains64GiB rather than summing each command's stores again.

At the current supplied host sample, reserving the full64GiB accounting envelope leaves118.716430664GiB of observed headroom. The authorized private OS-tool temporary maximum1GiB, if simultaneously retained and not already reflected in that sample, leaves117.716430664GiB before private full-source capture, Docker/base layers, release export and unrelated host changes. These remaining quantities must be measured/accounted for by their owners; no source-specific peak is claimed here. The arithmetic is favorable for the declared managed-store demand and does not itself authorize allocations or certify future host availability.
