# Plugin Localized Label Integration

## Native compile receipt

Native74 stopped in `semio-framework-plugin` with twelve production errors before the renderer and frame-turn law could compile. The compiler fingerprint is under `🗑️generated/astra-runtime/native-trace-isolated/build/debug/build/semio-framework-plugin/*/fingerprint/output-lib-semio_framework_plugin`.

Nine errors passed `Option<String>` or `String` into the newly typed `record_command(..., Option<LocalizedLabel>, ...)` boundary. One passed `Vec<LocalizedLabel>` from `ChildEmit` into the store's stale `ChildDispatch.labels: Vec<String>`. Two treated `LocalizedLabel` as one mutable `String` in the bounded child retirement cursor.

## Semantic repair

`Emit.description` remains `Option<String>`. It is persisted edit description/user data, so live emit, grouped emit, and synthetic transaction rows broadcast that exact runtime text through `LocalizedLabel::data`. This is an explicit data-label classification at each consumer; no `From<String>` compatibility conversion was added.

Typed commands use their declared action/command label matrix. Their durable edit description is no longer mistaken for authored UI copy. Tool runs similarly take the full `LocalizedLabel` from `registry.tool_run` instead of resolving the registry label to English through the persisted edit description. Framework `revertToCommand` rows pass no override and resolve the existing framework manifest definition (`Revert to Command` / `Auf Befehl zurücksetzen`). Shell-noted command labels are host runtime data and use `LocalizedLabel::data`, preserving the exact host text without declaring it English authored copy.

`ChildEmit.labels` and `store::ChildDispatch.labels` now share `Vec<LocalizedLabel>`. The dispatch seam therefore preserves the complete locale/terminology matrix rather than choosing a default locale or dropping forward audit metadata. Store fixtures with raw protocol labels classify those fixture strings explicitly as runtime data.

The child close cursor now uses `LocalizedLabel::texts_mut`. One close call removes at most one UTF-8 scalar from one label cell and reports its exact byte count; only a completely drained matrix releases the label item. The existing retained-child test now uses distinct multibyte English/German cells and still requires terminal-empty under the production grant. It does not truncate a matrix or pretend the matrix itself is one string.

## Verification boundary

Rustfmt parsed the five edited Rust files and `git diff --check` is clean. No Cargo command was run in this lane. Native compilation and the retained-child law remain root-owned. The exact focused law is:

```text
retained_child_wire_rejection_retires_nested_owners_under_the_production_grant
```

The frame scheduling packet remains recorded separately in `📓️astra-sol-frame-turn-scheduling.md`; its native law could not run because Native74 stopped first at this plugin compile boundary.
