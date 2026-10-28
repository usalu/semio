# Closeout inventory, waves 1025f and 1026g (aud-final-g)

Read-only audit for `GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Snapshot taken 2026-10-26 18:13 CET (`git status --porcelain`, about 10,470 entries: 1,622 A, 4 AM, 561 M, 3 MM, 8,280 D, 2 ??). Nothing was edited, staged, deleted or committed. Intermediate files (status, classification, lane-text index, ignore rules) live in the session scratchpad, not in the ticket.

## Method

- Window: wave 1025f started 2026-10-25 19:15 CET. A changed file is counted when its mtime is at or after that time and `git status` lists it (M, A, MM, AM). Deleted paths have no mtime, so deletions are judged by path and by the session-start `gitStatus` snapshot.
- Result-file cross-check: the 1025f and 1026g lane result files (`C/result-*-f.md`, `C/result-*-g.md`, 85 files) have no uniform changed-file list. Each path was therefore matched by its last two segments (`dir/file`) against the concatenated text of those 85 files. 692 of 936 paths are named there. The 244 unnamed paths are flagged `?` below. Most are generic `🟦️.ts` files under directories that the lanes abbreviate. They were not verified one by one.
- Paths are the exact `git -c core.quotepath=off` spelling, which is the form `ticket_close` needs.

## 1. Source files changed or added by waves 1025f and 1026g

Totals in the window: 936 paths (546 M, 388 A, 2 MM, 0 D attributable). Of these, 8 are ticket and goal fixtures, not source (section 3). That leaves 928 source or config paths.

| Area (top level) | Paths | Notes |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo` | 645 (388 M, 257 A) | build library `📚️library` 588 (332 M, 256 A); `🔨️modules` cli, dashboard, tickets, workspace, client, launcher, hooks, mcp and others 57 |
| `🧰️framework/🛍️products/` (os, print, course) | 7 M | `💻️os` 5, `📓️print` 1, `📚️course` 1 |
| `🧰️framework/🔨️modules/🏃️process` | 190 (109 M incl. 2 MM, 81 A) | `📦️artifacts/🏗️native-build` 70; `🏗️build/🌡️host-capacity` 31 (10 M, 19 A, 2 MM); `🔒️leases` 37; `🧪️testing` 27; `🧭️routing` 10 |
| `🧰️framework/🔨️modules/📁️filesystem` | 30 (8 M, 22 A) | `🧾️observation` churn and changes |
| `🧰️framework/🔨️modules/⚙️runtime` | 20 (4 M, 16 A) | `🧲️native/📚️libc` 5 A, `🟢️node/🧲️native` 11 |
| other modules (`🖱️ui`, `🎭️actor`, `🌱️value`) | 3 M | |
| `🧰️framework/📦️packages` | 4 (3 M, 1 A) | typescript runtime and rust |
| `🧰️framework/⚙️configuration/🏃️process` | 1 M | |
| `🎓️teaching` (course, proctor) | 15 (14 M, 1 A) | `📚️course/🧪️tests/🐕️pet-walk` 3, `🎭️e2e` 6 |
| `🌎️hub` | 3 M | provenance and stdio composition |
| `♻️mit-bestand` (projektetage deck, bericht) | 2 M | |
| `✏️s/🔌️plugins/🔱️trinity` | 1 M | `🔌️jack/Cargo.lock` |
| repo root and CI | 7 (`nx.json` M, `CMakeLists.txt` A, `.devcontainer` 2 M, `.github` 1 A and 2 M) | see below |
| fixtures, not source | 8 A | `.🧬semio/…/🎫️tickets/🎆️26/🌙️10/☀️26/{RENAMED-IMPORTANT, CANONICAL-IMPORTANT, CANONICAL-IMPORTANT-COLLISION}`, `…/🌙️08/☀️26/IMPORTANT-LIFECYCLE`, `.🧬semio/🦑️repo/🎯️goals/SKETCHPAD/🎯️goal.json` |

Notes on the root and CI rows:
- `CMakeLists.txt` (A) is not a fixture. The coordination text records that the root file was restored from `HEAD^` at 10:21 because its digest is pinned by `🛠️cmake/🧬️schema`.
- `nx.json` (M) carries the `schedulingSourceExclusions` named input from `exec-f`.
- `.devcontainer/Dockerfile`, `.devcontainer/devcontainer.json`, `.github/actions/devcontainer-suites/action.yml`, `.github/dependabot.yml`, `.github/workflows/build-system.yml` were already staged in the session-start snapshot. Their mtimes fall inside the window, but the lane owning them is not confirmed (likely `xplat-g` or `xplat2-g`).

