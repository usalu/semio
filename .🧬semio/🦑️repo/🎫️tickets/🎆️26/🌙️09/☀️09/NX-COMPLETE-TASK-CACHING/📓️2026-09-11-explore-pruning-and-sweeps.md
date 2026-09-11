# Cache Deletion & Pruning Paths - Comprehensive Audit

## Summary
Found 13 code paths that delete or prune directories, categorized by trigger (explicit nx target, automatic git hook, manual cleanup). Most target test/coverage outputs; only 2 paths target build artifacts with size-based gating.

---

## NX Configuration & Settings

### `nx.json`
- **Line 63**: `"cacheDirectory": ".nx/cache"` (default Nx cache location)
- **Line 64**: `"maxCacheSize": "8GB"` (Nx auto-prunes cache to 8GB limit)

Nx enforces bounded cache via `maxCacheSize`; no manual code implements the prune.

---

## Build/Cache Deletion Patterns

### 1. **Root `📜️script.ts`** - sccache Extraction (Line 330)
```typescript
rmSync(extractDir, { recursive: true, force: true });  // Line 330
```
- **Path**: `./.🧬semio/🦑️repo/⚡️cache/extract/` (temporary extraction directory)
- **Trigger**: `setup` nx target (dependency: `deps-tools`)
- **Risk**: Only cleans own temp folder, no in-use cargo target risk
- **Related**: Lines 199, 325–340 (sccache bootstrap)

### 2. **Root `📜️script.ts`** - CMake Cache Purge (Line 15116)
```typescript
rmSync(cacheDir, { recursive: true, force: true });  // Line 15116
```
- **Function**: `purgeStaleCmakeCache(preset)` at line 15107
- **Path**: `./.🧬semio/🦑️repo/⚡️cache/cmake/{preset}/` (CMake cache only)
- **Trigger**: Before every C++ configure (`cpp-configure` nx target → line 15056 call)
- **Risk**: Zero—targets only stale CMake generator cache, not cargo/build dirs
- **Condition**: Only deletes if Windows generator changed

### 3. **Root `📜️script.ts`** - CLAUDE.md/GEMINI.md Symlink Cleanup (Line 411)
```typescript
rmSync(aliasPath, { force: true });  // Line 411
```
- **Path**: Root-level alias files only (CLAUDE.md, GEMINI.md)
- **Trigger**: `setup git` nx target (line 405 call)
- **Risk**: Zero—not build/cache related

### 4. **Root `📜️script.ts`** - Rust Build Temp Extract (Line 104)
```
// File: ./🧰️framework/📦️packages/🦀️rust/📜️script.ts
rmSync(temp, { recursive: true, force: true });  // Line 104
```
- **Path**: Temporary extraction directory (mkdtempSync allocated)
- **Trigger**: Part of Rust package build setup
- **Risk**: Zero—cleanup of temporary files only

### 5. **Root `📜️script.ts`** - Build Artifact Removal (Line 16375)
```typescript
rmSync(dir, { recursive: true, force: true });  // Line 16375
```
- **Function**: Coverage cleanup in `CleanScript.run()` (line 16368 branch)
- **Paths**: Removes coverage report directories:
  - `{repoMetaDir}/📊️coverage/js/`
  - `{repoMetaDir}/📊️coverage/rust/`
  - `{repoMetaDir}/📊️coverage/go/`
  - `{repoMetaDir}/📊️coverage/py/`
  - `{repoMetaDir}/📊️coverage/dotnet/`
- **Trigger**: `bun nx run workspace:clean-coverage` (line 668)
- **Risk**: Zero—only coverage, not build cache

### 6. **Root `📜️script.ts`** - Repo Shard Cleanup (Line 15705)
```typescript
rmSync(join(shardRoot, entry.name), { recursive: true, force: true });  // Line 15705
```
- **Path**: `.🧬semio/🦑️repo/⚡️cache/{shards}🔖️[hash]/` (stale taxonomy shards only)
- **Trigger**: Taxonomy inventory shard publication
- **Risk**: Zero—only removes unreferenced shards

### 7. **Root `📜️script.ts`** - Main Workspace Clean (Line 16700)
```typescript
rmSync(abs, { recursive: true, force: true });  // Line 16700
```
- **Function**: `cleanRemovePath()` at line 16695
- **Paths Targeted** (via `runWorkspaceClean()` at line 17010):
  - **Build artifacts**: Directories named `target/`, `dist/`, `build/`, `out/` **exceeding 10GB** (line 15166: `CLEAN_BUILD_ARTIFACT_MAX_BYTES`)
  - **Gitignored ticket files**: Files under closed ticket folders that match `.gitignore`
  - **Ticket-generated outputs**: `🗑️generated/`, `🧾️runs/`, `🧪️runs/`, `🧾️taxonomy-transaction/` inside tickets
  - **Misplaced files**: Found via `cleanCollectMisplaced()`
  - **Windows illegal names**: Files violating Windows naming rules
