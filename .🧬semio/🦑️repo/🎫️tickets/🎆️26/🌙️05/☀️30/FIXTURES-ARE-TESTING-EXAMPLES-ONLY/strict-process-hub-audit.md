# Fresh Process, Hub and Stdio Audit

Finite source audit after current UI moves. No source edits or Cargo/test execution. Hashes in 🗑️generated/strict-process-hub-inputs.json.

## Process Whole Trial Schemas

`🧰️framework/🔨️modules/🏃️process/📥️capture/📥️invocation/🧬️schema/🔣️.json` remains on disk. It constrains an entire three-case pre-cancel/live-cancel/publication trial corpus with fixed test ceilings, expected exit, stdout, acquired/published. This is independent whole-trial schema, not actual Process input/output API. Adjacent invocation tests already import plain corpus and actual capture/📋️policy/🧬️schema at lines6-7; they compile only policySchema at15 and54 and owned validator compares actual policy inputs. No need to replace removed corpus schema.

`🧭️routing/📥️invocation/📥️loading/🧬️schema/🔣️.json` remains and requires id,cancelAtLoad,trace,rejected; this is a test trial row shape. No TypeScript file exists in the bounded loading subtree, so its actual direct test validator reader is absent. Treat as orphan trial schema candidate. Genuine routing invocation schema requires version,owner,maximumElapsedMilliseconds; nested invocation/process schema requires version,policy,deadlineEpochMilliseconds,capabilities. Preserve these domain contracts. Preserve capture policy budgetMs/maxOutputBytes contract.

## Hub and Stdio Resource Findings

No genuine production fixture-reading edge was proven in the bounded Rust/TypeScript inspection. Do not report zero runtime fixture edges for the whole repo: this was a finite text/control-context audit, not compiler closure verification.

- Hub fixture file loads found in package scripts are verification/test paths, not source runtime evidence by themselves. `🌎️hub/📦️packages/🦀️rust/🦀️.rs:48` mounts test_artifact_root for cfg(any(test,feature=integration-fixtures)); Cargo feature :37 enables native-artifact-execution. This feature can compile testing support into an integration target, but inspected source is an artifact-output helper, not fixture asset consumption. Selected production features must be observed before alleging release leakage.
- TIFF io/text/mutations/🦀️.rs:63,78,93,108,116 load mutation fixture payloads, but the helpers are individually #[cfg(test)] and reexports #[cfg(test)]. Genuine domain mutation codec modules precede these and should be preserved.
- Stdio PDF sqlite snapshot source has many include_bytes!(ICC fixture) and include_str!(test vectors), all beneath #[cfg(test)] mod tests at116-117. The ICC includes are test resource bytes, not proven production color-profile assets.
- GeoJSON schema/conformance/🦀️.rs number-lexeme include_str!:72 is inside #[cfg(test)] mod tests at66-67; its production cancellable borrowed conformance implementation remains domain code.
- Stdio glTF schema/inferences/<domain>/🧪️contract/🟦️.ts imports fixtures from sibling collections. These are outside canonical tests ancestry and remain test-placement candidates; the 🧪️contract spelling alone does not make them runtime modules. Parent package routing/import graph remains unresolved in this bounded pass.

Concrete fixture collections still under production schema scope: PDF snapshot/value/octets/fixtures encoding vectors and GeoJSON schema/conformance/fixtures number-lexemes; move under local schema-owner tests/fixtures and update cfg(test) readers, preserving actual production schema/conformance behavior.
