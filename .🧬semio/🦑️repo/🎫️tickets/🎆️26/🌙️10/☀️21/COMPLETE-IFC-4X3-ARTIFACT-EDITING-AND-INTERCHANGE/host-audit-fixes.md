# IFC4x3 Host Audit Fixes

All five findings in `host-audit.md` have source fixes and focused regression cases:

1. IFC4x3 command resolution now invokes the shared `complex_edit` authority after reference normalization and before ordinary rules. This handles omitted empty lists and retains the existing domain mutation/publication path.
2. Both primary and additional complex components use direct attribute declarations; entity type templates preserve positions in the same partial layout.
3. Every entity projects the complex field as an array even when native serialization omits an empty list. The field is reachable through fixed entity object keys and reports its real item count and creation capability.
4. Optional unset choices apply to the top-level attribute or the current optional item expression, with ARRAY optionality propagated on descent. Optional container declarations no longer make required scalar children optional.
5. Unique collection templates seek an unused admitted reference, enum or SELECT alternative, then bounded distinct scalar candidates. They no longer disable insertion immediately when one default value is present.

Rust tests and a language-neutral feature cover first complex insertion, primary partial attributes, empty complex reachability, optional container versus required element, and selecting the second admitted reference. Rustfmt parsing/formatting passes. **Rust unit/host tests remain unexecuted because the authentic compiler ingress is unavailable; these are source fixes, not runtime verification claims.**

The follow-up BINARY review also added IFCBINARY SELECT creation and a canonical empty binary template. Invalid binary text is refused in the shared central Rust diff applier before publication, including header, inserted entity, argument and complex payloads. The value check walks nested aggregates/typed values with lazy iterator cursors and retains ordinary schema-conformance diagnostics separately. A host test covers the control choice and atomic refusal of nonzero unused padding; it remains unexecuted with the same compiler limitation.

Reference and scalar candidate discovery remains bounded to 256 suggestions/candidates. Exact reference input can edit any existing slot, but inserting into an exhausted bounded candidate window still requires a complete searchable creation chooser. This is a remaining full-control/performance feature, not a claim of unlimited candidate selection. Existing full-model scheduling, compiler, browser accessibility and MVD limitations continue to apply.
