# Snapshot registry module ownership

The semio-framework-value-snapshot-read Cargo manifest uses its ../../🦀️.rs library. That file path-mounts the physical OS store read registry retirement source as the public snapshot_read_registry_retirement module. The source is therefore compiled by the value snapshot-read package.

The new snapshot_registry_alias_work_demand is public through that module but is not included in the facade reexport beside the alias type, demands and close step; the store read import similarly does not expose it. Explicit canonical reexports are needed when using the root-level path.

Current Option close separates an already-closed slot removal into a grant-checked one-work turn; preceding owner steps retain their actual last_turn_work and report progress as nonterminal until that shell turn. No concrete defect found in that separation. Source-only review; no build.
