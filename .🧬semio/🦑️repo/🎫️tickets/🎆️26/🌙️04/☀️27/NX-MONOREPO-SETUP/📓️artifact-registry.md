# Artifact Registry and Storage Accounting

## Contract

The registry derives deliverable owners from Nx target output declarations and classifies shared native/dependency/tool/report stores separately. Each entry declares retention, budget group, cleanup handler and lease coverage. Pending lease integration is explicit; a declaration alone never permits deletion. Namespace stores can contain more specific owned outputs without being treated as competing producers. Distinct producers claiming identical or nested outputs are violations.

The first implementation is read-only. Disk measurements share one inode set across all owners, assign nested paths to their most specific declaration, count symlinks without following them and reject symlinked parent paths. Cancellation and progress are part of the measurement contract. JSON Schema validates language-neutral ownership/path/link fixtures; native Python filesystem accounting supplies an independent byte/count oracle. Source movement during measurement must be reported as incomplete rather than silently swallowed.

This establishes local workspace accounting. User-level stores, external environment overrides, container state, full runtime consumer provenance and task-lifetime lease integration remain follow-up gates.

An inactive private fixture daemon log retained 494,619,567 bytes after its prior run. With no open descriptors confirmed by `lsof`, the final 256 KiB was retained and the obsolete full generated log was removed. No shared cache or active process state was removed. The ticket directory accounted for approximately 1.71 GiB before reclamation, so it cannot explain the remaining machine-wide disk pressure.

## Native Red/Green Checkpoint

The first schema fixture needed its own absolute `$id` for the installed jsonschema resolver; after that fixture repair the baseline failed on the missing registry implementation. The implementation passed the native Python unique-byte/file oracle, link exclusions, duplicate-owner normalization, unsafe path vectors and cancellation checks. Additional red cases caught a case-insensitive owner collision and a symlinked declared output root; both now pass after their repairs. The current baseline inventory contained 812 deliverable declarations, with zero identical/nested ownership conflicts. The live Nx disk-report and full repository regression are running.

The registry is integrated into `repo:audit`, `repo:artifact-check`/`policy-check`, and `repo:disk-report`. The existing editor commands still select those targets. Explicit direct target prerequisites supply initial known consumers; wildcard, transitive and runtime-only consumer provenance remains incomplete. Cleanup stays disabled for deliverables until producer, reader and Nx restoration leases participate.

The final native fixture passed after also rejecting cache-root/native-store output claims. Windows allocation is represented as unavailable (`null`) because portable Node file metadata does not establish allocated bytes there; apparent-byte and unique-file accounting remain available. Windows/Linux native qualification remains outstanding. The initial full repository regression had already exited 1 when a stop was considered; no process was signalled. Its exact failure is being inspected before retrying the completed source.

The earlier full run failed after 2m01s on the newly added broad cache-root rejection vector: its process had loaded the earlier registry implementation before that final fixture was added. The completed implementation passes that vector in the focused native run. A fresh `repo:test` invocation is now running against the final registry source (`artifact-registry-final-repo-tests.log`).

The consistent-source rerun exited 1 after 12.3 seconds because a concurrent taxonomy change moved fixture schemas from `🧬️schema` to `🛂️schema`. Three remaining dynamic fixture-path reads (continuous services, Nx bootstrap and the new artifact registry fixture) were changed to their verified current paths. Domain schemas retain their current `🧬️schema` location. No removed schema was recreated. A fresh full rerun is recorded in `artifact-registry-schema-repo-tests.log`.

## Live Nx Disk Measurement

`repo:disk-report` exited 0 after 10m29s. It enumerated 285,112 unique files, recognized 44,609 further hard links and excluded 282 symbolic links. The declared workspace scopes retained 231.05 GiB of allocated space; all visited reads completed without a reported error and the inventory had zero ownership conflicts. The run began before the final additional rejection vectors; those separately pass on the completed implementation.

| Owner                     | Category         | Allocated GiB |
| ------------------------- | ---------------- | ------------: |
| cargo                     | compiler-state   |        170.06 |
| cargo-browser             | compiler-state   |         38.98 |
| agents                    | service-state    |          6.40 |
| repo-cache                | unclassified     |          5.19 |
| bun                       | dependency-store |          4.10 |
| workspace:build-storybook | deliverable      |          2.75 |
| test-oracles              | dependency-store |          1.29 |
| uv                        | dependency-store |          0.69 |

Compiler state totals 209.10 GiB against the provisional 20 GiB group budget. Nx task results occupy 0.08 GiB. These observations identify retained compiler state as the dominant measured category; they do not establish when or why all of it accumulated. No compiler state was deleted. The approximately 5.19 GiB unclassified cache remainder still needs narrower ownership and retention. The report covers registered workspace scopes and does not claim a complete machine/container/user-cache baseline.

The next full rerun reached continuous-service qualification, passed isolated/cancelled modes, then failed in default-sharing. The first consumer attempted `http://127.0.0.1/` without a port and exited 1. The fixture exposed a partially written readiness file because existence was used before its write completed. Readiness now publishes through a same-directory rename; parent waits also report early child exits immediately instead of misreporting them as service-sharing timeouts. This repairs the native test fixture; production session readiness already uses its separate generation protocol. The next full run is `artifact-registry-atomic-repo-tests.log`.
