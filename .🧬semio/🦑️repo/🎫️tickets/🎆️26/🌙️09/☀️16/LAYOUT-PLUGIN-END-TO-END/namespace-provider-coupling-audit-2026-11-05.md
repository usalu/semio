# Namespace Provider Coupling Audit

Read-only; no source edit or new gate. Root owns focused85210 and product74324. Reviewed domain shortening audit, provider-manifest audit and residual audit against current Source.

## Python Styling Alignment

Current Python package `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/pyproject.toml:6` declares `semio-ui-styling`, version0.1.0. The editable root row in adjacent `uv.lock:48` has exactly the same distribution name/version. Both diffs change only the former `semio-framework-ui-styling` distribution name. Build backend remains hatchling1.27.0; Python range remains>=3.14,<3.15. Python import module remains `🎨️styling.🐍️`; `[tool.hatch...] packages/force-include` still names the actual module, not a renamed Rust crate.

Producer schema `🎨️styling/🏗️builder/🐍️python/🧬️schema/🔣️.json:148` now requires the same name. Output fixture `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/🎨️styling-outputs/🐍️python/🔣️.json:8` and builder fixture `library/🎨️styling/🏗️builder/🐍️python/🧪️tests/🧫️fixtures/🔣️.json:5` both use `semio_ui_styling-0.1.0-py3-none-any.whl`, consistent with hyphen→underscore wheel distribution normalization. Dependency-manager actual package fixture and embedded uv-lock text also use the new name. No old hyphen or underscore styling package token remains in the inspected styling producer/consumer/output-fixture scopes.

Source alignment is complete; no actual new wheel-build result is present in this ticket's retained styling/Python logs.

## Smallest Owned Proof

Existing neutral builder test `library/🎨️styling/🏗️builder/🐍️python/🧪️tests/🟦️.ts`, named `neutral actual Python Source contract keeps old portable wheel vectors`, compares actual TOML through Bun and @iarna/toml and validates actual manifest against the updated schema. Its imported `library/⚡️caching/🧪️tests/🐍️styling-python-outputs/🟦️.ts:8` checks backend, project targets and compiler bundle ownership, but never checks `fixture.wheel` or uv.lock's editable root name. The generic WheelName test checks portable filename syntax only. Root's Cargo dataset/path tests cannot establish Python provider behavior.

Smallest existing owned end-to-end check is `repo run @semio-tech/ui-styling-py:test:prototype` (do not run both modes for this name-only change). Its actual project manifest declares test→build→deps and token generation. Actual package command `🎨️styling/🧪️tests/🔬️python-package-commands/🟦️.ts` imports source and the published wheel through the issued interpreter. Producer `library/🎨️styling/🏗️builder/🐍️python/🟦️.ts:61` executes locked offline Hatchling and publishes exactly one admitted wheel; consumer63 reads the published wheel. This proves actual locked provider build and package import. It does not independently assert the fixture's expected filename/METADATA identity: a precise coupling regression law would additionally compare actual editable lock name and normalized expected wheel name to actual pyproject.name, and inspect real wheel METADATA Name after the owned build. No new implementation or gate was performed in this audit.

## Semantic Identifier Spot Checks

No demonstrated causal identifier corruption in inspected rows. UI presentation schema diff changes only a Rust crate-qualified comment. OS MCP schema changes a Rust API name in documentation. Plugin descriptor-emitter protocol's binary changes to `semio-plugin-describe`, matching renamed Cargo executable semantics. UI ownership owner constant changes to `semio-ui-contract`, which is the actual renamed canonical Rust owner; this is deliberately coupled authority DATA, not a language-neutral schema ID. Existing `$id` values remain unchanged in these schema diffs. Replication ownership negative case at `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts:166` still substitutes `semio_os_hostile` for the exact replication owner and remains foreign. Independent build project names are not Cargo names and must retain their actual declared IDs. This was a focused causal spot check, not a claim that all21,164 changed files received semantic review.