- **Trigger**: `bun nx run workspace:clean` OR manual `bun ./📜️script.ts clean` (root project.json line 654)
- **Protection Mechanism**:
  - Skips closed ticket roots (line 17014)
  - Skips directories under `protectedPrefixes` (lines 17015, 17025–17026)
  - Build artifact cleanup **only triggers if >10GB** (line 16980)
- **Risk**: **MODERATE** — builds in-repo (e.g., `target/`) matching the 10GB threshold could be deleted if no protection rule covers them

---

## Workspace-Wide Clean Coverage Inventory

### Constants (Lines 15164–15172)
```typescript
CLEAN_TICKET_FILE_MAX_BYTES = 5 MB
CLEAN_TICKET_DIR_MAX_BYTES = 10 MB
CLEAN_BUILD_ARTIFACT_MAX_BYTES = 10 GB  // ← Gating threshold for build dir removal
CLEAN_BUILD_DIR_NAMES = ["target", "dist", "build", "out"]
CLEAN_CACHE_DIR_NAME = "⚡️cache"
CLEAN_TICKET_GENERATED_OUTPUT_DIRS = ["🗑️generated", "🧾️runs", "🧪️runs", "🧾️taxonomy-transaction"]
```

### Removal Categories (Line 15174)
1. `misplaced` — Files in wrong places
2. `gitignore` — Gitignored files under closed tickets
3. `ticket-file` — Oversized ticket files (>5MB)
4. `ticket-dir` — Oversized ticket directories (>10MB)
5. `build-artifact` — Build dirs >10GB matching `["target", "dist", "build", "out"]`
6. `ticket-generated` — Marker-identified generated outputs
7. `windows-illegal` — Windows-invalid file names

---

## `.gitignore` Cache/Build Patterns (Lines 16–23, 58–59, 92)

```
target*                    # Line 16  — Cargo target dirs
🎯️target/                  # Line 17
🎯️target-*/               # Line 18  — Named variants (target-s-e2e, target-demo*)
.nx/cache                  # Line 59
.nx/workspace-data         # Line 60
**/🤖️generated/           # Line 92  — Nx generated outputs
```

**Ignored Directories**: `.nx/cache`, all `target*` patterns, generated outputs. These are git-ignored and thus candidates for removal by `clean` if not under protected prefixes.

---

## Git Hooks - Micro-Commit Cleanup

### `.git/hooks/post-commit` (Lines 8, 25)
```bash
rm -f "$GIT_DIR"/gkcommittemplate*     # Line 8
rm -f "$GIT_DIR"/compose-micro-commit-*  # Line 25
```
- **Paths**: `.git/gkcommittemplate.txt`, `.git/compose-micro-commit-*`
- **Trigger**: Every commit (automatic)
- **Risk**: Zero—git metadata only, not build/cache

### `.git/hooks/post-checkout` (Lines 8, 25)
- Same cleanup as post-commit

### `.git/hooks/prepare-commit-msg` (Lines 8, 18)
```bash
rm -f "$GIT_DIR"/compose-micro-commit-*  # Line 8, 18
```
- **Risk**: Zero—git metadata only

---

## DevContainer Setup

### `.devcontainer/Dockerfile` (Lines 70, 103)
```dockerfile
rm -rf /var/lib/apt/lists/*   # Line 70, 103
```
- **Path**: APT package manager cache only (not repo-related)
- **Risk**: Zero

### `.devcontainer/post-start.sh` (Line 265)
```bash
rm -f "$SSH_AGENT_SOCKET"   # Line 265
```
- **Path**: SSH agent socket
- **Risk**: Zero

### `.devcontainer/post-attach.sh` (Lines 41, 92, 126 via trap)
```bash
trap 'rm -rf "$temp_dir"' RETURN
```
- **Path**: Temporary download directories
- **Risk**: Zero—cleanup after install

---

## NX Project Targets

### Root `📋️project.json`
| Line | Target | Command | Dry-Run Capable | Risk |
|------|--------|---------|-----------------|------|
| 654–661 | `clean-test` | `bun ./📜️script.ts clean test` | Yes | Delegates to test domain |
| 663–670 | `clean-coverage` | `bun ./📜️script.ts clean coverage` | Yes | Removes coverage only |
| 672–706 | `clean-taxonomy-*` | `bun ./📜️script.ts clean taxonomy {op}` | Varies | Taxonomy shards only |
| 1179–1185 | `purge` | `bun ./📜️script.ts purge` | No | Neo4j only (see below) |

### Test Module `📋️project.json`
| Line | Target | Command | Risk |
|------|--------|---------|------|
| 137–143 | `test-gc` | `bun ./📜️script.ts gc` | Unknown (not found in search) |
| 51–57 | `test-clean` | `bun ./📜️script.ts clean` | Delegates to test domain |

---

## PurgeScript - Neo4j Only (Lines 15144–15160)

```typescript
export class PurgeScript extends Script {
  run(segments: string[]): void {
    if (segments[0] !== "neo4j") { /* error */ }
    // Connects to Neo4j, no filesystem deletion
    console.log("[purge.neo4j] connectivity ok; no_operation.");
  }
}
```
- **Trigger**: `bun nx run workspace:purge neo4j`
- **Risk**: Zero—database only, no cache/build deletion

