# Note Physical Adapter

Fresh React activation1 completed successfully in 3m15s (`🗑️generated/astra-runtime/activate-note-react-1.log`). The owned no-HMR listener runs at port 6380. The pre-existing 6080/6180 listeners are preserved.

Note1 mounted both Canvas and Navigator and exposed a duplicate-selector error in the physical adapter. All block, editor and image selectors now live under the canonical `note.play.composite` surface and owning `window:note-composite` host. Counts no longer accidentally include the Navigator copy.

Note2 reaches the exact editable block but does not publish the selected state. Its actual public ledger reports `setHover` and `setSelection` as refused, with `reason: undeclared-action`, for `note-composite`. `inkApplyEvents` is admitted. No successful Note interaction acceptance is claimed.

The source explains the mismatch: InkCanvasHost still emits `inkCanvasActions.setSelection` and `setHover`, while the Note app now owns selection through the framework-reserved `interactionSelect` and `interactionHover` commands on the `blocks` domain. Its `render_canvas_scene()` explicitly documents that the Ink scene still carries empty default selection and hover because the render path lacks an InteractionView. Repair must cover the scene domain contract, command envelope and current presence projection in both React and WGPU; merely renaming the command strings cannot establish live selection parity.
