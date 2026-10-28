# Native Reference Reduction Independent Audit

Read-only source checkpoint: shared IFC details `reference.rs`, `work.rs`, `pointer.rs` NativeAddress additions and `value_edit.rs` request handling. No production changes or native execution were performed by this audit. Authentic compiler ingress refuses before rustc; rustfmt parsing and authored tests do not establish type or runtime correctness.

## Corrected finding

P2: the initial typed-ancestor route adopted the imported constructor name before checking it against the attribute's original SELECT/defined domain. Because work skips duplicate DomainWork for an admitted ReferenceRequest, an entity tag such as IFCACTOR inside IfcValue could bypass the previous ancestor check while a nested reference changed. This was reported directly to the worker and parent.

The current correction retains `owner_expression`, requires one constructor item, resolves the name to an actual declared type rather than an entity, runs retained TypedMembership against the original domain, and only then adopts its child expression. TypeSearch accepts an exact declared type before rejecting aggregate/primitive descent, preserving the legitimate IfcPropertySetDefinitionSet aggregate-alias path. Defined domains require their exact constructor name; SELECTs search their members. The worker added rejected entity-tag and accepted aggregate-alias neutral cases. Source review confirms the reported bypass is closed in this checkpoint; execution remains unproven.

## Other inspected boundaries

Target search requires one matching source ID, checks actual constituent types against the declared entity/select domain and walks subtype ancestry. Direct reference SET/UNIQUE checking, aggregate append bounds/ARRAY refusal and optional unset routing are explicit. Canonical decimal virtual IDs are parsed incrementally without accepting signs, leading zeros or overflow. The original event, snapshot and addressed value are retained as identity pins. Reference requests are prevalidated before value staging; complex values retain their real parts and stage SetEntityComplex through the existing inverse route.

The cursor rejects cancellation before progressing and retains controlled retirement. Current source distinguishes metadata/key quote envelopes from actual copied key receipts; the worker is adding native prefix/short-loan/close laws. Those tests are not runtime evidence. No second concrete defect was confirmed in this bounded review. This report does not establish full EXPRESS validity, arbitrary imported complex-value validity, all inverse semantics, or end-to-end native renderer interaction.
