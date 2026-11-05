# Producer Selector Callers

Read-only audit, 2026-11-05. No source edits/native runs.

## Signature And Actual Callers

OriginalActionCommandBridge<C> now carries (OriginalCommandBirth<C>, fn(&C)->usize). admit_op_binary_producer now takes make,birth,maximum_for,cx,output. OpBinaryProducer stores maximum_for and selected maximum.

Tracked-source searches across repository (excluding ticket artifacts), framework and Lowpoly found:
- Lowpoly play-app:257 explicit bridge supplies command_from_action plus lowpoly_command_maximum_raw_bytes.
- Declaration op-binary:76 factory admit forwards self.0 and self.1 to producer.
- Declaration op-binary:83 generic declared bridge supplies |_|RUNTIME_COMMAND_WIRE_MAXIMUM.
- Command typed runtime:11 direct born-command producer supplies same default selector.
- Runtime fixture born helper:36 supplies default selector.
- Runtime bridge test:57 uses updated two-argument bridge.
- New runtime unit:90 passes explicit large/default max selector.
- Job facade reexports functions/types, not a constructor.
No stale one-argument bridge or old producer call found in production/tests/docs search results.

Default RUNTIME_COMMAND_WIRE_MAXIMUM remains65536; ordinary generic routes remain bounded. Lowpoly opt-in maps normal commands16KiB and import2MiB+4096+32. Earlier import-wire-bound-audit blocking default public producer ceiling is addressed by this explicit Lowpoly bridge; its old finding is historical inspected state, not remaining defect.

## Test Execution Gap

New owned_producer_limit_admits_large_wire_and_keeps_ordinary_limit is a cfg(test) unit in semio-framework-plugin-command at command/🎟️typed/🏃️runtime/🦀️.rs. Merely compiling it as Lowpoly dependency does not run it.

Permanent target is @semio-tech/lowpoly-plugin:test-runtime, owner 🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}. RuntimeTestScript currently selects seven Lowpoly packages via runRepositoryCargoTests, omitting semio-framework-plugin-command. Add that exact package to the selected list so its lib unit executes. Root/runtime notified. No edit made by this audit.

Command crate Cargo.toml workspace resolves repository root and it is an existing root workspace member. This is clean existing package selection, requiring no workspace migration/new dependency. Existing cargo package selection routes group packages by owner workspace. Target inputs default/^default plus Lowpoly subtree; command crate is already a transitive dependency, but explicit package selection is needed for tests. Scope/source freshness still follows canonical repo admission.

## Limits

Source caller audit, not a Rust compile or native pass. No claim that final tests ran. Current symbol signature migration has no obvious missing call sites.