Deletions in source: none attributable to waves 1025f or 1026g. The 8,027 staged deletions outside this ticket are all older or unexplained and are listed in section 3.

Trackable: every path below appears in `git status`, so it is tracked or staged and not gitignored. Paths marked `?` are not named in any lane result file.

### Full list, grouped by area

Status letters follow `git status`: M modified, A added (staged), MM and AM staged plus worktree change. `?` marks an unnamed path.

### `.devcontainer`

```text
  M  .devcontainer/Dockerfile
  M  .devcontainer/devcontainer.json
```

### `.github/actions/devcontainer-suites`

```text
  A  .github/actions/devcontainer-suites/action.yml
```

### `.github`

```text
  M  .github/dependabot.yml
```

### `.github/workflows`

```text
  M  .github/workflows/build-system.yml
```

### `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08`

```text
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️26/IMPORTANT-LIFECYCLE/🎫️ticket.json
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️26/IMPORTANT-LIFECYCLE/📌️important/📝️.md
```

### `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10`

```text
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CANONICAL-IMPORTANT-COLLISION/📌️important/📝️.md
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CANONICAL-IMPORTANT/🎫️ticket.json
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CANONICAL-IMPORTANT/📌️important/📝️.md
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/RENAMED-IMPORTANT/🎫️ticket.json
? A  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/RENAMED-IMPORTANT/📌️important/📝️.md
```

### `.🧬semio/🦑️repo/🎯️goals/SKETCHPAD`

```text
? A  .🧬semio/🦑️repo/🎯️goals/SKETCHPAD/🎯️goal.json
```

### `CMakeLists.txt`

```text
  A  CMakeLists.txt
```

### `nx.json`

```text
  M  nx.json
```

### `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck`

```text
  M  ♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🪞️zukunft-bau-mirror/🟦️.ts
```

### `♻️mit-bestand/📋️bericht/🔨️modules/✅️report-verification`

```text
  M  ♻️mit-bestand/📋️bericht/🔨️modules/✅️report-verification/🟦️.ts
```

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack`

```text
? M  ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.lock
```

### `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/📥️reading`

```text
  M  🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/📥️reading/🟦️.ts
```

### `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧪️tests`

```text
  M  🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧪️tests/🟦️.ts
```

### `🌎️hub/🧩️compositions/🗄️stdio/📥️execution`

```text
  M  🌎️hub/🧩️compositions/🗄️stdio/📥️execution/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config`

```text
  M  🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions`

```text
  A  🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🔣️.json
  M  🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🟦️.ts
  M  🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🧫️fixtures/🔣️.json
  M  🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🔒️sessions/🧬️schema/🔣️.json
```

### `🎓️teaching/🏛️architecture/📚️course/🎭️e2e`

```text
  M  🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript`

```text
  M  🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json
```

### `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎭️sessions`

```text
  M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎭️sessions/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🏗️proctor-build`

```text
? M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🏗️proctor-build/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk`

```text
  M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/🟦️.ts
  M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/🧰️pet-kit/🟦️.ts
  M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/🪜️mischief-ladder-and-phone/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🗣️both-languages`

```text
  M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🗣️both-languages/🟦️.ts
```

### `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy`

```text
  M  🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🏗️site-build/🟦️.ts
```

### `🎓️teaching/🛂️proctor/🏗️bootstrap`

```text
  M  🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts
```

### `🧰️framework/⚙️configuration/🏃️process`

```text
  M  🧰️framework/⚙️configuration/🏃️process/🔣️.json
```

### `🧰️framework/📦️packages/🟦️typescript`

```text
  M  🧰️framework/📦️packages/🟦️typescript/package.json
```

### `🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🏁️main`

```text
? M  🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts
```

### `🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🧲️native`

```text
? A  🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🧲️native/📚️libc/🟦️.ts
```

### `🧰️framework/📦️packages/🦀️rust`

```text
  M  🧰️framework/📦️packages/🦀️rust/📜️script.ts
