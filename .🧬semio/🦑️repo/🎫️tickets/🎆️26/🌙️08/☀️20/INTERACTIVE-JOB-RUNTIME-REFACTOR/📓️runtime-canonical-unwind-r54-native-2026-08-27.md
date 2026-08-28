# Existing-Root Unwind R54 — One Fixture Frontier Not Injected

Actual R54 ran two of four selected canonical tests: **1 passed, 1 failed, 104 skipped**, with two selected tests not run after failure (0.097s). The full-grant comparison/retirement test passed again. The new unwind test recovered and typed-closed the actual comparison, copy and source-returned frontiers, then failed its `failed.is_err()` assertion at candidate-returned.

The fourth frontier already had a pending patch. `SurfaceReconcileCursor::step` therefore selected `advance_pending_patch` before the existing-component branch, and the new test-only injection in that branch did not run. No ownership failure was observed at that fourth frontier; it was not exercised by the intended panic.

The repair adds the same test-only post-step injection at the actual pending-patch branch. It does not change production work, Drop guards or retirement assertions. The test continues to retain the entire cursor outside `catch_unwind` and requires unchanged original document plus complete typed retirement.

Actual raw output: `🧪️member-runtime-canonical-unwind-r54-native-2026-08-27.txt`; session 76412 exited 1. Follow-up remains pending. No full unwind PASS is claimed from R54.