---

## Auto-Commit Tooling

No automatic cache purge or build cleanup found in:
- `.claude/launch.json` (no deletion patterns)
- Git hooks (only git metadata cleanup)
- Post-commit/post-checkout (no build/cache cleanup)

---

## Risks and Gaps for a Bounded Shared Cache

### 1. **Build Artifact Threshold Not Size-Predictive**
- **Risk**: The 10GB threshold (line 16980) is a *hard limit*. A `target/` directory at 9.9GB is safe; at 10.1GB it's deleted by `clean` without warning.
- **Impact**: If a in-flight build is near the threshold and another build completes, pushing it over 10GB, the first build's `target/` is eligible for removal even if in-use.
- **Mitigation**: Requires explicit protected path registration or smaller threshold + graduated cleanup (e.g., LRU age-based, not size-based).

### 2. **No Per-Lane Isolation**
- **Risk**: All lanes share same `.🧬semio/🦑️repo/⚡️cache/` directory (`.gitignore` line 38).
- **Impact**: A `bun nx run workspace:clean` in one lane can delete cached files (taxonomy shards, CMake caches) used by concurrent lanes.
- **Mitigation**: Namespace cache directories per lane (e.g., `.🧬semio/🦑️repo/⚡️cache/lane-{id}/`) or guard cleanup with lane-specific exclusions.

### 3. **Build Dir Glob Overbroad**
- **Risk**: `CLEAN_BUILD_DIR_NAMES = ["target", "dist", "build", "out"]` matches at *any depth* (line 16972 `cleanWalkDirs`).
- **Impact**: Unrelated projects using `build/` or `dist/` as artifact dirs (if >10GB) get swept.
- **Mitigation**: Restrict to project root only, or whitelist known safe patterns.

### 4. **Cargo Target Not Honored**
- **Risk**: `CARGO_TARGET_DIR` env var overrides cargo's default `target/`. Cleanup at the repo root won't find `target/` if it's symlinked or on a different volume.
- **Impact**: Private target dirs (e.g., `/private/tmp/cargo-target`) are never cleaned, staying unbounded.
- **Mitigation**: Query `cargo metadata --format-version 1` to find actual target dirs, or require targets to be under `.🧬semio/`.

### 5. **No Active Build Detection**
- **Risk**: `clean` does not check if a `target/` or `dist/` dir is *in-use* (open by rustc, cargo, vite, etc.).
- **Impact**: Deleting an in-use target dir mid-build can orphan lock files and cause cascading failures in concurrent builds.
- **Mitigation**: 
  - Check for lock files (`target/.cargo-ok`) or active processes (lsof)
  - Use generation markers (e.g., `.generation` file touched on each build) and only delete stale generations
  - Require explicit flag for non-dry-run removal of large dirs

### 6. **Cache Directory Not Scoped**
- **Risk**: `.nx/cache` at repo root is global; Nx's own `maxCacheSize` limit is repo-wide, not per-lane.
- **Impact**: One lane's cache invalidation can cascade to others if they share the same `.nx` directory.
- **Mitigation**: Lane-qualify cache dir (`.nx/cache-{lane-id}`) or mount private caches.

### 7. **No Visibility into Shared Cache Operations**
- **Risk**: `clean` runs silently. Deleting >10GB of build dirs leaves no audit trail of *why* or *what* was removed.
- **Impact**: Debugging cache-related build failures is opaque.
- **Mitigation**: 
  - Write audit log (timestamp, path, bytes, triggering command)
  - Use `disk-report` / `disk-prune` nx targets (found in script line 1179, but unimplemented) to expose cache state
  - Implement `disk-prune inspect` to preview what would be deleted before running it

### 8. **Taxonomy Shard Cache Volatile**
- **Risk**: Line 15705 deletes unreferenced taxonomy shards at publication time. If publication fails mid-flight, shard deletions are not rolled back.
- **Impact**: Partial taxonomy state, possible reconciliation errors.
- **Mitigation**: Transactional shard publishing (atomic swap of manifest) or staged cleanup (defer shard deletion until next publish succeeds).

---

## Recommended Actions for Bounded Shared Cache

1. **Implement `disk-report` and `disk-prune` nx targets** (currently missing) to expose cache state and controlled cleanup
2. **Add lane-qualified cache namespacing** (`.🧬semio/⚡️cache/lane-{id}/`, `CARGO_TARGET_DIR=...lane-{id}...`)
3. **Replace size-based gating with time-based LRU** (keep last N builds or last 7 days, not >10GB)
4. **Add active-process checks** before deleting build dirs (use lsof or pid checks)
5. **Audit log all cache deletions** (timestamp, path, bytes, reason, command)
6. **Query cargo metadata** to find actual target dirs instead of hardcoding `target/` pattern
7. **Transactional shard publishing** to avoid partial taxonomy state
8. **Document cache lifecycle** in a new `.🧬semio/⚡️CACHE.md` (retention policy, lane isolation, emergency cleanup procedures)
