# Standalone Journey Lock Readiness

The journey package deliberately declares its own empty `[workspace]` in `dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml`. Its dev dependencies are portable-pty, vt100, serde_json and the declared first-party fixture reader. Its Cargo.lock remains a required authored prerequisite for locked journey/load gates. No lock was synthesized or removed during this review.

The canonical native Cargo launcher schema already classifies `generate-lockfile` and `update` as closed `lock` operations. Its actual run branch uses `withRepositoryCargoResolutionV1` before the ordinary dependency-input/fetch path, and therefore does not first demand the absent locked graph. The genuine resolution view retains Original network grant, physical registry writer custody and canonical manifest selection. No new generic admission bypass is needed on current source.

The exact retained plan step `journey-workspace-lock` invokes native `repo cargo --store=shared --configuration=prototype -- generate-lockfile --manifest-path <authored journey manifest>`. It must run only after explicit native slot release; the actual command terminal and resulting independent lock inventory remain pending. A first-host missing dependency/cache result must be handled through the same granted acquisition route, never by dropping --locked from compilation.
