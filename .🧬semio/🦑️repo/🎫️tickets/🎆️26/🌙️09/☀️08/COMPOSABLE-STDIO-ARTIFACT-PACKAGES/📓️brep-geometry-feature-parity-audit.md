# BREP Geometry Feature and Backend Parity Audit

## Conclusion

The 15 Boolean, Offset, body-reconstruction, and engine failures are not explained by the stdio artifact's default feature set, a missing geometry backend, or skipped native registration. The representative run directly compiled the standalone Semio artifact and entered the in-crate algorithms. The current working-tree extraction changes do not touch the BREP algorithm sources or either relevant Cargo manifest.

This establishes feature/dependency parity for the question investigated. It does not establish that the geometry algorithms are correct; the raw diagnostics remain real core geometry failures.

## Direct feature and dependency evidence

* The raw receipt begins with `[DEBUG] stdio representative semio-default`, then compiles `semio-s-artifact-stdio-semio` and runs its `../../🦀️.rs` unit binary (raw lines 1–12). Its manifest declares `default = []` and makes the core geometry dependencies direct, non-optional dependencies: `semio-framework-3d`, `-geometry`, `-math`, and `-mesh-engine` at [`Semio Cargo.toml`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml) lines 17–43.
* The only BREP feature is `conversion-brep`, which adds the DWG and STEP *artifact codecs*, not a kernel. Its conditional branches are confined to BREP I/O: imports at [`BREP IO`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs) lines 7–14 and composer/DWG registration at lines 145–180. With that feature absent, `register()` still registers the BREP schema descriptor, document codec, validator, and inferences at lines 140–147. No Boolean, Offset, body, or native-engine module has a feature gate.
* The BREP engine directly imports `boolean_solid`, `offset_solid`, `thicken_face`, primitives, validation, and tessellation from this artifact's own BREP tree at [`engine`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🦀️.rs) lines 44–74. The sole normal implementation dispatches the engine trait directly to those synchronous functions, including `draft` and `offset_solid`, at lines 1785–1791.
* The failing tests bypass stdio registration entirely. Boolean and Offset tests construct `Body::new()` and call `make_*`, `boolean_solid`, `offset_solid_with_corner`, `thicken_face`, and `draft_angle` directly; see [`boolean tests`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔀️boolean/🧪️tests/🔬️unit/🦀️.rs) lines 151–281 and [`offset tests`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/↔️offset/🧪️tests/🔬️unit/🦀️.rs) lines 17–151. The engine red likewise creates `Brep::new()` and reaches `cut` after successfully creating, measuring, and tessellating sphere and torus ([`engine test`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🧪️tests/🔬️unit/🦀️.rs) lines 52–62). A registration omission therefore cannot cause these calls to select another backend.

## Current extraction parity check

Read-only `git diff --quiet` against `HEAD` reports no current-worktree delta for all four algorithms that define the reported groups:

| Source | Current worktree delta | Affected red cohort |
| --- | --- | --- |
| `🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` | none | 5 Boolean |
| `🧬️schema/🔺️diff/↔️offset/🦀️.rs` | none | 5 Offset |
| `🧬️schema/⚙️engine/🦀️.rs` | none | 1 engine preview cut |
| `🧬️schema/📸️snapshot/🔁️body/🦀️.rs` | none | 4 snapshot-body |

Both `semio-s-artifact-stdio-semio` and plugin `semio-s-plugin-stdio` Cargo manifests also have no current-worktree delta. The only six BREP-path changes in the current extraction diff are the generator configuration, mutation-harness sources, and two removed harness fixture assets; no BREP algorithm implementation changed. This is the relevant old-to-current source comparison. This audit makes no attribution claim about changes already committed before `HEAD`.

## Meaning of the live failures

The receipt's first diagnostics match active implementation paths rather than an unavailable capability:

* Boolean returns a constructed but invalid result (`shell-not-closed` / `non-manifold-edge`) at raw lines 150–153 and 217–238.
* Offset returns a body whose measured volume is wrong or zero at raw lines 225–257 and 3179–3184.
* The engine has already constructed and tessellated both inputs; its cut fails at the active imprint path, `imprint point does not lie on face boundary loop`, raw lines 775–777.

Keep these 15 failures in a separate geometry ownership lane. They block the standalone package's full suite, but no present feature/dependency or registration evidence supports treating them as stdio extraction regressions.

## Value absorb contract correction

The earlier first-cause report is corrected to reflect the current source. `absorb_named`'s `let mut added: Vec<T>` is not an index-free repair: its two Value call sites use `NamedAdded<T>` ([`Value diff`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🔺️diff/🦀️.rs) lines 158 and 523), and `apply_*` sorts on the embedded index at lines 340–345 and 364–369. `absorb_named` still merely pushes later additions at line 702.

The smallest sound repair is a transport-contract repair: represent named removals with their original position, preserve it through text/binary codecs, then replay named base/mid/after tokens to calculate surviving and later-added final indices. The alternative is to alter the base-free `MutationDiff::absorb` contract to accept the base and derive `between(base, after)`. The existing trait explicitly requires base-free structural composition ([`MutationDiff`](../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs) lines 105–119), so the positional named-removal form is the compatible route. Reindexing all additions or clamping their index is unsound: whether a removed key shifts an existing added entry depends on the missing original position.

No native command ran and no implementation was edited in this audit.
