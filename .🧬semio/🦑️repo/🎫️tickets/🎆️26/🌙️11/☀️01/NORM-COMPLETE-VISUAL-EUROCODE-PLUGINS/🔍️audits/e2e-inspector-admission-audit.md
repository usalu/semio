# Inspector Admission Audit

Source inspection only; no test or browser execution.

Base shared path: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/`.

## Confirmed Rendering Behavior

`🔖️field-meta/🦀️.rs` FieldSpec declares label/help/clause/kind/range/soft_range/step/precision/group/order/tier/visible_if/ndp/handle/affects. It has no readOnly, editable or action-admission field. FieldKind has Inferred/Text/Int/Bool/Angle/Ratio/Percent/Choice/EntityRef/Quantity and no readonly kind.

`🪟️visual-windows/🧾️properties/🎛️controls/🦀️.rs` FieldView likewise has no editability state. Its verb returns setField, or setFields for bulk. Every scalar branch in field_controls creates a bound toggle/select/stepper/input. Text and number inputs bind Commit; others bind Change. No set-rule capability check is performed.

`🪟️visual-windows/🧾️properties/🦀️.rs` render_node sends every scalar to field_row. field_row unconditionally appends field_controls. Only bytes and depth-limited nodes become description-only rows. shown_in filters tier and visible_if, not editability. Missing metadata falls back to a type-inferred control and remains visible. The inspector does not skip id or kind fields by name.

`🌳️report-tree/🦀️.rs` bind_on always resolves norm_action and attaches a trigger. It does not check native mutation availability.

All five non-Eurocode families (DIN4108, DIN16798, DIN18599, ISO16757, VDI3805) call render_inspector_for in their `✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs`. That shared helper builds NormHost::live and Engineering/Expert InspectorOptions. Its parameters contain family document, locale, controller id, field metadata function and tree windows; no edit-rules argument or mutability predicate is supplied.

## Concrete Unsupported Control

VDI `catalog.products[id=RAD-600].id` is a String and therefore renders an inferred text input with local ID `norm-inputs.catalog.products[id=RAD-600].id.input`. Its edit rules only provide product configuration and title replacements, not the product object or id. Therefore neither exact nor enclosing typed set rule covers this path. A commit dispatches setField but resolution refuses it. This is source-confirmed misleading action availability; no browser refusal was executed in this audit.

The identity field should not be made mutable merely to satisfy this control. Product/article identity has domain invariants and native keyed event semantics, documented separately in e2e-vdi-product-identity-audit.md.

## Existing Readonly Mechanisms And Limits

The norm FieldSpec/FieldView/control route has no existing readonly metadata mechanism. General JSON Schema readOnly is recognized as an annotation by framework schema validator at `🧰️framework/🔨️modules/🧬️schema/✅️validator/🔨️compile/🦀️.rs`, but this inspector does not read schema annotations, so adding readOnly to snapshot JSON alone would not change these controls.

The framework has separate scene-editor settingsJson readOnly and textarea target read_only mechanisms. Those do not establish a scalar norm-inspector admission policy and should not be treated as already integrated.

A future correction should preserve visible values while distinguishing supported mutation paths from intentional immutable metadata. A domain-owned metadata/admission interface is needed before general rendering changes; it must consider exact rules and nearest valid typed parents after descendant support, and must also suppress related mutating affordances such as NDP reset on inadmissible paths. This audit authorizes no production change and does not imply every scalar must become mutable.
