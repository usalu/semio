# Deterministic plugin declarations

Plugin declaration assembly no longer seeds `RandomState` when constructing the app builder, checking its declaration membership, or initializing the plugin app and command indexes. `AppDefinition::terminology_breadcrumbs` now holds a `BTreeMap`. Its serialized object shape and replacement behavior remain the same, with keys emitted in lexical order. Existing app definition literal constructors use the field's default empty map.

Actions, modes, window kinds, terminology IDs and plugin commands remain in their authored vectors. Membership checks still report the first duplicate encountered in the authored sequence. Window ownership still removes the matching actions from only the app roster. The program's private indexes retain ID lookup, and app factory iteration now follows ID order. The runtime registry built inside `Plugin::new` already uses ordered collections, including artifact identity, inference, host media and flow registries. User and document state were outside this change.

The neutral fixture covers action ordering and ownership, first duplicate rejection, terminology order and last declaration replacement, and plugin command order. Bun validated it through Ajv and an independent SQLite query with 6 assertions. Two app builder tests and one program builder test consume that same fixture. The pinned Rust parser passed all 21 edited Rust sources, and the focused diff check passed. Native type checking and execution are pending the shared build lane. The existing manifest value codecs and retirement implementations support `BTreeMap<String, T>`.

The next pure path to audit is the declaration-level subset-example table in `declaration/surface/surface-builders`. It is a separate publisher and was outside this change.
