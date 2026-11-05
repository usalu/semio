# Browser Scenario Audit

This is a read-only audit of the twelve authored Forms scenarios while the declared owners wait for capacity. None has executed in a live browser yet.

The source asserts exact values for form/question title authoring, deletion identity, 4800-byte initial pastes into both text kinds, complete submitted readonly answers, generic output 7 versus 2 for two question leases, and output 7 versus 4 for two simultaneous Fill windows. Both language contexts assert localized wizard/Run/source/response labels. The editor helpers wait for the actual attached session and echoed complete value; readonly results additionally assert the native readOnly property and non-editability.

Live cancellation asserts a real running transition (Run disabled and Cancel enabled), stopped status, restored controls and a subsequent successful geometry evaluation. Its catchable timing is unmeasured. There is no fabricated delay or progress. It does not yet prove that canceling one child leaves a simultaneously running other child untouched.

The building-component submission verifies the exact changed slider value and persisted 6.5 response, but geometry currently asserts only a nonempty mesh collection. It does not independently measure preview height6.5, so the preview-specific assertion could pass if the computation ignored the changed parameter. Cancellation restart likewise checks nonempty meshes rather than all four intended geometries. Generic numeric output assertions are stronger: they require exact computed values.

Window creation uses the existing Display Windows palette and a real template drag. Source identities are audited, but DOM affordances and split-drop coordinates remain unobserved. A failure there must be diagnosed from actual service execution, not assumed green from discovery.

No source changed for this audit. Actual browser/service startup, renderer typecheck and canonical cold-output helper checks remain pending.


Root authorized strengthening the identified preview gaps. The submission scenario now evaluates the initial geometry, independently reads public mesh data positions and asserts exact vertical extent6, changes the control, evaluates again and asserts extent6.5 plus changed geometry JSON. Cancellation restart measures all four distinct authored extents6,6.5,7,7.5, proving four distinct geometric bounds rather than any nonempty collection. These are browser test-only edits and do not change descriptor inputs. Actual runtime execution remains pending. Concurrent cancellation isolation remains unproven until the live UI can establish two genuinely running workloads.


Provider audit identified intentional dual preview records: each extrude neuron and its output-preview widget both project geometry. The browser coordinate oracle now compares distinct measured extents, so canonical duplicate previews do not cause a false failure. Submission requires the single unique6 then6.5 extent; cancellation recovery requires exactly four unique extents. Native laws separately scope all four output-preview identities and check every record height/radius.
