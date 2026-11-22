# Original Media Picker Cancellation

The renderer’s strict TypeScript check flagged the cancellation promise after a call to the closure that assigned it. The picker now awaits the memoized retirement directly. It returns cancellation only after the actor exchange accepts retirement and an ownership check fails. A failed retirement preserves both errors when the picker initiates the close. If the sender already attempted retirement, its existing error is preserved.

The renderer’s media and file suite reads the language-neutral cases in `original-media.json`. The cases cover a complete two-page transfer, ownership lost while picking a file, a dismissed picker, cancellation after the first acknowledged page, and a rejected retirement. The test compares the page payloads against independent Blob slices and checks the progress, control frames, and transfer cleanup.

The full strict renderer TypeScript run cleared TS18048 in the picker. It still reports 27 diagnostics elsewhere in the renderer and its dependencies.

The policy-owned React renderer Vitest run at the long test level passed all 15 tests in the media and file suite, including all five picker cases. `git diff --check` passed.
