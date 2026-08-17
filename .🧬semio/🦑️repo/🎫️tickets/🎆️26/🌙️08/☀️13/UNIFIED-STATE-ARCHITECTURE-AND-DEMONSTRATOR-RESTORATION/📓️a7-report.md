# A7 — Policy enforcement of the four state mechanisms

The mandate was "only these 4 different mechanisms are used and **enforced by api and policies**".
A1 delivered the API half. This is the policy half: an available lane that nothing forces you to use
just becomes a fifth option beside the ad-hoc ones.

## `policyStateLaneExhaustivenessBreaches` (new, root `📜️script.ts`)

Report-mode, medium priority, two sub-kinds. Registered in the aggregate policy run.

| Sub-kind | Count | What it catches |
|---|---|---|
| `taxonomy/state-lane-ephemeral-box` | 110 | `ephemeralBox`/`ephemeralMap`/`ephemeralSet`/`ephemeralWeakMap` — untyped process-local state keyed by string. Their own docstring calls them the sole lane "until the OS draft snapshot owns these keys"; the **transient** lane now owns exactly that role, typed and dispatched. |
| `taxonomy/state-lane-storage-outside-config-lane` | 7 | Direct `localStorage`/`sessionStorage`/`indexedDB` outside the sanctioned adapter. Persisted local-only state IS the config lane. |

**117 total.** The full policy run stays at **23,866 high-priority across 30 rules** — unchanged,
because this rule is deliberately medium (it measures a large real surface that wave A4 retires, and
gating on it today would just wedge the build).

### Sanctioned storage owners (exempt, with reasons)

- `🧰️framework/🔨️modules/🖥️platform/` — this IS the config lane's `StoragePort` persistence adapter.
  The storage MEDIUM was never the problem; bypassing the lane is.
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` — backs `BlobStore`, i.e. content-addressed
  **artifact** content (what a `LinkPin::Snapshot` escrows). Flagging it would be a category error:
  it is not local-only UI state routing around a lane.
- `.test.ts`/`.test.tsx` and `🧪️` paths — a test that drives the adapter, or asserts an ephemeral box
  was retired, must be able to NAME the thing it tests.
- `compose/`, `♻️mit-bestand/`, `🌎️hub/`, repo tooling — the parallel stack and non-app product code,
  matching the exclusions the existing OS-state-authority rule already uses.

### Precision work (the rule was wrong twice before it was right)

First run reported 128. Investigating each class rather than accepting the number found two false
positive classes, both now fixed:

1. **Doc comments.** A prose line reading `* (not a \`localStorage\` default) …` tripped the rule.
   Stripping `//` is not enough — a block-comment *continuation* line starts with `*`, and that is
   exactly where a docstring mentioning storage lives. This repo's vocabulary policies have been
   bitten by the same hazard before (a comment explaining that a banned identifier was removed
   trips the rule that bans it).
2. **Test files** asserting on storage behaviour.

Every one of the 117 survivors was then spot-checked against its source line and is genuine code.

## Also updated: the stale `plugin-purity` guidance

`policyPluginPurityBreaches`'s `thread-local-state` solution text still said *"Replace with typed
Draft-lane state once the Draft mechanism lands (W1)"* — advice that was both stale (the lanes exist
now) and **wrong** (draft is for uncommitted document content, not UI state). It now routes by what
the state actually IS:

- ephemeral local UI state → **Transient** lane
- ephemeral shared state → **Presence** lane
- uncommitted document content → **Draft** lane
- a cached view of an owned CHILD's content → **do not cache**; read `ArtifactView.children`, which
  cannot go stale — with a pointer to `📓️cw2-child-cache-finding.md` for the one case still blocked
  on a design decision.

## What this does and does not achieve

It **measures and names** every route around the four mechanisms, with an actionable per-site
solution. It does **not** yet gate: turning either sub-kind to high priority is the closing move
after wave A4 migrates the shell's storage into `OsShellConfig` and the ephemeral boxes into the
transient lane. The count is the burn-down target: **117 → 0**.
