# FEM Serde Conversion Wave

The value derive supports `tag` plus `content`; FEM contains only supported `rename_all`, `default`, and internally tagged enum attributes. The previous additive derive conversion is therefore retained and serialization call sites can move directly to `ToValue`/`FromValue` and `json::{to_json_string, from_json_str}`.

The two standalone FEM JSON generator manifests also declared `serde_json`. They now use the owned `pack::json::Value` carrier. Its required mutable object/array accessors are first-party additions in the JSON codec.

Fixture adapters decode JSON text straight through `from_json_str`, retain canonical-object comparison against `DslValue`, and encode values through `ToValue::to_value`; no JSON text round trip is introduced for in-memory mutation values.
