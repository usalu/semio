# P8yz-g Flow Retained VCS Second Fresh Adversarial Audit

Date: 2026-08-26  
Auditor: Codex (independent read-only source/static audit)  
Verdict: **RED — RED-1 is remediated, and the retained source route remains GREEN; RED-2 remains incomplete because the language-neutral hostile ledgers are not executable exact I/O ledgers.**

## Scope And Boundaries

Read in full:

- `AGENTS.md`;
- the prior Terra audit `📓️terra-p8yz-g-flow-retained-vcs-remediated-adversarial-reaudit-2026-08-26.md`;
- the current implementation report `📓️codex-p8yz-g-flow-retained-vcs-source-static-implementation-2026-08-26.md`;
- the complete retained route in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` (lines 959–2757); and
- all three checked-in retained VCS JSON fixtures.

No source, Cargo, Nx, Bun/Nx test runner, Wasm, browser, or cache was modified or run. The sole Bun invocation was a fixture-only JSON census, permitted for this audit. This is source/static evidence, not a Rust compilation or runtime-pass claim.

## RED-1 — GREEN: The 13-Feature Oracle Now Compares Live Retained Results

The former self-authored semantic-label loop is absent. The test-only `SerdeJsonFlowOracle` is behind the owned `FlowSemanticOracle` trait and independently parses the fixture using `serde_json`:

- `evaluate_operations` takes the multilingual initial document, applies all thirteen fixture inputs in its own `serde_json::Value` state, retains its own undo/redo/version state, and computes its own revision/generation/count/digest/page/history/handback case (`component.rs:3020–3127`).
- `expected_operations` separately decodes every fixture document/page/history/handback ledger (`3110–3127`), and the test asserts independent evaluation equals that ledger before inspecting the subject (`3480–3483`).
- The subject consumes each fixture input through the corresponding real `begin_*` route, progresses to a real page, takes it, ACKs it, incrementally closes it, and then extracts the actual case from the live `FlowRetainedVcs` session (`3331–3401`, `3431–3439`, `3491–3500`).
- `flow_oracle_actual_case` reads the retained document canonically, all ten page fields, undo/redo, every credit, active/leased/retired ownership, revision/generation/digest, document-version state, edit owner, retention, and closing (`3298–3329`).

The final assertion compares the complete thirteen-element live vector to the independently evaluated oracle vector. No literal per-feature semantic result is pushed by the subject. This satisfies the requested RED-1 semantic-document/page/history/version/digest/handback comparison at the source/static boundary.

## RED-2 — RED: Hostile Data Is Listed, But It Is Not A Deterministic Executable I/O Ledger

The fixtures now improve materially: all thirteen regular feature cases have input plus document/page/history/handback references, cardinalities are `[0, 1, 3, 16, 17, 256, 257]`, all five grants are present, 24 boundaries name both controls, and multilingual values exist. The fixture-only census produced:

~~~json
{"operations":13,"allOperationLedgers":true,"documents":13,"cardinality":[0,1,3,16,17,256,257],"grantVectors":5,"transferBoundaries":24,"allTransferControls":true,"multilingual":true,"byteVectors":[],"fingerprintFields":["credits","activeOperations","leasedPages","retiredActionOwners","retiredSurfaceOwners","editOwner","documentRetained","closing","semanticState"],"perTransferExactFingerprint":false,"oracleExpectedHandbackReferences":true}
~~~

Three acceptance defects remain.

1. **No byte-cap I/O vectors exist.** `📒️lifecycle.json` declares the limit `bytes: 65536`, but neither fixture has a byte-named vector or a deterministic accepted/max/max+1 byte input and expected result/fingerprint. The executable Rust test constructs `"x".repeat(FLOW_VCS_MAX_BYTES + 1)` directly (`3443`), so it cannot validate fixture-owned byte/multibyte cap data. A limit declaration is not a cap ledger.
2. **Every transfer result refers only to the string `"exactPreOperation"`, not an exact per-boundary result.** The 24 records in `lifecycle.json:44–67` do not contain a resolved document/authority/handle/grant/fingerprint. The shared template itself omits undo owners, redo owners, revision, parent revision, document generation, digest, versions, and active version; it substitutes the untestable label `"semanticState": "byteExactPreOperation"` (`70`). Thus it does not specify the exact fingerprint requested for every transfer boundary.
3. **Fixture data is only shape-checked, not driven through the hostile protocol.** The fixture law checks authority and malformed arrays only by length (`3525–3526`, `3555–3556`), grant vectors only by length (`3521`, `3558`), and transfer controls only by count (`3522–3524`, `3550–3553`). It never converts their inputs/grants/expected result/fingerprint into a `FlowRetainedVcs` call. The subsequent cancellation tests construct their own phase targets and grants instead. Removing or changing a hostile vector's `result`, `fingerprint`, authority fields, malformed input fields, or grant values can therefore still pass the fixture-law predicate. In particular, `expected_operations` reads `handback.documentVersions` and `activeDocumentVersion`, then hard-codes all other terminal values in `flow_oracle_expected_handback` (`3120–3123`, `3276–3296`); the referenced JSON terminal fingerprint values are only existence-checked (`3557`).

Consequently the required stale/wrong/ABA, malformed/omitted, deadline/zero-fuel/interrupted, and every cancel/fault transfer boundary outcome is not fixture-defined and execution-verified with exact fingerprints. The production tests may cover portions of this behavior, but they do not turn the language-neutral artifacts into the required deterministic I/O ledgers.

## Retained Source Route — GREEN (Static)

- Exact rollback is explicit and one owner at a time: visibility, surface, history, each retired redo owner, semantic mutation, edit ownership, and loaded undo/redo are reversed in `close_operation_step` (`1751–1810`). `cancel` and `fault` enter rollback whenever any corresponding ownership/mutation state was acquired (`1676–1717`).
- The full grant envelope is enforced on cancel, fault/panic-fault, operation close, and retained close; preflight/can-charge bounds items and bytes (`1676–1722`, `1740–1747`, `1896–1965`).
- Publication remains split as history transfer → surface transfer → visibility → page (`2080–2158`).
- The retained route has no scan/clone/whole-serialization/whole-apply forbidden spelling and no forbidden browser/host token. The exact static route census below found all ten required bounded-route tokens and zero of the 23 forbidden spellings.

~~~text
route_start=959 route_end=2757 route_lines=1799 forbidden_present=0 required_present=10
~~~

## Exact Read-Only Commands And Results

~~~sh
sed -n '1,260p' AGENTS.md
sed -n '1,260p' '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📓️terra-p8yz-g-flow-retained-vcs-remediated-adversarial-reaudit-2026-08-26.md'
sed -n '1,300p' '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📓️codex-p8yz-g-flow-retained-vcs-source-static-implementation-2026-08-26.md'
sed -n '780,1365p' '🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/🔮️oracle.json'
sed -n '3100,3410p' '🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs'
sed -n '3410,3968p' '🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs'
nl -ba '🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs' | sed -n '1668,1820p;1890,1990p;2060,2170p;2680,2760p'
~~~

Result: source/fixture evidence above; no mutation.

~~~sh
bun -e 'const fs=require("fs"); const p="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/"; const o=JSON.parse(fs.readFileSync(p+"🔮️oracle.json","utf8")); const l=JSON.parse(fs.readFileSync(p+"📒️lifecycle.json","utf8")); const x=o.operations; const has=(v,k)=>Object.prototype.hasOwnProperty.call(v,k); const result={operations:x.length,allOperationLedgers:x.every(v=>has(v,"input")&&has(v,"expected")&&["document","page","history","handback"].every(k=>has(v.expected,k))),documents:Object.keys(o.documents).length,cardinality:l.cardinalityVectors.map(v=>v.value),grantVectors:l.grantVectors.length,transferBoundaries:l.transferControlLedger.length,allTransferControls:l.transferControlLedger.every(v=>v.controls?.join(",")==="cancel,fault"),multilingual:["Grüße 🌊️","节点-ä","改訂 текст","替代","শেষ"].every(v=>fs.readFileSync(p+"🔮️oracle.json","utf8").includes(v)),byteVectors:[...l.cardinalityVectors,...o.capacityVectors].filter(v=>/byte/i.test(v.name)),fingerprintFields:Object.keys(l.exactHandbackFingerprints.exactPreOperation),perTransferExactFingerprint:l.transferControlLedger.every(v=>typeof v.expectedHandback==="object"),oracleExpectedHandbackReferences:x.every(v=>typeof v.expected.handback.fingerprint==="string")}; console.log(JSON.stringify(result));'
~~~

Result: the JSON object in RED-2; notably `byteVectors: []` and `perTransferExactFingerprint: false`.

~~~sh
awk '$0=="//#region 🌊️RetainedVcs"{inside=1;start=NR} inside{block=block $0 "\\n";lines++} $0=="//#endregion 🌊️RetainedVcs"{end=NR;inside=0} END{split("apply_action|flow_vcs_apply_action|flow_vcs_fixture_digest|flow_vcs_dictionary_census|flow_vcs_tree_census|flow_vcs_flow_ui_census|fn publish_cursor|.widgets.insert(|.widgets.remove(|.synapses.insert(|.synapses.remove(|mem::replace|.iter()|.position(|.find(|.filter(|.fold(|.sum(|.clone()|from_fn|for |while |serde_json",f,"|");forbidden=0;for(i in f)if(index(block,f[i]))forbidden++;split("flow_vcs_fixture_scalar_digest|flow_vcs_fixture_census|transfer_history_cursor|transfer_surface_cursor|publish_visibility_cursor|publish_page_cursor|history_transferred|surface_transferred|visibility_published|redo_retired > 0",r,"|");required=0;for(i in r)if(index(block,r[i]))required++;print "route_start="start" route_end="end" route_lines="lines" forbidden_present="forbidden" required_present="required}' '🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs'
~~~

Result: `route_start=959 route_end=2757 route_lines=1799 forbidden_present=0 required_present=10`.

No Cargo/Nx/Bun test runner/Wasm/browser/cache command was run.

## Required Remediation

1. Add fixture-owned byte-cap accepted/max/max+1 inputs and exact retained-source/result/fingerprint outputs, including multibyte byte lengths.
2. Replace transfer-boundary labels with fully resolved, per-boundary protocol inputs and exact expected document/page/history/complete fingerprint values (or exact fixture references that resolve to complete values), including both cancel and fault.
3. Make the fixture law execute every authority, malformed/omitted, grant, and transfer vector through live `FlowRetainedVcs`; compare each specified result and every fingerprint field. Parse terminal/handback fixture values instead of hard-coding `[0; 7]` and the remaining constants in the test helper.
4. Add hostile in-memory fixture mutations for every required vector field/value, not merely deletion/count checks, so a changed result/fingerprint/grant/boundary cannot silently pass.

Until these changes are made, the packet is **RED** despite the GREEN retained route and resolved RED-1 oracle path.
