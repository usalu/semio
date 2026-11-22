# Note Ink Tool Authority

On an empty Demo the Pencil button was visibly armed and the trusted pointer entered the real ink surface. A full body refresh was already in flight. The scene was still on Select when the pointer entered, and the gesture ran a selection action. A full editor run that warmed the canvas first happened to publish the new scene before its Pencil gesture.

The Browser shell owns one active utility register per window. Each interpreted window body now binds that value to the exact surface address used by the Interpreter's scene host. Ink pointer gestures and their selection chrome read the bound register, including an explicit clear. Without a matching registered window utility they use the scene's own utility. The scene's numeric controller was the local record ID, while the shell controller was the app declaration ID. The new binding uses the owning surface address common to both.

The neutral fixture covers activating against a stale Select scene, clearing against a stale Pencil scene, an unrelated surface, and an unwritten register. SQLite independently resolves the expected actions. The mounted React suite reproduced both stale-scene failures and passed all 47 tests across four related suites after the fix.

A fresh Chromium session on the actual attempt 55 guest passed the entire focused sequence. Pencil created one stroke in Canvas and Navigator, Point Eraser split it to two, Stroke Eraser removed one fragment, and the Undo sequence returned counts of two, one and zero in both panes.

The managed renderer TypeScript check returned 448 diagnostics across the shared tree, including missing generated manifest declarations and shell export mismatches. No diagnostic mentioned the new utility context or its props. The mounted suite and live browser scenarios were independent of that failed workspace check.

The expected shared UI contract projection was absent. Its repository-owned schema exporter passed and restored the TypeScript mirror. The renderer check is being repeated with that prerequisite present.

The UI contract reduced the renderer TypeScript diagnostics from 448 to 360. The manifest and actor exporters also restored their missing projections. The renderer typecheck target now declares the manifest, UI, actor and shell exporters as prerequisites. The repository planner reproduced the empty prerequisite list before the change and resolves all four owned executable targets afterward.
