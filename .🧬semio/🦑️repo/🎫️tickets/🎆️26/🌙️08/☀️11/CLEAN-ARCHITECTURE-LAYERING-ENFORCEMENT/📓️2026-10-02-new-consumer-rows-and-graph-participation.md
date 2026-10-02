# New Consumer Rows and Graph Participation

Read-only source inspection; no compiler/tests. Source paths below relative to Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Five rows

The last five rows in reachability fixture `🧫️fixtures/📡️mutation-reachability/🔣️.json` and type-origin fixture `🧫️fixtures/🧬️mutation-type-origin/🔣️.json` are equivalent authored scenarios. Dormant `cfg_attr(any(),rewrite)` leaves default valid Rust at root, leaf, out-of-line child and inline child. ChildPayload is public and reexported through public payload; aggregate tuple references the reexport. The fifth known-and-unproven-mount uses a disabled cfg(any()) first declaration and enabled second declaration of the same symbol/path, so default Rust avoids duplicate definitions. It tests authored alternative cardinality, not unsupported metadata—the disabled declaration is conditional but not unresolved. All nativeAccepted/compileAccepted true expectations are source-semantically coherent; no runtime receipt is claimed.

Current type-origin test `🧪️tests/🔬️workspace-contract/🟦️.ts:7663–7666` already adds unproven-root-body to module/aggregate/variant conditional lists and known-and-unproven-mount to first module conditional list. Correct: root inner cfg_attr is inherited by aggregate and variant, while first dormant mount condition does not make the later aggregate conditional. Leaf/child conditional metadata does not affect root aggregate golden. No remaining list mismatch identified.

Provider `🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts:328` root proof and :344 leaf proof reject first two; :347–352 counts all child alternatives then selects exact inline/out-of-line body scope, rejecting both child rows. Inline selection [child.name] is necessary: selecting [] alone would falsely accept unproven-inline-child-body. Root namedMounts cardinality :334 rejects fifth before filtering conditional rows. Source review predicts all five intended refusal branches, not a passing claim.

## Graph wire ownership and minimal participation

`🔍️discovery/🟦️.ts` owns RustModuleGraph runtime API; existing source-direction schema owns scopeFact/scopeProof definitions and closed graphAuthority fixture expectations (`🧬️schema/🧱️rust-source-direction/🔣️.json:27`). No complete serialized graph API schema was identified by the inspected graph-name search. Participation must be added under this existing domain schema rather than a normalization-private ad hoc scanner.

Minimal required graph observation array: `{sourcePath,crateRoot,manifestPath,modulePath,sourceScope,origin}` with origin root/module/include and closed declaration/source locator provenance. Record observed candidate participation before admission and retain denied reason separately: unresolved scope/key, invalid manifest, missing source bytes or conflicting key. Arrays can preserve alternative origins; canonical key remains crateRoot/modulePath, not source filename. Include manifest-root observations even when strict parsing or source root proof fails, otherwise denied root can masquerade as orphan. Candidates for known explicit mounts must retain physical source locators before pruning; do not invent absent target bytes.

Evidence consumer `🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts:134` currently omits strictManifests. It should consume the canonical strict graph and classify admitted/denied/unmounted using this observation contract. Orphan literal declarations retain physical observations only when no authored graph participation is known; denied origins never regain authority through :179–192 fallback. Distinct valid manifest roots remain independent.

## Prior placement correction

The extraction report has been corrected: normalization/mutation/direct-owner-index is production, not nested tests. Focused canonical test placement belongs in the existing library 🧪️tests collection, following mutation-case-pair; no nonexistent normalization/mutation/tests path should be introduced from that prior mistaken citation.