```

### `🧰️framework/🔨️modules/⚙️runtime/🏁️main`

```text
? M  🧰️framework/🔨️modules/⚙️runtime/🏁️main/🟦️.ts
```

### `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native`

```text
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🦀️.rs
? A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🏃️corpus/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🐍️.py
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🔣️.json
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧮️system/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧮️system/🧪️tests/🟦️.ts
```

### `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🛑️termination`

```text
  A  🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🛑️termination/🟦️.ts
```

### `🧰️framework/🔨️modules/⚙️runtime/🧪️tests`

```text
  M  🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts
```

### `🧰️framework/🔨️modules/⚙️runtime/🧬️schema`

```text
  M  🧰️framework/🔨️modules/⚙️runtime/🧬️schema/🗣️messages.json
```

### `🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc`

```text
? A  🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc/🧪️tests/🐍️.py
  A  🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc/🧪️tests/🔣️.json
  A  🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/⚙️runtime/🧲️native/📚️libc/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🌱️value/♻️retirement/🌀️support`

```text
  M  🧰️framework/🔨️modules/🌱️value/♻️retirement/🌀️support/🧪️tests/🟦️.ts
```

### `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧬️typegen`

```text
? M  🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧬️typegen/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity`

```text
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🌦️forecast/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📋️contract/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts
  MM 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🔼️upgrade/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json
  MM 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🧫️fixtures/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔮️sqlite/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🚪️gate/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🥒️.feature
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🧫️fixtures/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🚪️gate/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🧫️fixtures/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🧫️fixtures/🐍️tree.py
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪟️windows/🧫️fixtures/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪜️phases/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript`

```text
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🟦️.ts
? A  🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧪️tests/🔁️incremental/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process`

```text
  M  🧰️framework/🔨️modules/🏃️process/📋️project.json
```

### `🧰️framework/🔨️modules/🏃️process/📖️texts`

```text
  M  🧰️framework/🔨️modules/🏃️process/📖️texts/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build`

```text
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🔓️tokens/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧵️frontend-threads/🐍️.py
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧵️frontend-threads/🔣️.json
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧵️frontend-threads/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧵️frontend-threads/🧬️schema/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/⏳️wait/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧪️tests/📥️inspection/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/📋️plan/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/📡️observe/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🟦️.ts
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🐍️.py
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🩺️heal/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧬️schema/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🧪️tests/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🧬️schema/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤫️silence/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤫️silence/🧪️tests/🟦️.ts
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🧬️schema/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🔀️exchange/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🔀️exchange/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🪞️fold/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧬️schema/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🪟️host/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🪟️journal/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🪟️journal/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🍎️mounted-image/✍️sealing/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🍎️mounted-image/✍️sealing/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🍎️mounted-image/📐️geometry/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📁️directory-partition/📐️geometry/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🔌️operations/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🧪️tests/⚖️laws/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🧪️tests/⚖️laws/🧫️fixtures/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🧪️tests/✅️conformance/🟦️.ts
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧰️programs/🧬️schema/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧬️schema/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🧪️tests/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔓️unit-lock-model/🐍️.py
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔓️unit-lock-model/🔣️.json
? A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔓️unit-lock-model/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔓️unit-lock-model/🧬️schema/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🧱️unit-exclusion/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript`

```text
  M  🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/package.json
? M  🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/⏳️wait`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/⏳️wait/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/⏳️wait/🧪️tests/🧫️fixtures/🔣️.json
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/⏳️wait/🧬️schema/📜️policy.json
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/⏳️wait/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness/🧪️tests/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/📂️descriptors/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/🧫️fixtures/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/🪟️windows/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧪️tests/🪟️windows/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing`

```text
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⏱️eta/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⚖️priority/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🐍️model.py
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/📥️worker/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🚦️live/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🧫️fixtures/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/📋️status`

```text
? M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/📋️status/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🧫️fixtures/📜️queue.json
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order`

```text
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧩️component-driver/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🪟️registry-root/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows`

```text
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget`

```text
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🧪️tests/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox`

```text
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🧫️fixtures/📒️ledger.json
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers`

```text
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/📐️learning/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/📐️learning/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🧫️fixtures/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧪️tests/🧱️capped-run/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites`

```text
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🏃️run/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo`

```text
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/📥️inputs/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest`

```text
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/📋️contract/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain`

```text
? M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🟦️.ts
  A  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🧪️tests/🐍️oracle/🐍️.py
  M  🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/📌️pinned/🧪️tests/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command`

