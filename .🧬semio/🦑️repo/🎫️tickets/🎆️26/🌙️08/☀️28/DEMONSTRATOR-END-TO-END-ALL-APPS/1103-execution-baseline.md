# Execution Baseline — 2026-11-03

## Current Result

The declared `repo run @semio-tech/mit-bestand-demonstrator:test --output=stream` initially exited 1 before graph admission with `Canonical build owner identity collision`. Four duplicate physical owners in `📋️build-owners.json` had stale names. The four retained rows match current physical manifests; four stale duplicate rows were removed. No package manifests or dependencies were edited.

## Regression Evidence

The existing language-neutral authored-registry identity check in graph admission uses SQLite UNIQUE constraints as an independent oracle. Before the edit: 0 pass, 1 fail (`SQLITE_CONSTRAINT_UNIQUE`, owner.manifest). After the edit: 1 pass, 0 fail, 5 filtered out. Run via `repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/🟦️.ts -t "authored workspace registry"`.

The full admission file also ran: 5 pass and 2 failures from Original parent Script process invocation required, because the separate invocation-dependent case requires its declared process birth. This did not affect the focused uniqueness regression.

## Corrected Rows

Removed stale @semio-tech/replication-mutation-retained-field-rs, @semio-tech/replication-mutation-retained-rs, semio-s-artifact-stdio-ifc-2x3-base-catalog, semio-s-artifact-stdio-ifc-4-catalog. Current manifests declare @semio-tech/replication-retained-field-rs, @semio-tech/replication-retained-rs, semio-s-artifact-stdio-ifc2x3-catalog, semio-s-artifact-stdio-ifc4-catalog.

## Retried Declared Operation

The retry passed owner admission, read the build implementation, verified the sealed dependency generation, and reached build graph planning. Session 49595 remains active while verifying 414159 graph facts. No dependency lock/sync, foreign process stops, cache resets, or Git mutations were performed.

## Capacity Investigation at 906 Seconds

The graph completed two input closure passes and entered inference for 1650 projects. At 890 seconds it had completed 35 projects, with TypeScript input precision deriving `✏️s/tsconfig.json`. Native doctor reported graph process PID 38481 using 91.7% CPU and 4336 MB RSS. The nested optional farm request wanted 3584 MB behind queued catalog and cargo requests, with 280 MB spare above the 4096 MB floor. The outer baseline holds 1609 MB of reserved capacity, while observed RSS is larger.

Source inspection of `🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs` shows that the host farm request is optional: `capacity.granted.then` starts workers when granted, without blocking serial project inference. Its close method releases a pending optional capacity request before awaiting opening when only owned pools exist. Therefore this observation is active expensive inference under tight memory, without current evidence of a capacity deadlock. No guards, budgets or capacity limits were changed.

## Resume Revalidation

After the explicit pause and resume, polling original unified execution session 49595 returned `Unknown process id 49595`. Native doctor confirms original baseline processes 38437, 38446 and 38481 are absent and no baseline grant remains. Only the separately owned runtime catalog process remains active. The raw baseline capture ends with inference 307/1650 and has no terminal line. Consequently the original baseline outcome and exit code are unverified; this report does not classify it as a test pass or a test failure. A fresh declared baseline must be captured directly to a ticket output file, including a durable exit status, after coordinating the active graph owner. No restart or source edit was performed during revalidation.
