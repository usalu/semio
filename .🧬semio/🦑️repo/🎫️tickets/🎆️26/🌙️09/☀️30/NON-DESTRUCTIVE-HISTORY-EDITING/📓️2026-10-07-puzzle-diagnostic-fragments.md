# Puzzle Borrowed Diagnostic Composition

Current MoveNode and CreateNode controlled producers return scalar dispositions and genuine native owner descriptors; they do not install a replay factory or reconstruct diagnostics. Root previously retained ownership of the generic final-clamped builder. The following exact composition requirements come from current physical cold mutation semantics; the builder must preserve severity before visible selection.

| Plan | Severity | Code | Body fragments | Target |
|---|---|---|---|---|
| Move nonfinite X/Y | Fatal | mutation.invariant | Static `newX must be a finite number` or `newY must be a finite number` | Borrowed mutation.id |
| Move target missing | Error | mutation.target-missing | Static `node "`, borrowed mutation.id, static `" not found` | Borrowed mutation.id |
| Move unchanged | Warning | mutation.no-op | Static `no changes to apply` | Borrowed mutation.id |
| Move changed | No diagnostic | — | — | — |
| Create nonfinite X/Y | Fatal | mutation.invariant | Static `node x must be a finite number` or `node y must be a finite number` | Borrowed mutation.node.id |
| Create invalid shape | Fatal | mutation.invariant | Static `node shape must be circle or rectangle, not ` plus quoted/escaped borrowed shape | Borrowed mutation.node.id |
| Create invalid radius/width/height/scale | Fatal | mutation.invariant | Static named field plus ` must be a finite number greater than 0` | Borrowed mutation.node.id |
| Create invalid handle angle | Fatal | mutation.invariant | Static `handle angle must be a finite number` | Borrowed mutation.node.id |
| Create invalid handle radius/scale | Fatal | mutation.invariant | Static named handle field plus ` must be a finite number greater than 0` | Borrowed mutation.node.id |
| Create duplicate ID | Fatal | mutation.duplicate-id | Static `node already exists` | Borrowed mutation.node.id |
| Create changed | No diagnostic | — | — | — |

Only one target segment is needed for these two families. Its known native UTF8 byte length must permit skipping an unfit target without copying its giant content. Bodies must copy only the admitted UTF8 prefix after charging the same96-byte row/24-byte target normative ledger. Static fragments and native text projections must remain externally immutable and retained through cursor closure. No severity can be downgraded because its visible body or target does not fit. No whole PagedUtf8-to-String input reconstruction is necessary.

The shape diagnostic needs a real quoted/escaped text fragment authority, because arbitrary shape text can contain quotes, control characters, newlines and multibyte Unicode. The current shared PagedUtf8 Debug implementation emits the physical `PagedUtf8(...)` tuple and creates a full temporary String via to_string_owner; that physical-owner display must not be treated as a controlled semantic diagnostic authority. The original text-domain shape message was based on quoted string debug output. A bounded builder should specify the semantic escaped output directly and compare against an independent escape oracle before adoption. Cold human debug display and unbounded cold mutation code are distinct from that hot authority.

Evidence was read directly from MoveNode/ CreateNode diff owners and puzzle2d_node_invariant/puzzle2d_shape on2026-10-07. This is an API/composition requirement, not a new native implementation receipt.
