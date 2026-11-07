# 1105b Policy Audit (aud-policy)

Read-only audit of source changed since 2026-11-03 under `🧰️framework/`, `✏️s/`, `🌎️hub/`, `♻️mit-bestand/`, against AGENTS.md. Nothing in the repo was edited, and no git modifying command was run. Read-only git only (`log`, `diff`, `ls-files`, `check-ignore`).

## Scope and method

- Changed files: 53,029 path entries (unique, with rename skipped). Existing `.rs`/`.ts`/`.tsx`/`.mjs` in scope: 29,139 (after dropping node_modules, target, dist). `.json` in scope: 21,755.
- Current-state scan: Python over the working tree. Test paths are excluded (`tests/`, `/test/`, `unit/` under 🔬, `.test.`, `.spec.`). Rust inline `#[cfg(test)]` blocks are excluded too.
- Introduced-lines scan: `git diff --no-renames -U0 15d755f053c HEAD` (last commit before 2026-11-03), added lines only. Test paths are excluded by path only.
- Closure flag: rows under `🌎️hub/` are marked as in the closure (hub `semio-hub-*`). Framework and plugin rows are marked "no / untraced" because the Cargo dependency graph was not walked. Treat them as candidates.
- Crate attribution: nearest `Cargo.toml` or `package.json` name. Root-level fallbacks show as `workspace`.
- Caveat: heuristic rules (2, 5, 7) need manual review. Counts are signals, not confirmed violations.

## Summary

| # | rule | current non-test hits | introduced since 15d755f | status |
|---|---|---|---|---|
| 1 | unwrap/expect | 10138 | 2828 | violation |
| 2 | indented comments (fn bodies) | 1464 | 19 | heuristic, review |
| 3 | docstring not starting with emoji | 136 | n/a | 105 confirmed after keycap correction |
| 4 | [DEBUG] / debug print leftovers | 42 tagged, 334 print | 21 print | violation |
| 5 | compat/legacy/adapter/deprecated | 1739 | 316 | heuristic, mostly domain terms |
| 6 | #[allow] / lint suppressions | 3059 | 565 | violation (greenfield) |
| 7 | raised limits/constants | 1109 | 190 | review |
| 8 | UI default language / de missing | 0 defaults | 0 | no default-language violation found |
| 9 | path >= 256 chars | 0 chars (max 212) | n/a | pass by char count |
| 10 | migration scripts | 0 | 0 | pass |
| 11 | source copied into ticket folders | 10 | n/a | violation (all git-tracked) |

## Rule 1: `.unwrap()` / `.expect(` in non-test Rust

