# Note whiteboard verification

The Note TypeScript production parser matched the Ajv oracle for 66 native snapshots, 33 diffs and the invalid owner and child vectors. Strict TypeScript checking passed for the document oracle and its imported schema modules.

The action cohort suite passed both tests and all 25 assertions. Its source audit now checks the cold mutation preparation factory used by the Note store and stops scanning tool IDs at the current publication contract declaration.

All 17 mounted React tests passed across the InkCanvas editing, clipboard and interaction-domain suites. These exercise the real React host with fixture documents and simulated user events. The suites ran with one Vitest worker, after generating the JS icon catalog with the asset owner's generator.

Two clipboard regressions reproduced before the fix. Pasting now leaves the read-only Navigator unchanged, and text fields inside table cells keep their browser clipboard behavior. Pointer leave now clears the hovered block.

The current checkout passed the 17 mounted React tests again. The canonical asset owners published support shims and fonts, and the real ServeScript opened and validated the Note session and the font artifact. It reached Note's component check, which still requires the guest build to finish. The full live verification is tracked in the browser acceptance report.

WGPU pointer-exit routing passed 92 browser tests for input wire and frame transport. Rust parsing and whitespace checks passed. The cold native WGPU regression was canceled to free capacity for the Note guest and still requires execution.

The guest compiler exposed that Note image assets used a map without retained cloning. The document and snapshot now use the framework's retained ordered map. The existing document fixture supplies key and value expectations for both the Rust serde oracle and the TypeScript Ajv oracle. The TypeScript document oracle passed again after the representation change. The Rust regression is pending.

All thirteen Rust files changed for the Note asset-map fix passed the pinned nightly parser. Whitespace checks passed. A new guest build includes the fix after a transient PDF API mismatch corrected in the shared worktree.

The first build with the retained map compiled the Note schema. Downstream compilation exposed Rust orphan-rule errors because the split snapshot-codec crate implemented framework traits for a type owned by the schema crate. The codecs now compile under the snapshot type's owning crate. The published schema descriptor moved to its own crate to keep the native source boundary intact. The relocated modules and their twenty embedded schema facets all resolve, and the changed Rust files passed parse and whitespace checks. Native linking remains pending.

The shared multi-pass Cargo driver keyed adopted views with the explicit workspace configuration and compiler writer selectors. Its initial acquisition captured the caller's raw arguments, so the unit-graph continuation restored the workspace selector and was refused by the exact-scope check. Capture and adoption now use the same normalized declarations. The neutral fixture covers both a plain pass and a targeted pass with an opaque forwarded config token. The regression failed on the unpatched owner and passed after the correction against Node's argument-parser oracle. Four focused dependency tests passed with 51 assertions. The live guest stage passed the corrected gate and entered Cargo.
