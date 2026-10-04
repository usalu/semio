# Pure Helper String Inference Defect

Six authored ValueError::new calls pass literal.into() into its generic impl Into<String> parameter. The intermediate Into target is unconstrained. The canonical defining API takes impl Into<String>; these six callers should pass the original literal directly, preserving their messages and explicit typed refusal kind.

The seven-row source slice remains pending this correction and nominal caller/provider review. No compiler result is inferred from this source-level finding.

Proof: [six exact sites](🗑️generated/cargo-workspace-general-transfer/independent-os-pure-helper-generic-string-inference-defect-1.json).

Successor four independently closes the reported defect. All 187 helper/Text full source hashes and exact forward/inverse pairs pass; no malformed scoped canonical path remains. This is a finite correction admission, with broader provider/native closure still separate. Proof: [correction preflight](🗑️generated/cargo-workspace-general-transfer/independent-os-helper-four-text-four-correction-preflight-1.json).
