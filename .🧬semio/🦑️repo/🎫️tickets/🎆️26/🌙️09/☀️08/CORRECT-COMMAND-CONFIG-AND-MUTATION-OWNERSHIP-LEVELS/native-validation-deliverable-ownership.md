# Native Validation Deliverable Ownership

The ticket script previously sent every unrelated native validation command to one cargo-trinity target directory. Runtime evidence shows the broad OS host build held that artifact-directory lock for several minutes while independent Forms, DAG, Terrain, Kit and Layout feedback queued. The repository's current .cargo/config.toml explicitly separates shared build-dir intermediates from target-dir uplifted deliverables and enables fine-grain locking.

The ticket script now assigns uplifted outputs to 🗑️generated/cargo/<validation-command>. The shared build-dir, rustflags, compiler cache and target remain unchanged. Existing CARGO_TARGET_DIR environment overrides still take precedence. Already-running processes retain their original directory; new invocations use command-owned deliverable directories. No cache was deleted or moved and no build-dir override was introduced.
# Superseded Native Attempts

The current root native attempts Forms9, DAG9, Stdio4, Terrain8 and Curation1 also remained idle on overlapping shared Cargo units for six to nine minutes. Root cancelled those five verified own Cargo processes (14287, 14285, 14290, 14280, 16626) and retained only its targeted SDK transient20 attempt (12275). Agent Layout and recursive SDK runs were preserved. These root laws must be requeued sequentially after the current SDK attempt returns; cancellation is not a compile failure or a test pass. Shared Cargo configuration and cache remain unchanged.

Root Forms contract8 and DAG contract8 were still idle after nineteen minutes while newer contract9 commands were already queued. Their Cargo processes (2739/2773) were explicitly cancelled after read-only command identity verification. These were root-owned superseded attempts, not other logical agents. Current contract9 attempts remain active. No cache, worktree, source or Git state was removed.

The four root launch registrations `.182` through `.185` were being lost from the generated `.vscode/launch.json`. Their canonical seed is `.vscode/🧩️launch.seed.jsonc`. Both files now contain the exact existing sparse-retirement and Curation oracle/native Nx targets. Future registration edits must update the seed and generated launch surface together.