```text
? M  🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/⚙️configuration/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/⚙️configuration/🧬️schema/🔣️.json
  A  🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🏠️runtime-store/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation`

```text
  M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️graph-phase/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️graph-phase/🟦️.ts
? M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json
? M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts
  M  🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json
```

### `🧰️framework/🔨️modules/🏃️process/🪓️termination`

```text
  M  🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts
```

### `🧰️framework/🔨️modules/🏃️process/🪓️termination/🧪️tests`

```text
  A  🧰️framework/🔨️modules/🏃️process/🪓️termination/🧪️tests/🪟️taskkill/🐍️taskkill.py
  A  🧰️framework/🔨️modules/🏃️process/🪓️termination/🧪️tests/🪟️taskkill/🟦️.ts
```

### `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor`

```text
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json
```

### `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes`

```text
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🌳️tree/🟦️.ts
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧪️tests/🐍️.py
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧪️tests/🔣️.json
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧪️tests/🟦️.ts
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧬️schema/🔣️.json
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts
```

### `🧰️framework/🔨️modules/📁️filesystem/🧾️observation`

```text
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️script.ts
```

### `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn`

```text
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🌊️races/🟦️.ts
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/📸️snapshot/🟦️.ts
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🌊️races/🐍️.py
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🌊️races/🔁️layers/🟦️.ts
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🌊️races/🔣️.json
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🌊️races/🟦️.ts
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🐍️.py
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🟦️.ts
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🎲️fuzz/🟨️.mjs
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/📸️snapshot/🟦️.ts
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🔒️busy/🟦️.ts
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🧾️refusals/🟦️.ts
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🌊️races.json
  A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/📸️snapshot.json
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json
? M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🧾️refusals.json
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧾️refusals/🟦️.ts
```

### `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests`

```text
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🌐️cross-runtime/📜️script.ts
? A  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🌐️cross-runtime/🟦️.ts
  M  🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts
```

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages`

```text
  M  🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts
```

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`

```text
  M  🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🟦️.ts
```

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`

```text
? M  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts
```

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`

```text
? M  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts
  M  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts
```

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl`

```text
? M  🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json
```

### `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification`

```text
  M  🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json
```

### `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react`

```text
  M  🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json
