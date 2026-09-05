# BCF Handpicked Emoji Repair

The BCF 2.1 subsets are `🖊️markup`, `👁️viewpoint`, and `📸️snapshot`; each local oracle is `🔮️oracle`. Options use `☑️`. Authored operation names now have valid single-emoji presentation; archive-wide snapshot uses `🗃️`, distinct from viewpoint image `📸️`.

Fixtures use meaningful individual identities: comment insertion `🗨️`, topic insertion `📌️`, duplicate-topic rejection `👯️`, unchanged baseline `⏸️`, comment removal `🧹️`, topic removal `🗑️`, missing-topic rejection `🕳️`, comment editing `✏️`, markup editing `🖊️`, version `🔢️`; viewpoint insertion `👁️`, viewpoint removal `🙈️`, missing-viewpoint rejection `🕳️`, camera `📷️`, components `🧱️`, screenshot `📸️`. The wellness-center archive uses `🏥️`. All before/after carriers use `⬅️`/`➡️`.

All owned cross-subset references, fixture manifests, directory fields, and exact shared Stdio mounts were repaired. The existing fixture generator explicitly carries the authored subset and directory; its default output now resolves from its own source to the correct subset rather than an arbitrary working directory. It emits the chosen before/after names.

Verification on 2026-09-05:

- Pre-parent-move audit: 223 files, 141 directories, 361 governed entries; all categories zero except the two subset collisions resolved by the final moves.
- Fixture verifier: 16 fixtures, zero problems, exit 0.
- All 33 JSON files parse; 16 manifests and 29 files match recorded SHA-256 and byte length.
- Nx generator completed for all 15 supported markup/viewpoint recipes under the ticket generated folder, writing the authored names in the correct subset trees.
- Final source scan found no stale subset names, local oracle coordinates, bare before/after paths, or old snapshot mutation names. The plugin-level oracle host path is intentional.
- Central oracle overrides and renamed-root full-audit follow-up were supplied to the root agent.

The subsequent semantic review replaced arbitrary white/black-circle/apple mutation-suite identities with `🔀️`, and the clash-resolution scenario with `🤝️`. Its mutation catalog scenario coordinate was repaired as well. All 27 regenerated markup/viewpoint carriers match the recorded SHA-256 and byte length exactly. The final renamed-tree audit counts the same 223 files/141 directories/361 governed entries and has zero findings in every category.
