# Current Rust Source Accounting Audit

Static read-only review of `readRustSourceClosureV1`, cfg selector and Source admission. No builds/setup/Source mutation. Root owns the already discovered source/embedded overwrite and duplicate-read drift fix; those are excluded from new findings.

## P1: An Inner Attribute Can Hide the Following Module From Source Census

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts:35–36` calls `rustAttributes`, which recognizes outer `#[...]` only (compiler syntax owner `.../📚️compiler/📖️syntax/🦀️rust/🟦️.ts:601`). With source `#![allow(dead_code)]
mod child;`, the scope loop stays at the inner `#`, does not recognize `mod`, and generic boundary handling at Source lines 57–59 consumes through the child's semicolon. `child.rs` is omitted. The inspector's module path references cannot repair this, because Source line 29 explicitly ignores kind `path` references. The same issue occurs for a first module after inner attributes inside an inline module.

Handle inner attribute advancement explicitly before module discovery, sharing the syntax owner's existing inner handling at lines 496–506. Add neutral file/inline inner-attribute module fixtures and independently validate the real Rust module list using compiler dep-info. This is a concrete undercount route; no runtime reproduction was performed in this audit.

## P2: Inner Cfg Selection Is Absent

`selectRustSourceV1` at Source lines 114–116 only sees `#` directly followed by `[`. It leaves file/module `#![cfg(...)]` and `#![cfg_attr(...)]` active. Current actual authored examples include `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs:2` (`#![cfg(feature = "typegen")]`) and `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs:4` (non-wasm inner cfg). After fixing module scope advancement, inactive inner-cfg files will still contribute all active lines and traverse otherwise inactive imports, causing false capacity refusals or missing-input refusals.

Author neutral root-file and inline-module cfg/cfg_attr cases. Apply conditional inner attributes to the owning scope, then compare actual cfg-specific compiler dep-info and Source totals. Keep file bytes admitted as original Source even when active compiled lines are zero.

## P2: Ordinary Source Admission Reads Oversized Files Before Checking Capacity

`.../📥️source/🎟️admission/🟦️.ts:13` performs `readFileSync` after checking only file type; the wrapper at line 19 compares aggregate limits after the entire byte image has been allocated. A single oversized Source/embedded file can exhaust memory before the advertised precompiler capacity refusal. The collector itself has no budget parameter, so its port must admit file size against remaining budget before reading. The same wrapper also charges candidate module probes, which is conservative and should remain explicit.

Check lstat size before read against actual remaining admitted Source budget; verify bytes/identity after read. Add oversized-file refusal with a port spy proving bytes were not read. Preserve cancellation checks before reads and admitted-reader settlement.

## P2: Ordinary Admission Does Not Enforce Ancestry or Hardlink Custody

The same production `actualCargoSourceAdmissionPortV1.read` at line 13 checks the final file symlink only. `requireMaterializedPathV1` (`🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:39–42`) resolves text and checks length, not physical ancestry. A symlinked parent directory or hardlinked final file therefore passes this ordinary Source port. The selected Rust producer later adds ancestry/nlink checks in `selectedRustSourceFactsV1`; `admitCargoSelectionV1` at admission lines 24–26 has no such later guard.

Use the same original regular-file/ancestor custody at ordinary admission, with before/after identity facts. Add ancestral symlink and hardlink refusal oracles for the ordinary selection owner, not just selected graph publication.

## Boundary Notes

Source escape check at lines 18–19 rejects `..` prefixes but omits `isAbsolute(relative(root,path))`; different Windows drives need explicit refusal in the cross-platform owner. Collector `visited` keys include module base, while file accounting is keyed by physical workspace path; Root's ongoing dominance/drift fix must preserve traversal of distinct module bases while refusing differing byte images for one path.

Generated token ownership is explicitly a separate compiler dep-info frontier. Ordinary admission supplies `generated:()=>undefined` (line 19), so its authored census alone must not be presented as generated-consumer Source qualification. The current unsupported unit adapters and existing generated-input refusal laws remain necessary.