```

### `🧰️framework/🛍️products/🦑️repo`

```text
  M  🧰️framework/🛍️products/🦑️repo/README.md
  A  🧰️framework/🛍️products/🦑️repo/📓️build-guide.md
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🏭️mcp-factory/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/📁️root-scope/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🔗️mcp/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🥅️goal/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/🧪️tests/🔦️search/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🛤️task-routes/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌀️control-plane/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🎛️dispatch/🟦️.ts
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧪️tests/📁️scope/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧪️tests/🔬️unit/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🎨️formatter/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/📡️files/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧪️tests/🔎️glob/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧪️tests/🔬️unit/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧪️tests/🧭️root-selection/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🔷️configure-micro-commit/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🚫️gitignore-and-formatters/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🎼️executor-harness/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🧬️root-dir-and-compose-policy/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️test-runners/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗂️catalog/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🏗️build-surface/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`

```text
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🔸️framework-reserved-exact/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn/📒️allowlist/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/♻️pressure/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧹️supersede/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧬️schema/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🔣️policy.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🚀️runtime-bootstrap/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧪️tests/🐍️.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧪️tests/🧫️fixtures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🧪️tests/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🧪️tests/🧫️fixtures/🧾️generation.original.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔒️lock-sync/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.d.mts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧪️tests/🦀️native-dependency-closures/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧊️memo/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️ancestry/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🔌️host/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🔣️policy.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🧪️tests/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🧪️tests/🔐️holder/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🎯️selection/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🏁️entry.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🐍️echo.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🟦️echo.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🔐️pool-claims/📥️worker/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🔐️pool-claims/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/📤️retention-model/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/📤️retention-model/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/📤️retention-model/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/📤️retention-model/🧬️schema/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔀️retention-interleaving/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔀️retention-interleaving/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔀️retention-interleaving/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔀️retention-interleaving/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔐️retention-io/📥️worker/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔐️retention-io/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🛣️consumer-lanes/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🧾️provenance-equality/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️import-edges/📁️paths/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/📏️recensus/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/📦️compact/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🏷️labels/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📅️dates/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📈️attribution/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🎨️format/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🔀️deltas/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🔢️uloc/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📊️metrics/🧮️summary/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📋️prepare-commands/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📖️texts/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📜️body/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/📤️emit/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🔖️tags/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🔢️counter/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🔸️bullets/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🧭️path-prefixes/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪜️steps/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪝️hooks/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪝️hooks/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📝️commit/🪝️hooks/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🧪️tests/🪟️nextest/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🌿️runtime-selection/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🏷️naming/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧰️tree-helpers/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧰️testkit/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🪞️unmirror/🔍️resolver/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗂️collection-catalog/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/📋️ledger/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️entry/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧪️tests/🌱️plan-child/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧭️selection/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/🧭️scoping/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/⌨️command/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🚏️marks/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🏊️farm/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/📈️memory-fit/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/📈️memory-fit/🧫️fixtures/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧪️workers/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🪟️partial/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️workers/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔒️hermetic-classes.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/README.md
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📥️execution/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔁️promoter/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🚦️trampoline/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🚪️entry/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/👥️processes/📜️child.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/👥️processes/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🔀️interleavings/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🔀️interleavings/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🔀️interleavings/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🔀️interleavings/🧬️schema/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🔬️syntax/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🔬️syntax/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🗂️store/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧭️mode/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧭️mode/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/⌨️main/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🔭️world/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🔭️world/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/🎲️fuzz/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/🎲️fuzz/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/📊️bench/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔂️validation-round/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔂️validation-round/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔂️validation-round/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/⌨️command/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🎯️slo/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🏃️runner/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🐍️oracle/🐍️.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🧫️fixtures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🤝️concurrent/🎬️workloads/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🤝️concurrent/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📐️conformance/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🟨️.d.mts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🛤️router/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🛤️router/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🛤️router/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/🔄️lifetime/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🤝️shared-writer/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🧓️stale-reader/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🪞️mirror/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🧬️schema/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🪞️mirror/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🪢️policy/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🔒️batch-snapshot/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🟨️.d.mts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪛️toolchain-activation/🧪️tests/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🏗️compiler/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🐍️oracle/🐍️.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🧪️tests/🏷️loader-identity/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧫️fixtures/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🐍️oracle/🐍️.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🍞️bun/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🔒️real-bazel/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️taskkill/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️toolchain-link/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👷️workers/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🧫️fixtures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🔐️mode/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📋️contract/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️coverage/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️environment/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️mode/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🐍️.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🧫️fixtures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🐍️model.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🧬️schema/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.d.mts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🧪️tests/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📋️model/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🐍️.py
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚦️governed-dispatch/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🧪️tests/🔁️observation/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🌊️streaming-edges/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🌊️streaming-edges/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🪣️re-edge-chunks/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🪣️re-edge-chunks/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🙈️presentation-resources/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🚧️withheld-rows/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🚧️withheld-rows/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🚧️withheld-rows/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🏪️store/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🏪️store/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🏪️store/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌍️environment/🟨️.d.mts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌍️environment/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.d.mts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🕊️early-release/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🧪️tests/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.mjs
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/📥️holder/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/📥️worker/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🏃️processes/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧬️schema/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌱️patch-growth/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🚧️unfounded-withheld/🐍️oracle/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🚧️unfounded-withheld/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🚧️unfounded-withheld/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🚧️unfounded-withheld/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🪪️row-keys/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🪪️row-keys/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/🗂️relative-owners/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/🗂️relative-owners/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/🗂️relative-owners/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/♻️tagged-retention/🐍️oracle/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/♻️tagged-retention/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/♻️tagged-retention/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/📤️portable/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔗️edge-refresh/🐍️oracle/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔗️edge-refresh/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔗️edge-refresh/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🕊️graph-release/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/📜️script.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🧫️fixtures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/README.md
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🏃️main/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🔄️restart/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🚦️startup/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🚦️startup/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎟️phase/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎯️source-roster/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎯️source-roster/🧫️fixtures/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎯️source-roster/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🌱️native-scope/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️engine.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/📝️plan-file/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🧫️fixtures/🪆️relaunch.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/👁️watch/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🧪️tests/🚧️withheld-task/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🟨️.d.mts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧪️tests/🔁️implementation-edit/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/💰️budget/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🚀️launch/🧶️nx/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🧶️units/🐍️.py
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🧶️units/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/📖️texts/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/📜️script.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🧫️fixtures/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🧫️fixtures/🔣️.json
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🧪️tests/🔎️probes/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/💊️remedies/📖️texts/🔣️.json
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/💊️remedies/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/💊️remedies/🧪️tests/🐍️oracle/🐍️.py
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/💊️remedies/🧪️tests/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/💊️remedies/🧬️schema/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📇️catalog/📣️publication/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📋️policy/⏯️tool-run/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📁️data-directories/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📄️manifest-projection/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🏃️command/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧬️metadata/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🔒️allowlist/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🖥️cli/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🧭️entrypoints/🧪️tests/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs
? A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🚀️launchers/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🎚️levels/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🎛️repository-command/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧠️memo/🟦️.ts
  A  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧠️memo/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🗄️lcov/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/⚖️comparison/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔣️json-schema/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🧱️contract/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔮️oracles/📦️package-boundary-classification/📦️packages/🦀️rust/Cargo.toml
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🔣️taxonomy-input/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️candidate-discovery/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️empty-folders/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️marker-only-folders/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🗑️removal/🧪️tests/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🧪️tests/🟦️.ts
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp`

```text
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🖼️assets/🏗️build-surface.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧪️tests/🏗️build-surface/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧬️schema/🏗️build-surface.json
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🧪️tests/📁️ticket-scope/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🧪️tests/🩹️autofix/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/next-env.d.ts
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/README.md
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🔵️.ps1
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🗂️runtime-registry/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🗣️language/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🛎️command/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🛰️resident-unix/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🛰️resident/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🧬️schema/🔣️.json
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🗂️registry-contract/🐹️.go
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🛰️resident-contract/🐹️.go
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧫️fixtures/🔣️.json
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers`

```text
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/🧪️tests/🔬️unit/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.d.mts
  M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks`

```text
? M  🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/🧪️tests/🎫️track/🐹️.go
```

### `🧰️framework/🛍️products/🦑️repo/🪝️hooks`

```text
? M  🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-checkout
? M  🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-commit
? M  🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-merge
? M  🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-rewrite
? M  🧰️framework/🛍️products/🦑️repo/🪝️hooks/prepare-commit-msg
```


## 2. Ticket folder: tool outputs to delete at close, inputs and reports to keep

The ticket holds 11,579 files under 566 top-level directories: about 268 MB on disk, about 239 MB of file bytes. Sizes are file bytes.

| Category | Files | Size | Decision at close |
|---|---|---|---|
| `🗑️generated/` (14 lane folders, incl. `nx/e2e` Playwright traces, 3 `trace.zip` = 116 MB) | 75 | 116.7 MB | delete (AGENTS rule) |
| `*.txt` outside `🗑️generated` (lists, grep dumps, census tables) | 514 | 16.3 MB | delete |
| `out/` directories and `*.out` | 112 | 0.1 MB | delete |
| backup trees (`backup/`, `Backup*/`), `*.bak`, `*.orig`, `*.absent` | 104 in backup trees, 18 `*.bak`, 27 `*.orig` | 4.5 MB, 0.4 MB, 0.5 MB | delete |
| `*.jsonl`, `*.log`, `*.pid`, `*.list` | 25 | 0.5 MB | delete |
| `*.tsv` (plan and manifest tables) | 76 | 3.3 MB | delete unless a report cites one |
| `*.cons` (`fnsplit4a-finish-tools/specs`) | 5 | 4 KB | review: spec input of the split tool |
| top-level outputs `cycle-break-delta.json` (851 KB), `cycle-break-supersedes.json` (49 KB), `close-files-candidate.json` (92 KB) | 3 | 0.9 MB | delete |
| JSON in `*-tools/` (heuristic: numbered `N.json`, `p-*.json`, plan, manifest, touched, applied, bases) | about 946 | 30.0 MB | delete |
| JSON in `*-tools/` (heuristic: `🔣️.json` fixtures, schema, corpus, tsconfig, policy, cases, goldens) | about 504 | 10.3 MB | keep |
| other JSON outside tools: `🎫️ticket.json`, `gocanon/overrides.json`, `stale-publication-intents/*.json` | 4 | under 1 KB | keep |
| other JSON outside tools, likely outputs: `gocomp/decls2.json` (474 KB), `gocomptest/decls.json` (82 KB), `gocomptest/assignment.json` (22 KB), `gocomp/dirnames.json` (4 KB) | 4 | 0.6 MB | review, probably delete |
| reports (about 870 `.md`, incl. `result-*.md`, `coordination.md` 367 KB, audits, briefs, handovers), scripts and code (`py`, `ts`, `mjs`, `cjs`, `rs`, `go`, `sh`, `toml`, `yml`, `tsx`), `staged/` batch patches, `Cargo.lock`, `go.mod` | 9,174 (keep share, incl. the ignored ones below) | 55.1 MB | keep |

Gitignored files in the ticket: 680 (`git ls-files --others --ignored --exclude-standard`). They are invisible to git and to `ticket_close`.

| Ignore rule (`.gitignore` line) | Files | What it hits |
|---|---|---|
| `🗑️generated` (line 20) | 256 | generated outputs (delete) |
| `Backup*/` (509) | 161 | backup trees (delete) |
| `*.out` (138) | 112 | tool outputs (delete) |
| `build/` (303) | 63 | 45 `.py`, 12 `.sh`, 6 `.ts` in `legacy-purge-tools/build/`, `tool-iface-tools/build/`, `tool-direct-code-tools/build/` and similar: scripts, keep |
| `target*` (18) | 32 | 16 `.py`, 5 `.ts`, 5 `.json`, 3 `.mjs`, 3 `.txt`, e.g. `bzl-hermetic-tools/targets.py`, `ci-bazel-tools/staged/targets.json`: scripts, keep |
| `*.spec` (320) | 18 | `gotix-tool/*.spec`, `i18n-core-tools/specs/*.spec`: specs, keep |
| `out` (81) | 12 | `proof2-g-tools/out/...` (delete) |
| `*.log` (428), `*.bak` (283) | 9, 5 | outputs and backups (delete) |
| `lib/` (309) | 6 | 6 `.py`: scripts, keep |
| `test-results` (86) | 2 | `ci-bazel-tools/src/test-results/{module,test}.ts`: source, keep |
| `generated` (13), `old` (10) | 2, 2 | `r6-tool-iface-tools/src/generated/...`, `alloc-f-tools/old/...` |

About 125 code, spec or config files are therefore not committed. The ignore rules `build/`, `target*`, `*.spec`, `lib/`, `test-results` and `old` catch them. Deciding whether to un-ignore them belongs to the user.

Trackable close list: pass `path` explicitly as `26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Start the `files` array with the ASCII path `brief-wave-1026g.md`, because an all-emoji array fails with "at least one file is required". Leave out gitignored files and anything under `🗑️generated`. The keep set is about 9,050 files, so pass the reports and the lane tool folders rather than a 9,000-entry array, unless the tool accepts it.

Checklist items:
- `📌️important.md`: none in this ticket (only `status-*.md` and `🎫️ticket.json` at top level). Nothing to clear.
- Live churn: `census2-g-tools/lib-rest.jsonl.38511470.log` vanished during the walk. Logs are still being written here, so re-run the inventory right before close.

## 3. Fixture pollution and user decisions

### 3a. Fixture pollution outside ticket folders (not deleted by me)

Cause, from `result-proof2-g.md` (FOR coordinator line): the official Go `:test` routes ran with TMPDIR inside the checkout, root discovery is `.git`-only, so tests that bind `t.TempDir()` as repository root wrote into the real tree. The Go route fix is staged there. Do not run the Go `:test` targets of tickets, cli, graphql or workspace until that fix lands.

| Path | Content | Mtime | Git state |
|---|---|---|---|
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/RENAMED-IMPORTANT/` | `🎫️ticket.json` (558 B), `📌️important/📝️.md` (8 B) | 17:16 to 17:39 | A (staged) |
| `…/🎆️26/🌙️10/☀️26/CANONICAL-IMPORTANT/` | `🎫️ticket.json` (407 B), `📌️important/📝️.md` | 17:37 | A |
| `…/🎆️26/🌙️10/☀️26/CANONICAL-IMPORTANT-COLLISION/` | `📌️important/📝️.md` | 17:44 | A |
| `…/🎆️26/🌙️08/☀️26/IMPORTANT-LIFECYCLE/` | `🎫️ticket.json` (189 B), `📌️important/📝️.md` | 17:44 | A |
| `.🧬semio/🦑️repo/🎯️goals/SKETCHPAD/🎯️goal.json` | goal "Sketchpad", due 2026-02-15, client claude-code | the go-g test seeds it into the real goals dir (17:50 red: "goal with id SKETCHPAD already exists") | A |

AGENTS forbids opening or closing goals without the developer's instruction, so the SKETCHPAD goal is a removal candidate for the user.

`CMakeLists.txt` at the repo root is not pollution. It is the restored root file (see section 1).

No gitignored pollution was found under `🎆️26/🌙️10/☀️26` or `.🧬semio/🦑️repo/🎯️goals` (`git status --ignored`).

### 3b. User decisions

1. 8,024 staged deletions under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/`. Breakdown: 355 files directly in the folder, `oinf-tools` 125, `pi2-tools` 111, `rhu2-tools` 98, `ohp-tools/families` 71, others. They are staged `D` already in the session-start snapshot (for example `2dc-tools/*`), so this window did not stage them. `result-proof2-g.md` records that the 8,024 tracked files vanished from the working tree at 17:16:16 and suspects the Go purge and finish tests, but says this is unproven. Decision: restore from HEAD, or accept the deletion. I ran no git command.
2. 253 staged deletions inside this ticket, all in older tool folders: `stage-clean-tools` 128, `gran-hubs-tools` 78, `aud-census8-tools` 26, `cc-tools` 21. Decision: accept or restore.
3. 3 staged deletions under the ticket `🎆️26/🌙️10/☀️04/SOLVE-ALL-MERGE-CONFLICTS/`: `domain-exact-ledger-repeat-695.md`, `framework-overlap-repeat-691.md`, `plugin-conflicts-repeat-695.md`. Decision: accept or restore.
4. The index as a whole holds 1,622 staged A, 561 M, 8,280 D and 3 MM entries. Committing or unstaging is the user's call.
5. Removal of the 8 fixture paths in 3a, including the SKETCHPAD goal file: the user decides, and removal of staged files needs a git command the user runs.

## 4. Draft ticket_close summary (25 lines or fewer)

```text
GET-CODEBASE-RUNNING-END-TO-END-AGAIN (path 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN).
Waves 1025f and 1026g, 2026-10-25 19:15 to 2026-10-26 18:20.
Goal: keep dashboard, architecture course and print running end to end on Bazel; make the build
system ready for many agents on one 16 GB workspace (incremental, warm cache, low RAM).
Source: 928 source/config paths changed or added (548 M, 380 A). No source deletion is attributable to
these waves. Largest areas: build library 588, process and native-build 190, products/repo outside
the library 57, course, print and os 8. Full list: closeout-inventory-1026g.md, section 1.
Lanes: 85 result files (20 -f, 65 -g). Batches landed one at a time through gate-g with one product
build each (result-gate-g.md). Latest integ-g check: course build GREEN, course test 1 red of 703 (result-integ-g.md).
Incident: the official Go :test routes ran with TMPDIR inside the checkout and wrote fixture tickets
RENAMED-IMPORTANT, CANONICAL-IMPORTANT, CANONICAL-IMPORTANT-COLLISION, IMPORTANT-LIFECYCLE and goal
SKETCHPAD into the real tree (result-proof2-g.md). Removal and the Go route fix await user approval.
Open user decisions: 8,024 staged deletions under 26/08/11 monolith-split (vanished 17:16:16, cause
unproven); 253 older staged tool deletions in this ticket; 3 under SOLVE-ALL-MERGE-CONFLICTS.
About 125 code, spec and config files are gitignored (build/, target*, *.spec, lib/) and are not committed.
Delete at close: 🗑️generated (75 files, 117 MB), 514 txt, 112 out, backup trees, 25 jsonl/log/pid/list,
76 tsv, about 946 output JSON (30 MB), 3 top-level cycle and close candidate JSON.
Keep: about 870 md reports (result-*.md, coordination.md, audits, briefs, handovers), scripts, staged
batch patches, about 504 fixture and schema JSON, 🎫️ticket.json.
📌️important.md: absent. Close files: start with brief-wave-1026g.md (ASCII), then tracked keep files.
```

Count of lines in that block: 22.

## 5. Wave-level verification notes

- The result files have no uniform changed-file list. Changed-file lists were reconstructed from `git status` plus mtimes, then cross-checked by name against the 85 lane files. 692 of 936 paths are named. The 244 unnamed paths are listed with `?` in section 1 and were not verified one by one.
- Byte counts come from `stat`. A file that vanished during the walk is noted above.
