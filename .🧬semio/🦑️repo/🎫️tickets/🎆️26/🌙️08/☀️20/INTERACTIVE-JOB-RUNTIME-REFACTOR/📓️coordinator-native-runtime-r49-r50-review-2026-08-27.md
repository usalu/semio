# Coordinator Native Runtime R49–R50 Review

Root read both actual native outputs and the R49 report on 2026-08-27. R49 selected all 106 tests and actually stopped with **58 passed, two failed, 46 not run**, exit 1, 0.214 s. No full runtime pass is claimed.

The first failure is the existing large-tree strict timing test checking the new candidate before its canonical assembly seal: node count zero instead of 31. The implementation owner is adopting the real seal into that fixture while keeping the strict timing assertion on every seal step. The second failure is the real 48 KiB cursor representation limit.

R50, after removing a duplicate pending record, still actually fails that cursor bound: runtime output reports reconciler 760 bytes, cursor 53,712 bytes and retained owner 70,152 bytes. This is not repaired by the earlier existing-component 2/2 or nine-job 2/2 checks.

## Review Requirements Sent to the Owner

- Keep the 48 KiB cursor limit and 4 KiB work grants unchanged. Boxing a 12 KiB enum must not hide a whole stack temporary, initialization or root move; backing admission, in-place initialization, exact descendant transfer and eventual empty backing release all need accounting.
- Comparison/copy child completion may consume a full grant. The parent's immediate take_completed, lease/component transfer or copy-start must not add uncharged work, nor may lease-close released bytes be discarded. Add actual near-grant laws and separate transfer phases.
- The shared output-entry reservation must exist before producer work and root seal; post-seal allocation-free insertion alone does not establish admission.

The original inline physical-census failure, complete paired transaction output, all runtime unwind paths and all-app timing remain open. No native publication, limit increase, cleanup or competing Cargo run was performed by root.

Evidence: `📓️runtime-canonical-regression-red-r49-native-2026-08-27.md`, `🧪️member-runtime-canonical-regression-r49-native-2026-08-27.txt`, `🧪️member-runtime-canonical-layout-r50-native-2026-08-27.txt`.

## Serial Compiler Dependency Review

Root read the actual common-framework and UI Cargo manifests and checked repository Cargo references. The common framework's source joins UI contract, UI declarative types, Actor and OS kernel; the only explicit ui-runtime dependency found by that census is Plugin. The sole compiler owner confirmed that the common Kernel header test can run independently of the in-progress runtime representation, and is running that missing-API RED first. Root started no native compiler and did not change target/profile or create a second compiler lane. The proposed oversized Box was not credited as a fix; representation and transfer work remains open.
