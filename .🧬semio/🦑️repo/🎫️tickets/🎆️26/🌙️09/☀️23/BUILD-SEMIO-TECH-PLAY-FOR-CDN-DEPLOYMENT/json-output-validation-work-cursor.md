# JSON output validation work cursor

Descriptor generation `w73S37W` failed with E0599 at the JSON output receiver’s call to `JsonGrammarCursor<DslValue>::validation_work_units`.

The live source already contains the required public method. The receiver’s Cargo dependency resolves to `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml`. Its library root publicly reexports `🧮️parser-and-grammar-cursor/🦀️.rs` unconditionally. That module defines the inherent method on `JsonGrammarCursor<V>` for any `V: JsonParsedValue`. The borrowed decoder provides the implementation for `DslValue`.

A focused static check verified the manifest and reexport, the method and trait implementation, and the receiver’s funded call path. The method uses the remaining bytes beyond the cursor’s UTF-8 validation position. The caller clamps the quote to 1–256 units, then bounds it by the remaining item and work grants. A fresh effect still requires a cancellation checkpoint. The parse step uses the available effect work capped by the quote.

The concurrent workspace change resolved the source mismatch. This investigation made no production edits and started no Cargo command. A fresh descriptor compile is the remaining integration confirmation.