- Current state, non-test code (scan excludes `tests/` paths and inline `#[cfg(test)]` blocks): **10138** hits in 1724 files. Top crates: workspace (5139), semio-s-workspace (4649), @semio-tech/ui (165), @semio-tech/framework-manifest (110), @semio-tech/ui-contract (34), @semio-tech/repo-product (16), semio-hub-workspace (5), semio-s-artifact-stdio-gif-schema-2 (2).
- Introduced since base `15d755f053c` (added lines, test paths excluded, inline cfg(test) not excluded): **2828** hits. Top crates: semio-s-workspace (1438), workspace (1321), @semio-tech/framework-manifest (40), @semio-tech/ui-contract (24), @semio-tech/framework-interaction (2), semio-s-artifact-stdio-jpg-io-registry (1), semio-os-mcp (1), semio-os (1).
- Hub (`semio-hub-*`) hits: 5 current, 0 introduced.
- Rows: first 40, hub first, then by crate.

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | semio-hub-workspace | `🌎️hub/🔐️auth/🔌️client/🦀️.rs:66` | yes (hub, semio-hub-*) | `.expect("Hub agent request strings serialize")` |
| 2 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎚️config/🦀️.rs:330` | yes (hub, semio-hub-*) | `let spec_fn = variants.iter().find(\|(k, _)\| k == &keyword).map(\|(_, s)\| *s).expect("variant spec must exist for its own ` |
| 3 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎚️config/🦀️.rs:98` | yes (hub, semio-hub-*) | `let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id()` |
| 4 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/🦀️.rs:216` | yes (hub, semio-hub-*) | `let spec_fn = variants.iter().find(\|(k, _)\| k == &keyword).map(\|(_, s)\| *s).expect("variant spec must exist for its own ` |
| 5 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/👥️presence/🦀️.rs:60` | yes (hub, semio-hub-*) | `let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id()` |
| 6 | @semio-tech/framework-interaction | `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🦀️.rs:77` | no / untraced | `let frame = self.pinch_frame().expect("two contacts have a pinch frame");` |
| 7 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:219` | no / untraced | `let schema = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(INPUT_NUMERIC_TRANSPORT_SCHEMA_JS` |
| 8 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:221` | no / untraced | `let number = input_union(&definition("Binary64Transport")).and_then(\|branches\| branches.iter().find(\|branch\| branch.get(` |
| 9 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:74` | no / untraced | `let document = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(INPUT_LABEL_GLOSSARY_JSON, semi` |
| 10 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:75` | no / untraced | `let labels = document.get("labels").and_then(DslValue::as_object).expect("the input label glossary carries labels");` |
| 11 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:76` | no / untraced | `labels.iter().map(\|(name, label)\| (name.clone(), input_localized_text(label).expect("every glossary label names every lo` |
| 12 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:14` | no / untraced | `assert_eq!(serde_json::to_value(action_arg_shape_seed(&input)).unwrap(),expected);` |
| 13 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:16` | no / untraced | `validate_action_arg_shape_seeds(&[input],&seeds).unwrap();` |
| 14 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:17` | no / untraced | `assert_eq!(seeds.len(),6);assert_eq!(seeds[0].variant_selector,Some(0));assert!(seeds.iter().skip(1).all(\|seed\|seed.vari` |
| 15 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:24` | no / untraced | `assert_eq!(serde_json::to_value(arg_schema_shape_seed(&input.schema)).unwrap(),serde_json::json!([3,3]));` |
| 16 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:27` | no / untraced | `validate_action_arg_shape_seeds(&[input],&seeds).unwrap();assert_eq!(seeds.len(),2);assert_eq!(serde_json::to_value(&see` |
| 17 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:28` | no / untraced | `assert_eq!(seeds[1].schema_path,vec![0,0]);assert_eq!(serde_json::to_value(&seeds[1].value).unwrap(),serde_json::json!(3` |
| 18 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:39` | no / untraced | `assert_eq!(serde_json::to_value(seed).unwrap(),serde_json::json!({"mode":"solid","length":""}));` |
| 19 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:44` | no / untraced | `let law:serde_json::Value=serde_json::from_str(include_str!("seed-cases.json")).unwrap();` |
| 20 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:45` | no / untraced | `for row in law["cases"].as_array().unwrap(){` |
| 21 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:46` | no / untraced | `let schema:ArgSchema=serde_json::from_value(row["schema"].clone()).unwrap();` |
| 22 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:47` | no / untraced | `let value=serde_json::to_value(arg_schema_shape_seed(&schema)).unwrap();` |
| 23 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:55` | no / untraced | `let original=action_arg_shape_seeds(&[input.clone()]);validate_action_arg_shape_seeds(&[input.clone()],&original).unwrap` |
| 24 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:65` | no / untraced | `let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🛂️manife` |
| 25 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:66` | no / untraced | `for row in law["cases"].as_array().unwrap(){` |
| 26 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:67` | no / untraced | `let schema:ArgSchema=serde_json::from_value(row["schema"].clone()).unwrap();` |
| 27 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:69` | no / untraced | `input.default=row.get("default").map(\|value\|serde_json::from_value(value.clone()).unwrap());` |
| 28 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:70` | no / untraced | `let rows=action_arg_shape_seeds(&[input.clone()]);validate_action_arg_shape_seeds(&[input.clone()],&rows).unwrap();` |
| 29 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:71` | no / untraced | `assert_eq!(serde_json::to_value(&rows[0].value).unwrap(),row["expectedSeed"]);` |
| 30 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/🧪️tests.rs:72` | no / untraced | `assert_eq!(serde_json::to_value(&rows[0].singleton).unwrap(),row["singleton"]);` |
| 31 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:103` | no / untraced | `for row in cases["cases"].as_array().unwrap(){` |
| 32 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:104` | no / untraced | `let binding=StagedFormBinding{token:token.clone(),layout:StagedFormLayout::Replace,edits:vec![serde_json::from_value(row` |
| 33 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:106` | no / untraced | `let mut plan=binding.plan(&status,input.as_ref()).unwrap();` |
| 34 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:107` | no / untraced | `let (kind,path,value)=match plan.next().unwrap(){` |
| 35 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:11` | no / untraced | `assert_eq!(!refused,valid);assert_eq!(output.is_some(),valid);if let Some(request)=output{assert_eq!(serde_json::to_valu` |
| 36 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:112` | no / untraced | `assert_eq!(kind,row["kind"].as_str().unwrap());` |
| 37 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:113` | no / untraced | `assert_eq!(serde_json::to_value(path).unwrap(),row["path"]);` |
| 38 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:124` | no / untraced | `let fixture:serde_json::Value=serde_json::from_str(include_str!("limits-cases.json")).unwrap();` |
| 39 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:125` | no / untraced | `let caps:StagedFormLimits=serde_json::from_str(include_str!("limits.json")).unwrap();` |
| 40 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/tests.rs:128` | no / untraced | `for row in fixture["cases"].as_array().unwrap(){` |

## Rule 2: comments inside definitions (indented `//`)

Heuristic: indented `//` line comments (not `///`/`//!`), non-test paths. Could include comments inside struct/enum/match bodies, not only fn bodies; needs manual review.
- Current: **1464** hits. Top crates: semio-s-workspace (927), workspace (300), @semio-tech/ui (145), @semio-tech/repo-product (29), semio-os-kernel (5), semio-s-artifact-stdio-svg-mutations-2 (4), semio-s-artifact-stdio-binary-io-registry (4), semio-s-artifact-stdio-wav-io-registry (4).
- Introduced: **19** hits.

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🗂️media-vfs/🦀️.rs:50` | yes (hub, semio-hub-*) | `// 🚧️ 'flatten_media_vfs_rows' is a no-op stub (os-core dissolve deleted the '🔖️WorkflowVfs' region` |
| 2 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/📜️space-manifest/🦀️.rs:152` | yes (hub, semio-hub-*) | `// the app instance now — see the kernel '🔁️workflow' crate's '🔖️InstanceIdentity' doc),` |
| 3 | @semio-tech/framework-effect | `🧰️framework/🔨️modules/🎠️kernel/⚡️effect/🟦️.ts:82` | no / untraced | `// --- new variants (📓️design-abi.md §2's table; nothing constructs these yet) ---` |
| 4 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:24` | no / untraced | `// 🎫️ W0 (Crate Consolidation & Plugin Taxonomy Restructure, root-script-policy-revival ticket):` |
| 5 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:240` | no / untraced | `// 🌱️ The original allowlist keyed off an enclosing 'pub mod d3 { … }' wrapper (one shared crate,` |
| 6 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:241` | no / untraced | `// multiple apps namespaced by mod); today (and in the future taxonomy) each app is its own crate,` |
| 7 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:242` | no / untraced | `// so fall back to the app id from the crate's own path when there's no such wrapper.` |
| 8 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:25` | no / untraced | `// downgraded from "high" the moment the discoverer revival first ran this rule against real` |
| 9 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:26` | no / untraced | `// crates — ~1850 pre-existing "// #region" (space) markers across the plugin tree had never` |
| 10 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:27` | no / untraced | `// been checked before (the old discoverer matched nothing), so this was never a hard gate in` |
| 11 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:28` | no / untraced | `// practice. Fixing that many markers spans hundreds of files outside this ticket's script.ts-only` |
| 12 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:289` | no / untraced | `// 🌱️ Line-based lookback (not a fixed char window): the 'app_labels!' invocation can sit several` |
| 13 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:29` | no / untraced | `// ownership; WARN-only until a dedicated formatting sweep ticket cleans it up, then flip back to "high".` |
| 14 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏗️crate-shape/🟦️.ts:290` | no / untraced | `// lines above the struct decl behind a multi-line /// doc comment (e.g. remodel, procedural).` |
| 15 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:208` | no / untraced | `// stacked attribute — keep waiting for the fn line` |
| 16 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:302` | no / untraced | `// 🧪️ Test files are exempt: a test that drives the storage adapter, or asserts that an` |
| 17 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:303` | no / untraced | `// ephemeral box was retired, must be able to NAME the thing it is testing.` |
| 18 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:311` | no / untraced | `// 🎯️ Mask literals, then strip BOTH comment forms: this rule greps raw source, and prose` |
| 19 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:312` | no / untraced | `// explaining that a helper was retired must not itself trip the rule — a hazard this repo's` |
| 20 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:313` | no / untraced | `// vocabulary policies have already been bitten by. '//' alone is not enough; the block-comment` |
| 21 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:314` | no / untraced | `// continuation (' * …') is exactly where a docstring mentioning 'localStorage' lives.` |
| 22 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:640` | no / untraced | `// 🎪️ BANNED, not downgraded (ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION` |
| 23 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:641` | no / untraced | `// D2): the demonstrator's twelve foreign-kind registrations moved to their owners (📐️cad, 🌍️gis),` |
| 24 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:642` | no / untraced | `// and 'semio-os' is gone from its Cargo.toml. The empty array is the ratchet — if the` |
| 25 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:643` | no / untraced | `// dependency is ever reintroduced, 'policyPluginDependencyAllowlistBreaches' flags it with no` |
| 26 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts:644` | no / untraced | `// carve-out to soften the message.` |
| 27 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts:86` | no / untraced | `// 🧪️ Owner-root test phases, routed through the testing domain's own router.` |
| 28 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts:87` | no / untraced | `// 🎚️Both 'test quick parity' and 'test parity quick' must mean the same run. The first form is` |
| 29 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts:88` | no / untraced | `// already resolved above (and exported through SEMIO_TEST_LEVEL); the second is resolved over the` |
| 30 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts:89` | no / untraced | `// segments that FOLLOW the phase word. Which phases exist, and where the router that serves them` |
| 31 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts:90` | no / untraced | `// lives, are both taxonomy vocabulary — this file never enumerates either.` |
| 32 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts:232` | no / untraced | `// 'name.workspace = true' dotted shorthand — must be checked before the generic key=value parse below (its "name" would` |
| 33 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:359` | no / untraced | `// 🏠️ "Space" here is a deliberate, deferred duplicate of the host plugin's own manifest label` |
| 34 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:360` | no / untraced | `// ('App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space"))',` |
| 35 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:361` | no / untraced | `// ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs:869) — reading it from there via` |
| 36 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:362` | no / untraced | `// 'resolveManifestLabel(hostApp.label, …)' (the same pattern 'appWindowLabel' already uses)` |
| 37 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:363` | no / untraced | `// is the correct fix, EXCEPT 'ShellHost/🟦️.tsx''s 'hostApp' lookup (line ~1132,` |
| 38 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:364` | no / untraced | `// 'manifest.apps.find(app => app.id === hostConfig?.hostAppId)') is ALREADY always` |
| 39 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:365` | no / untraced | `// 'undefined': 'hostConfig.hostAppId' is the raw Cargo.toml 'host = { shell = "studio" }'` |
| 40 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧭️shell-chrome/🟦️.ts:366` | no / untraced | `// alias, never the real dialect-derived 'AppDefinition.id'` |

## Rule 3: docstrings not starting with an emoji

Rust doc comments (`///`, `//!`) whose first text is not an emoji. Keycap emoji (`#️⃣`) and symbol emoji (`ℹ️`, `©️`) counted as emoji. Raw doc-block first-lines: 136; true violations after keycap correction: **105**. Introduced-line check not separated.

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓️actions-fields-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 2 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-sls-rail-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 3 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️test-series-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 4 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️building-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 5 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️design-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 6 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️foundations-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 7 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️layer-water-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 8 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️investigation-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 9 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️nested-rows-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 10 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermal-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 11 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️building-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 12 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️zones-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 13 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:1011` | no / untraced | `/// Footbridge deck with loaded length, width and first vertical natural frequency: uniform load q_fk = 2.0 + 120/(L + 3` |
| 14 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:1029` | no / untraced | `/// Railway span with its determinant length L_Φ and track maintenance: the dynamic factor Φ_2 (careful maintenance) or ` |
| 15 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:1042` | no / untraced | `/// Wheel load spreading at 45 degrees through pavement and half the slab to the mid-plane (EN 1991-2, 4.3.6): the footp` |
| 16 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:1060` | no / untraced | `/// Bulk solid with its angle of internal friction: the lateral pressure ratio K = 1 - sin(phi_i) and its mean K_m = 1.1` |
| 17 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:1071` | no / untraced | `/// Rotating machine with unbalanced mass, eccentricity and rotation frequency: the equivalent static force F_dyn = m e ` |
| 18 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:997` | no / untraced | `/// Hoist of the crane with its rated load Q_H and the payload fraction released suddenly (EN 1991-3 2.5 and 2.9): the d` |
| 19 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️falling-loads-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 20 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️general-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 21 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermal-sections-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 22 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️wind-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 23 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️cranes-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 24 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️silos-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 25 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️execution-imperfections-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 26 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️fire-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 27 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚆️derailment-cases-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 28 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️resistance-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 29 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️member-identity-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 30 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️project-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 31 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️members-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 32 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧬️geometry-tendons-studs-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 33 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️geometry-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 34 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️definitions-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 35 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-deck-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 36 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 37 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-observed-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 38 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️member-action-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 39 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️crane-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |
| 40 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️crane-rail-radius-more/🦀️.rs:1` | no / untraced | `//! more mutation leaves` |

## Rule 4: leftover `[DEBUG]` logs and debug output

[DEBUG]-tagged lines in the repo: **42** (in 27 files; 40 in test paths). All are leftovers: tagged debug output still present in test code.
- Non-test `println!/eprintln!/dbg!/console.*`: **334** hits (top crates: semio-os-kernel (62), @semio-tech/repo-product (58), workspace (54), semio-plugin (40), semio-hub-space (15), semio-os (14), @semio-tech/framework-renderer-wgpu (13), @semio-tech/plugin-registry (12)). Introduced since base: **21**.

[DEBUG] leftovers (all 42; first 40):

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:336` | no / untraced | `if(selection.kind==="products-graph-fixture"){if(!row.producer\|\|!row.fixture)throw Error("Current exact fixture and know` |
| 2 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts:41` | no / untraced | `expect(text).toContain('row.args.includes("--offline")&&scope.selections.has(');expect(text).toContain("offline:false");` |
| 3 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🕸️dependency-edges/🧪️tests/🧵️edge-reads/🟦️.ts:106` | no / untraced | `console.log("[DEBUG] edge-reads soundness",JSON.stringify({runs,kept,changed,flagged,perEdit}));` |
| 4 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/👷️child/📜️script.ts:9` | no / untraced | `await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"` |
| 5 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:21` | no / untraced | `const crossSpawn=createRequire(import.meta.url)("cross-spawn"),independent=crossSpawn.sync(process.execPath,["-e",'const` |
| 6 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:23` | no / untraced | `const delivered=await captureOwnedCommandResultV1(process.execPath,[wrapper,"capture-program",root,entry],root,{control:` |
| 7 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🟦️.ts:26` | no / untraced | `test("fresh canonical build and Nx factory schema matches the actual checkout",()=>{const root=resolve(process.cwd()),bu` |
| 8 | semio-s-workspace | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📨️child-preparation/🦀️.rs:183` | no / untraced | `other=>panic!("[DEBUG] actual child owner did not close: {other:?}"),` |
| 9 | semio-s-workspace | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs:340` | no / untraced | `assert_eq!(result.get("done").and_then(\|value\|value.as_bool()),Some(false),"[DEBUG] compact output phase {phase}: {resul` |
| 10 | semio-s-workspace | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:122` | no / untraced | `console.error('[DEBUG] Shooting independent complete SQLite tables='+names.length+' rows='+totalRows+' cells='+totalCell` |
| 11 | semio-s-workspace | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:52` | no / untraced | `let script="import{Database}from'bun:sqlite';const db=new Database(':memory:');try{db.exec(process.env.SEMIO_PLAYGROUND_` |
| 12 | semio-s-workspace | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs:325` | no / untraced | `eprintln!("[DEBUG] n={n} step {steps} phase-capacity {}", p.retained_capacity_bytes);` |
| 13 | semio-s-workspace | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs:331` | no / untraced | `eprintln!("[DEBUG] n={n} steps {steps} total capacity {total}");` |
| 14 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:283` | no / untraced | `PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenanc` |
| 15 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:284` | no / untraced | `::semio_framework_async::poll::resolve_ready(app.0.advance_typed_operation_publication(&mut semio_plugin::artifact_app_l` |
| 16 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:286` | no / untraced | `app.acknowledge_typed_operation_result(page.token).expect("[DEBUG] ack");` |
| 17 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:290` | no / untraced | `let _ = ::semio_framework_async::poll::resolve_ready(app.take_typed_operation_completion()).expect("[DEBUG] completion")` |
| 18 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:297` | no / untraced | `PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenanc` |
| 19 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:298` | no / untraced | `::semio_framework_async::poll::resolve_ready(app.0.advance_typed_operation_publication(&mut semio_plugin::artifact_app_l` |
| 20 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:300` | no / untraced | `app.acknowledge_typed_operation_result(page.token).expect("[DEBUG] ack");` |
| 21 | semio-s-workspace | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs:304` | no / untraced | `let _ = ::semio_framework_async::poll::resolve_ready(app.take_typed_operation_completion()).expect("[DEBUG] completion")` |
| 22 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:103` | no / untraced | `let script=r#"import{Database}from'bun:sqlite';const c=JSON.parse(process.argv[1]);const d=Database.deserialize(await Bu` |
| 23 | semio-s-workspace | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:89` | no / untraced | `console.error('[DEBUG] Raster independent SQLite complete tables='+names.length+' rows='+names.reduce((sum,name)=>sum+ro` |
| 24 | semio-s-workspace | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:87` | no / untraced | `let oracle = "const source=await Bun.stdin.text(); const names=[]; const attrs=[]; const parser=new HTMLRewriter().on('*` |
| 25 | semio-s-workspace | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/📄️natural/🦀️.rs:274` | no / untraced | `eprintln!("[DEBUG] semio-current-article {}",serde_json::json!({"schema":E::DOCUMENT_SCHEMA,"runs":384,"marks":344,"dsl"` |
| 26 | semio-s-workspace | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/📄️natural/🦀️.rs:88` | no / untraced | `eprintln!("[DEBUG] semio-current-example {}",serde_json::json!({"subset":subset,"declaredSchema":E::DOCUMENT_SCHEMA,"sch` |
| 27 | semio-s-workspace | `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎥️shooting-route/🟦️.ts:41` | no / untraced | `console.log("[DEBUG] shooting authored route, TOML oracle, contribution schema and TypeScript syntax checks passed");` |
| 28 | workspace | `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/dag.ts:17` | no / untraced | `console.log("[DEBUG] public control DAG real-clock counts "+JSON.stringify(measured));for(const row of measured)expect(r` |
| 29 | workspace | `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/freshness.ts:35` | no / untraced | `console.log("[DEBUG] public control DAG freshness "+JSON.stringify(measured));` |
| 30 | workspace | `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/freshness.ts:58` | no / untraced | `console.log("[DEBUG] public open-wait freshness "+JSON.stringify({authorized,delayed,observed}));` |
| 31 | workspace | `🧰️framework/🔨️modules/🔲️pixels/✍️editing/🪣️flood/🧪️tests/large-close.test.ts:25` | no / untraced | `console.log("[DEBUG] large flood close probe "+JSON.stringify({retained,released,closed:cursor.closed,initialQuote,grant` |
| 32 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:31` | no / untraced | `for _ in 0..1_000_000{if registry_retirement.terminal_is_empty(){break}let g=RetainedCloneGrant{maximum_items:1,maximum_` |
| 33 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:65` | no / untraced | `assert_eq!(observe(\|\|drop(decoder)),((),0,0));assert_eq!(original+births,releases,"[DEBUG] original={original} births={b` |
| 34 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:10` | no / untraced | `for step in 0..200000{if cutoff==Some(step){break}let source=events.get(position).map(\|event\|NativeSupersessionSource{ev` |
| 35 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:13` | no / untraced | `if cutoff.is_none(){assert!(complete,"[DEBUG] native supersession coroutine must finish at event={position} quote={:?}",` |
| 36 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧪️tests/original-view-read.rs:223` | no / untraced | `println!("[DEBUG] actual actor view pin job aliases returned while pinned: {}",job_aliases_while_pinned);` |
| 37 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🛬️lifted/👷️job/🧪️mutation/🦀️.rs:20` | no / untraced | `if row["gate"]=="poison"{let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(\|\|{let mut lease=handle.inner.try_l` |
| 38 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32` | no / untraced | `for index in 0..1048576{if index<512{eprintln!("[DEBUG] native opening turn={index}");}let(_,p)=turn(&mut caller,&mut de` |
| 39 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🫧️ephemeral/🧪️tests/🦀️.rs:9` | no / untraced | `fn original_ephemeral_source_same_roots_cancelled_prefixes_and_full_physical_close(){let law=law();for cutoff in std::it` |
| 40 | workspace | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧱️components/🏛️declarations/🌱️capture/🧪️tests/🦀️.rs:67` | no / untraced | `assert!(turn < 65535, "[DEBUG] original registration capture stalled phase={}", source.as_ref().unwrap().progress());` |

Non-test print/console hits (first 40):

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:115` | yes (hub, semio-hub-*) | `}], progress(event) { console.log('home-directory-projection-persistence ${event.stage}: ${event.law ?? ""} artifacts=${` |
| 2 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:116` | yes (hub, semio-hub-*) | `console.log('home-directory-projection-persistence-native-receipts: ${JSON.stringify(receipts)}');` |
| 3 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:118` | yes (hub, semio-hub-*) | `console.log('home-directory-projection-persistence-check: checks=${homeDirectoryProjectionPersistenceOracle(this.repoRoo` |
| 4 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:221` | yes (hub, semio-hub-*) | `}], progress(event) { console.log('home-directory-event-page-owner ${event.stage}: ${event.law ?? ""} artifacts=${event.` |
| 5 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:222` | yes (hub, semio-hub-*) | `console.log('home-directory-event-page-owner-native-receipts: ${JSON.stringify(receipts)}');` |
| 6 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:224` | yes (hub, semio-hub-*) | `console.log('home-directory-event-page-owner-check: checks=${homeDirectoryEventPageOwnerOracle(this.repoRoot)} clean');` |
| 7 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:432` | yes (hub, semio-hub-*) | `], progress(event) { console.log('home-directory-identity-rows ${event.stage}: ${event.law ?? ""} artifacts=${event.arti` |
| 8 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:433` | yes (hub, semio-hub-*) | `console.log('home-directory-identity-rows-native-receipts: ${JSON.stringify(receipts)}');` |
| 9 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:435` | yes (hub, semio-hub-*) | `console.log('home-directory-identity-rows-check: checks=${homeDirectoryIdentityRowsOracle(this.repoRoot)} clean');` |
| 10 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:520` | yes (hub, semio-hub-*) | `console.log('interactive-job-catalog: descriptor rows without an interactiveJob disposition: ${staleRows} (regenerated b` |
| 11 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:528` | yes (hub, semio-hub-*) | `console.log('plugin-identity-check: checks=${spacePluginIdentityOracle(this.repoRoot)} clean');` |
| 12 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:569` | yes (hub, semio-hub-*) | `}], progress(event) { console.log('interactive-job-catalog ${event.stage}: ${event.law ?? ""} artifacts=${event.artifact` |
| 13 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:570` | yes (hub, semio-hub-*) | `console.log('interactive-job-catalog-native-receipts: ${JSON.stringify(receipts)}');` |
| 14 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:572` | yes (hub, semio-hub-*) | `console.log('interactive-job-catalog-check: checks=${interactiveJobCatalogOracle(this.repoRoot)} clean');` |
| 15 | semio-hub-space | `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:613` | yes (hub, semio-hub-*) | `console.log('persistence-data-class-check: checks=${persistenceDataClassOracle(this.repoRoot)} clean');` |
| 16 | semio-hub-stdio | `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/🗺️stdio-text-walk-and-surfaces/🟦️.ts:175` | yes (hub, semio-hub-*) | `console.log('stdio-home-io-surface-oracle: AJV=1 TOML=bun+iarna library=${fixture.libraryPackage} component=${fixture.co` |
| 17 | semio-hub-stdio | `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/🗺️stdio-text-walk-and-surfaces/🟦️.ts:57` | yes (hub, semio-hub-*) | `console.log('stdio-dwg-artifact-ownership: cases=${fixture.cases.length} AJV=1 framework=clean modules=clean artifact-fa` |
| 18 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:124` | yes (hub, semio-hub-*) | `console.log('local-bootstrap-launch: unlimited-budget=1 opted-budget=1 negative-budget=1 staging-cases=5 session-broker-` |
| 19 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:178` | yes (hub, semio-hub-*) | `const server = await ensureHubBackend(repoRoot, hubBackendIdentity(name), { onProgress: (progress) => console.log('[INFO` |
| 20 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:180` | yes (hub, semio-hub-*) | `console.log('[INFO] dev ${name}: backend server ${server.host}:${server.port} (container ${server.identity.container}; r` |
| 21 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:256` | yes (hub, semio-hub-*) | `console.log('[INFO] secure local hub ready at http://127.0.0.1:${run.port}');` |
| 22 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:262` | yes (hub, semio-hub-*) | `console.log("[INFO] secure native WGPU child started with an fd3 credential endpoint");` |
| 23 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:269` | yes (hub, semio-hub-*) | `console.log("[INFO] secure MCP child started with an fd3 credential endpoint");` |
| 24 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:277` | yes (hub, semio-hub-*) | `console.log('[INFO] secure local admin starting at ${adminRelay.url}/admin/');` |
| 25 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:296` | yes (hub, semio-hub-*) | `console.log('[INFO] secure local OS profile starting at ${uiOrigin}');` |
| 26 | semio-hub-workspace | `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:299` | yes (hub, semio-hub-*) | `console.log('[INFO] local session broker for ${broker.record.profiles.join(",")} — every \'dev s\' row signs in through ` |
| 27 | @semio-tech/flow-core-build | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📜️script.ts:31` | no / untraced | `console.log('Flow browser package preview: ${preview.files.length} files, ${preview.browserBytes} browser bytes, ${previ` |
| 28 | @semio-tech/framework | `🧰️framework/📦️packages/🟦️typescript/📜️script.ts:360` | no / untraced | `console.log('retained UI native strip-only oracle: ${result.laws} fixture laws, ${result.grants} grants, ${result.retire` |
| 29 | @semio-tech/framework-os-mcp | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts:22` | no / untraced | `try{console.log('[test] ${requireMcpBinary(this.repoRoot)}');await runVitest(this.root,rest,"../../🧪️tests/🎚️config/🟦️.t` |
| 30 | @semio-tech/framework-renderer-react | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts:300` | no / untraced | `console.log("framework-renderer-react: region/host-contract lint passed");` |
| 31 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:188` | no / untraced | `const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:calle` |
| 32 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:189` | no / untraced | `console.log('media-slot-contract: ${receipts.reduce((count, receipt) => count + receipt.laws.length, 0)} exact native la` |
| 33 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:307` | no / untraced | `console.log('directory-retained-home-bootstrap-source-check: checks=${directoryRetainedHomeBootstrapOracle()} clean');` |
| 34 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:328` | no / untraced | `}],progress(event) { console.log('directory-retained-home-bootstrap ${event.stage}: ${event.law ?? ""} artifacts=${event` |
| 35 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:329` | no / untraced | `console.log('directory-retained-home-bootstrap-native-receipts: ${JSON.stringify(receipts)}');` |
| 36 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:330` | no / untraced | `console.log('directory-retained-home-bootstrap-native-check: sourceChecks=${checks} nativeLaws=3 clean');` |
| 37 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:352` | no / untraced | `console.log('normalized-presence-rows-source-check: checks=${normalizedPresenceRowsOracle()} clean');` |
| 38 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:366` | no / untraced | `console.log('normalized-presence-rows-native-check: sourceChecks=${checks} nativeLaws=1 clean');` |
| 39 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:442` | no / untraced | `console.log("framework-renderer-wgpu: generated 🎞️frame-worker.js");` |
| 40 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:459` | no / untraced | `console.log("framework-renderer-wgpu: 🎞️frame-worker.js is fresh");` |

## Rule 5: compat shims, adapters, deprecated items, legacy aliases (heuristic)

Keyword heuristic (`deprecated`, `legacy`, `compat`, `shim`, `adapter`, `fallback`), non-test. Mostly domain vocabulary (e.g. `compatibility` in standards mutations), so review needed.
- Current non-test hits: **1739**. Term breakdown: fallback 583, adapter 446, compatibility 328, legacy 210, ? 106, shim 50, compat 13, deprecated 3.
- Introduced since base: **316**.

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | semio-hub-workspace | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🎭️fixture-builders/🦀️.rs:14` | yes (hub, semio-hub-*) | `("🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js".to_owned(), b"export const streams = {};\n".to_vec()),` |
| 2 | semio-hub-workspace | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📚️verified-catalog/🦀️.rs:209` | yes (hub, semio-hub-*) | `/// 🎯 Resolves one exact descriptor, subject role, and optional surface preference without fallback. Without a preferenc` |
| 3 | semio-hub-workspace | `🌎️hub/🧩️compositions/➗️mathematical/🦀️.rs:22` | yes (hub, semio-hub-*) | `/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this` |
| 4 | semio-hub-workspace | `🌎️hub/🧩️compositions/🌿️vcs/🦀️.rs:22` | yes (hub, semio-hub-*) | `/// kept alongside it (a second parallel registration channel is the compatibility layer this ticket` |
| 5 | semio-hub-workspace | `🌎️hub/🧩️compositions/🎬️sequence/🦀️.rs:22` | yes (hub, semio-hub-*) | `/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this` |
| 6 | semio-hub-workspace | `🌎️hub/🧩️compositions/💠️lowpoly/🦀️.rs:25` | yes (hub, semio-hub-*) | `/// is deleted, not deprecated. '.editor::<E>(…)' registers the mutation-capable surface (the former` |
| 7 | semio-hub-workspace | `🌎️hub/🧩️compositions/📋️forms/🦀️.rs:22` | yes (hub, semio-hub-*) | `/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this` |
| 8 | semio-hub-workspace | `🌎️hub/🧩️compositions/📸️remodel/🦀️.rs:22` | yes (hub, semio-hub-*) | `/// compatibility layer this ticket forbids, so the old calls are gone rather than kept alongside.` |
| 9 | semio-hub-workspace | `🌎️hub/🧩️compositions/🔱️trinity/🦀️.rs:29` | yes (hub, semio-hub-*) | `/// registration channel is the compatibility layer this ticket forbids), same cutover '🗒️note'/` |
| 10 | semio-hub-workspace | `🌎️hub/🧩️compositions/🕸️dag/🦀️.rs:24` | yes (hub, semio-hub-*) | `/// registration — that is a forbidden compatibility layer, already rejected once on this ticket).` |
| 11 | semio-hub-workspace | `🌎️hub/🧩️compositions/🖍️draw/🦀️.rs:22` | yes (hub, semio-hub-*) | `/// channel is NOT kept alongside it (a second parallel registration channel is the compatibility` |
| 12 | semio-hub-workspace | `🌎️hub/🧩️compositions/🗒️note/🦀️.rs:23` | yes (hub, semio-hub-*) | `/// channel is the compatibility layer this ticket forbids). 'crate::artifacts::note::artifact_kind()'` |
| 13 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️.rs:388` | yes (hub, semio-hub-*) | `/// 🔎️ First selected node — the fallback target for actions that implicitly operate on "the"` |
| 14 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🏷️rename-app-instance/🦀️.rs:30` | yes (hub, semio-hub-*) | `/// 'apply' below instead); the fallback still honors 'config.active_node_id' — only the "fall back to` |
| 15 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🚮️remove-app-instance/🦀️.rs:37` | yes (hub, semio-hub-*) | `/// 'apply' below instead); 'payload.node_id' (when set) is unaffected, and the fallback still honors` |
| 16 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️.rs:9` | yes (hub, semio-hub-*) | `//! per-file 's_play_action' (returning the legacy 'ActionDescriptor') is gone — 'crate::engine::space::` |
| 17 | semio-hub-workspace | `🌎️hub/🧩️compositions/🪵️sourcing/🦀️.rs:23` | yes (hub, semio-hub-*) | `/// NOT kept alongside it (a second parallel registration channel is the compatibility layer this` |
| 18 | @semio-tech/draw-js | `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts:132` | no / untraced | `const decompositions:{code:number;values:number[];compatibility:boolean}[]=[];let first:number\|null=null;` |
| 19 | @semio-tech/draw-js | `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts:133` | no / untraced | `for(const row of lines("UnicodeData.txt")){const code=parseInt(row[0]!,16);if(row[1]!.endsWith(", First>")){first=code;c` |
| 20 | @semio-tech/draw-js | `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts:141` | no / untraced | `const compositions=decompositions.filter(row=>!row.compatibility&&row.values.length===2&&!(flags[row.code]!&4)).map(row=` |
| 21 | @semio-tech/draw-js | `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts:144` | no / untraced | `ranges.forEach((row,index)=>{const at=rangeOffset+index*24;word(at,row[0]!);word(at+4,row[1]!);for(let field=2;field<8;f` |
| 22 | @semio-tech/framework-filesystem | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:281` | no / untraced | `let volume:string\|undefined;const facts=(owned:ArtifactWindowsRelativeHandleV1,deleted=false):ArtifactWindowsRelativeFac` |
| 23 | @semio-tech/framework-filesystem | `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:64` | no / untraced | `const stat=():ArtifactReadFileIdentityV1=>{if(!live())throw Error("Exact live Windows file read capability required");ch` |
| 24 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:1` | no / untraced | `//! 🎚️ Typed media vocabulary: media types, ports, compatibility, converters and artifact kind specs.` |
| 25 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:152` | no / untraced | `/// carry a format kind id string (the legacy format enum was retired — ticket 26/08/11/` |
| 26 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:220` | no / untraced | `/// 🔂️ The single source of truth for wire compatibility: classes must match exactly, 'MediaForm::Any' on the accepting ` |
| 27 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:26` | no / untraced | `// generic obj-glb-stl codecs and the DWG bit-codec, but the legacy media-format enum itself was retired` |
| 28 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:319` | no / untraced | `/// 🎞️ The value that actually flows over a workflow wire, produced by 'ArtifactApp::export_media' and consumed by 'Arti` |
| 29 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:64` | no / untraced | `/// 🗄️ Export target format kind ids (string, the legacy format enum was retired — ticket 26/08/11/` |
| 30 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:89` | no / untraced | `/// ⌨️ Typed-media lattice: every port/wire in the workflow carries a 'MediaType' ('class' × 'form') instead of the lega` |
| 31 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:351` | no / untraced | `/// 📹️ In collecting mode records 'result''s error and goes on with 'fallback'; the fail-fast mode passes it through.` |
| 32 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:352` | no / untraced | `fn recover<T>(&self, result: Result<T, InputSchemaError>, fallback: impl FnOnce() -> T) -> Result<T, InputSchemaError> {` |
| 33 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:356` | no / untraced | `Ok(fallback())` |
| 34 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:67` | no / untraced | `/// locale for the most frequent mutation input names, the fallback when an input carries no 'x-semio-ui.label'.` |
| 35 | @semio-tech/framework-manifest | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:87` | no / untraced | `/// fallback. TypeScript twin: 'mutationInputDefs' ('🛂️manifest/🟦️.ts'); fixture '🧫️fixtures/🧫️mutation-inputs'.` |
| 36 | @semio-tech/framework-renderer-wgpu | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:136` | no / untraced | `/** 📤️ Checks exact nonempty media output and the actual original Source scalar adapter. */` |
| 37 | @semio-tech/mit-bestand-demonstrator | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts:27` | no / untraced | `/** 🖥️ Every one of the demonstrator's eight panes drives a GPU surface, and one of them only boots on a real adapter.` |
| 38 | @semio-tech/mit-bestand-demonstrator | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts:33` | no / untraced | `* default is the machine's real adapter; 'DEMONSTRATOR_E2E_GPU=swiftshader' forces the software stack` |
| 39 | @semio-tech/plugin-registry | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts:134` | no / untraced | `* discovery contract. The pre-Shape-V2 legacy sandwich shape this used to also admit was removed once` |
| 40 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs:482` | no / untraced | `/// '<fallback>/<xxh3 of the preferred path>.sock', so a deep runtime directory never keeps a daemon from starting` |

## Rule 6: new `#[allow(` and lint suppressions

Non-test `#[allow]`/`#[expect]`/`eslint-disable`/`@ts-ignore`/`biome-ignore`: current **3059**, introduced since base **565**.
- Most common forms (current): `#![allow(async_fn_in_trait)` 1348; `#[allow(unused_imports)` 789; `#![allow(clippy::result_large_err)` 324; `#[allow(clippy::too_many_arguments)` 89; `#[allow(clippy::too_many_arguments, reason = ` 77; `#![allow(unexpected_cfgs)` 72.
- Introduced rows, first 40 of 565:

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | @semio-tech/repo-product | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🧬️mutation-tree/🟦️.ts:256` | no / untraced | `const head = existing ?? '//! 🧬️ ${NEW_SCAFFOLD_MARKER}: mutation family \'${domainLabel}\' — one crate of leaf payloads` |
| 2 | semio-framework-dispatch-macros | `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/🦀️.rs:4` | no / untraced | `//! consuming crate needs '#![allow(async_fn_in_trait)]'; 'Send' stays structural (R3), never a '+ Send' bound.` |
| 3 | semio-s-workspace | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs:3` | no / untraced | `#[allow(unused_imports)]` |
| 4 | semio-s-workspace | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:28` | no / untraced | `#[allow(unused_imports)]` |
| 5 | semio-s-workspace | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:3` | no / untraced | `#[allow(unused_imports)]` |
| 6 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🧱️model/🦀️.rs:50` | no / untraced | `#[allow(clippy::too_many_arguments)]` |
| 7 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🕹️handle-kit/🦀️.rs:65` | no / untraced | `#[allow(clippy::too_many_arguments)]` |
| 8 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:631` | no / untraced | `#[allow(clippy::too_many_arguments)]` |
| 9 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🎛️live-editor/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 10 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/👁️viewer-surface/🦀️.rs:127` | no / untraced | `#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]` |
| 11 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🔌️shell-hooks/📚️library/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 12 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🖼️visual-chrome/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 13 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:19` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 14 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧰️entity-editing/📚️library/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 15 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧱️foundation/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 16 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧲️live-core/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 17 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧾️panels/🔎️inspection/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 18 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧾️panels/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 19 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🖼️canvas-lowering/🦀️.rs:604` | no / untraced | `#[allow(clippy::too_many_arguments)]` |
| 20 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧊️viewport3d/🦀️.rs:10` | no / untraced | `#![allow(async_fn_in_trait, unused_imports)]` |
| 21 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓️actions-fields-more/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 22 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓️actions-fields/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 23 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-fatigue-rows/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 24 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-sls-rail-more/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 25 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-sls-rail/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 26 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️bridge-sls/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 27 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️factor-overrides-rows/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 28 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️factor-overrides/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 29 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️analysis-rows/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 30 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️reliability-analyses/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 31 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-analysis/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 32 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-deflection/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 33 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-geometry/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 34 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-horizontal/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 35 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-identity/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 36 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-material/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 37 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-resistance/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 38 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️members-vibration/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 39 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️design-basis-structure/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |
| 40 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️placements/🦀️.rs:3` | no / untraced | `#![allow(async_fn_in_trait)]` |

## Rule 7: raised limits and constants (introduced lines)

Heuristic on constant/variable names containing LIMIT/MAX/CAP/BUDGET/TIMEOUT/CEILING/QUOTA or `max_*/limit_*/cap_*` assigned >= 3-digit numbers. Current non-test: **1109**; introduced since base: **190**. The diff does not show whether each value was raised, so the introduced rows are the review set.

| # | crate | file:line | in demonstrator closure | snippet |
|---|---|---|---|---|
| 1 | @semio-tech/plugin-window-kits | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🦀️.rs:310` | no / untraced | `pub const TREE_WINDOW_BODY_ITEM_BUDGET: usize = semio_ui_runtime::SURFACE_RECONCILE_MAX_ITEMS - 1 - TREE_WINDOW_FIXED_NO` |
| 2 | @semio-tech/ui | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/📐️panel-fit/🟦️.ts:4` | no / untraced | `export const PANEL_MAX_VIEWPORT_FRACTION = 0.45;` |
| 3 | semio-s-workspace | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:239` | no / untraced | `const GRID2D_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 1 << 20;` |
| 4 | semio-s-workspace | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📌️pin-solution/🦀️.rs:111` | no / untraced | `pub const BITMAP_MAXIMUM_OUTPUT_EDGE: usize = 512;` |
| 5 | semio-s-workspace | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📌️pin-solution/🦀️.rs:112` | no / untraced | `pub const BITMAP_MAXIMUM_OUTPUT_CELLS: usize = BITMAP_MAXIMUM_OUTPUT_EDGE * BITMAP_MAXIMUM_OUTPUT_EDGE;` |
| 6 | semio-s-workspace | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/🦀️.rs:15` | no / untraced | `pub(super) const BITMAP_MUTATION_OUTPUT_MAXIMUM_BYTES: usize = pin_solution::BITMAP_MAXIMUM_OUTPUT_CELLS * 128 + 16_384;` |
| 7 | semio-s-workspace | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕸️graph-operation-route/🦀️.rs:16` | no / untraced | `pub const FLOW_GRAPH_OPERATION_CAPACITY: semio_plugin::retained_command::ArtifactRetainedWorkCapacity =` |
| 8 | semio-s-workspace | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs:70` | no / untraced | `const MARKED_IDS_LIMIT: usize = semio_ui_contract::UI_FIXED_LIST_ITEMS;` |
| 9 | semio-s-workspace | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs:72` | no / untraced | `const MARKED_IDS_LIMIT: usize = semio_ui_contract::UI_FIXED_LIST_ITEMS;` |
| 10 | semio-s-workspace | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🦀️.rs:52` | no / untraced | `pub const MAXIMUM_VALUE_BYTES: usize = 32_770;` |
| 11 | semio-s-workspace | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧵️retained-commands/🦀️.rs:7` | no / untraced | `const MAXIMUM_BYTES: usize = 16_384;` |
| 12 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/⚖️compliance/✅️check-results/🦀️.rs:19` | no / untraced | `pub const UTILIZATION_CAP: f64 = 1.0e6;` |
| 13 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/⚖️compliance/✅️check-results/🦀️.rs:62` | no / untraced | `static NEAR_LIMIT_RESERVE_BITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(NearLimit::DEFAULT_RESE` |
| 14 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🎨️colour-maps/🦀️.rs:96` | no / untraced | `pub const DEFAULT_DOMAIN_MAX: f64 = 1.2;` |
| 15 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🌍️ground/🦀️.rs:26` | no / untraced | `const COUNT_MAX: f64 = 12.0;` |
| 16 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:14` | no / untraced | `const RAY_MAX: f64 = 3.0;` |
| 17 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/📏️dimensions/🦀️.rs:16` | no / untraced | `pub const ANGLE_MAX: f64 = PI - ANGLE_MIN;` |
| 18 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔩️joints/🦀️.rs:23` | no / untraced | `const COUNT_MAX: f64 = 12.0;` |
| 19 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:15` | no / untraced | `pub const MAX_SIZE: f64 = 60.0;` |
| 20 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🦀️.rs:20` | no / untraced | `pub const BAR_MAX: f64 = 6.0;` |
| 21 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/⌨️command-args-and-dispatch/🦀️.rs:95` | no / untraced | `pub const NORM_LIVE_MAX_WRITES: usize = 64;` |
| 22 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪄️selection-mirror/🦀️.rs:136` | no / untraced | `pub const PAINT_BUDGET: std::time::Duration = std::time::Duration::from_millis(16);` |
| 23 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪄️selection-mirror/🦀️.rs:139` | no / untraced | `pub const DRAG_BUDGET: std::time::Duration = std::time::Duration::from_millis(8);` |
| 24 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/✅️checks/📶️utilisation-bar/🦀️.rs:5` | no / untraced | `pub const UTILISATION_SCALE_MAX: f64 = 1.2;` |
| 25 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧵️lane-runtime/🦀️.rs:12` | no / untraced | `pub const MAX_LINK_LOSS_MS: u64 = 30_000;` |
| 26 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🏗️members/🦀️.rs:9` | no / untraced | `const MAX_EXAGGERATION: f64 = 200.0;` |
| 27 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs:13` | no / untraced | `const BETA_MAX: f64 = 7.0;` |
| 28 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs:97` | no / untraced | `const HEIGHT_AT_MAXIMUM: f64 = 2.0;` |
| 29 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs:98` | no / untraced | `const HEIGHT_LIMIT: f64 = 5.0;` |
| 30 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🏗️piles/🦀️.rs:10` | no / untraced | `const CAP_THICKNESS: f64 = 0.8;` |
| 31 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🏗️piles/🦀️.rs:11` | no / untraced | `const CAP_BELOW_GROUND: f64 = 0.2;` |
| 32 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🛡️safety/🦀️.rs:7` | no / untraced | `const CEILING: f64 = 10.0;` |
| 33 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧩️entities/🦀️.rs:12` | no / untraced | `const GROUP_CAP: f64 = 0.8;` |
| 34 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧱️walls/🦀️.rs:13` | no / untraced | `const MAX_REINFORCEMENT_LAYERS: usize = 40;` |
| 35 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🌋️lateral-and-seismic-checks/🦀️.rs:9` | no / untraced | `const LATERAL_LIMIT_ID: &str = "ndp.en1997.7.7.lateral-deflection-limit";` |
| 36 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🌊️seepage/🦀️.rs:6` | no / untraced | `pub const MAX_CELLS: usize = 160;` |
| 37 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🌊️seepage/🦀️.rs:8` | no / untraced | `const MAX_SWEEPS: usize = 20_000;` |
| 38 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🔬️investigation/🦀️.rs:87` | no / untraced | `pub const SPACING_MAX_IDS: [&str; 2] = ["ndp.en1997.2.4.2.spacing.gc12.max", "ndp.en1997.2.4.2.spacing.gc3.max"];` |
| 39 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:301` | no / untraced | `cap_vertical: 1_800_000.0,` |
| 40 | semio-s-workspace | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:302` | no / untraced | `cap_moment_x: 100_000.0,` |

## Rule 8: UI strings with a default language or missing German

- `defaultLocale` / `defaultLanguage` / `fallbackLng` / `DEFAULT_LOCALE` / `default_locale` in source (ts/tsx/rs/mjs): **0** hits in non-test code.
- Rust `const EN:` localization tables without a matching `const DE:`: **0** files. Every `const EN:` table is paired with a `const DE:`.
- Hard-coded `locale: 'en'` / `"en"` in tests and in tooling typings is test data or types, not UI defaults. Not a violation.
- Not checked: English literal strings in `.tsx` JSX text. Needs a separate sweep; this audit did not sample them.

## Rule 9: paths >= 256 chars

- Max repo-relative path: 184 chars. Max absolute path: 212 chars (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧪️tests/🧫️fixtures/👁️brush-preview/✅️ok/🔣️.mutation.json` is the byte-longest, 304 UTF-8 bytes).
- Absolute paths of >= 256 chars: **0** (char count). Absolute paths of >= 256 UTF-8 bytes: **4226** (emoji path segments are 4 bytes each). The rule says "chars", so the byte count is an upper-bound signal, not a violation.

## Rule 10: migration scripts in the source tree

- No migration scripts found in the changed source tree. The 45 name hits are mutation names like `connect-kind-compatibility` and `add-compatibility-rule` (domain terms), not migrations. Listed in the scan at `/private/tmp/claude-501/-Users-ueli-Documents-semio/52893ade-82ad-4a46-aa3d-0c3ed23edafb/scratchpad/audit/scan.json` (key `migration`).

## Rule 11: source files copied into ticket folders

Content-hash match (sha256) of .rs/.ts/.tsx/.mjs files under `.🧬semio/🦑️repo/🎫️tickets` against the source tree. **10** copies, all version controlled (`git ls-files` matches). Violation of "do not copy source files to tickets folder which are checked into version control".

| # | ticket-folder copy | source original | git |
|---|---|---|---|
| 1 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/actinfra/patches/emitter-request-bytes.test.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🧪️tests/📏️request/🟦️.ts` | tracked by git |
| 2 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/actinfra/patches/artifact-refusal-reason.test.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🧪️tests/🧾️refusal/🟦️.ts` | tracked by git |
| 3 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/actinfra/patches/retention-size-law.test.ts` | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🪶️size-law/🟦️.ts` | tracked by git |
| 4 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/actinfra/patches/worker-demand.test.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/🧪️tests/🎯️demand/🟦️.ts` | tracked by git |
| 5 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/actinfra/patches/strip-debug/module.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️strip-debug/🟦️.ts` | tracked by git |
| 6 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1102-tools/actinfra/patches/strip-debug/test.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️strip-debug/🧪️tests/🟦️.ts` | tracked by git |
| 7 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1101-tools/ts2d/patches/booleans.test.ts` | `🧰️framework/🔨️modules/◻️2d/🔀️booleans/🧪️tests/🟦️.ts` | tracked by git |
| 8 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1101-tools/ts2d/patches/paths.ts` | `🧰️framework/🔨️modules/◻️2d/🔀️booleans/🛤️paths/🟦️.ts` | tracked by git |
| 9 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` | tracked by git |
| 10 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | tracked by git |

Folder C copies: 1101-tools and 1102-tools under C (8 files). Ticket 26/10/18 GET-CODEBASE-RUNNING holds 2 more.
