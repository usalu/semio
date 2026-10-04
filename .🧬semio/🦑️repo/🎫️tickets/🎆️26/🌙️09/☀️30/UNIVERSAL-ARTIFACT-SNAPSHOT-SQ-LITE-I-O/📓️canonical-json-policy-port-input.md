# Canonical JSON Caller Port Input

The owned Block2d/3d/5d, Puzzle3d, DAG, Jack and Process3d caller ports were one-time in-process file edits. No script file was retained and no migration exists.

For each actual Rust body, replace only the removed authority prefixes `dsl::json::`, `dsl_core::json::`, `store::json::`, and `protocol::json::` with `semio_framework_pack_json::`. A local `use dsl::json;` is removed only when its local `json::` calls are replaced by that same direct authority. Declare the actual first-party Pack JSON Cargo dependency in the owning manifest when missing.

For each canonical `from_json_str` call, locate its opening parenthesis after any balanced turbofish. Walk characters while tracking balanced parentheses, brackets, braces, quoted strings and escapes. Count commas only at call argument depth one and outside nested brackets/braces/strings. If there is no existing second argument, insert `, semio_framework_pack_json::JsonMemberPolicy::Reject` immediately before the matching closing parenthesis. Calls already carrying an explicit policy remain unchanged. Handle Rust comments/raw strings if present by inspection before applying this algorithm; text replacement alone is insufficient for nested calls. Reread every edited call and preserve custom policy semantics.

This is prerequisite authority repair, not SQLite feature runtime evidence. Root owns shared Plugin/Graph and Puzzle5d ports; this worker does not edit those scopes.
