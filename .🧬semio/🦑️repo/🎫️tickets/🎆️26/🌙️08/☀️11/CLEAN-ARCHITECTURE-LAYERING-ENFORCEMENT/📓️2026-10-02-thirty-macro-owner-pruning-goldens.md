# Thirty Macro Owner Rows and Context Pruning Goldens

Read-only authored-source review. No compiler, tests, Nx jobs or generation executed; route 48762 remains parent-reported pending, not a passing receipt.

Owner paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Five new rows

`🧫️fixtures/🧱️rust-source-direction/🔣️.json` has 30 macroOwners; matching `🧬️schema/🧱️rust-source-direction/🔣️.json` requires exactly 30 and its closed ID enum includes all five additions.

| Row | Source validity reasoning | Golden |
|---|---|---|
| known-before-unresolved-module-key | `cfg(all())` enables ordinary sealed; `cfg(any())` removes raw sealed alternative before duplicate declaration/rewrite resolution | leaf module scope [], mounts [] |
| unresolved-before-known-module-key | Same semantics in reverse order; tests order independence | leaf module scope [], mounts [] |
| unresolved-same-file-local-scope | Dormant alternative references same existing leaf; enabled declaration provides run | local body [13,164), mounts [] |
| unresolved-parent-descendant-local-scope | Enabled parent.rs has explicit nested.rs path; both are sibling files, nested run is pub and parent delegates | nested body [13,164), mounts [] |
| local-function-item-rewriting-producer | Dormant cfg_attr leaves a legal nested function extra; run still executes load | local body [13,206), root mount; unsupported-expression |

All strings are ASCII, so byte and UTF16 offsets coincide. Local start 13 is the opening `{`; end 164/206 is immediately after closing `}`, before newline. Each non-producer row contains exactly one literal and one dynamic include site and one actual load invocation. Input.txt is physically present at the include definition file directory. Alternative.rs contains no macro/input/unknown attribute; parent.rs adds only a known path module. No independently visible extra source scanner problem follows from these authored bytes. This is source reasoning, not native confirmation.

Dormant nested rewriting producer must refuse the entire source template proof rather than invent expansion facts; its unsupported-expression expectation intentionally skips the two-reference/scope golden assertions (`🧪️tests/🧱️rust-source-direction/🟦️.ts:359–367`). The test's catch converts scanner failure to empty refs, then the owned source executor must emit exactly the one expected problem (:369). All 30 native booleans currently true make the unconditional runtime binary launch :349 coherent.

## Required admitted-context golden review

The exact context-kind assertion is `🧪️tests/🧱️rust-source-direction/🟦️.ts:357`; inline contextProof is :358. Existing row **raw-module-identity-ambiguous-target** still expects mounts `[module]`. If both canonical sealed alternatives poison this key and contexts becomes admitted authority, change that golden to `[]`; its references and unresolved-template-scope remain retained. The five new rows already expect pruned [] at known/unresolved keys and descendants.

Do not mechanically erase all mounts on a template refusal. macro-use-parent, public-file-module, alias-can-expose-module-template and root-module-template have genuine unambiguous physical mount contexts; their stricter template seal refuses for separate reasons. `same-module-key-dual-origin` and `include-and-module-dual-origin` currently expect `[include,module]`: decide under the explicit mount-conflict contract whether the whole canonical key is inadmissible or both physical origins remain independently admitted; if conflicting origins poison that key, both goldens must become [] as well. Retain physical authored references independently of contexts in either representation.

Other exact context consumers whose existing goldens need targeted regression checking:

- `🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:197–201`, fixture `🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json:1051`: moduleGraphCases expects manifest membership. The row named ambiguous (:1054) means two separately legitimate manifest roots mount the same physical source; it is **not** same-key target ambiguity. Preserve both manifests rather than prune globally by physical filename.
- Same test :178 and :193 prove root contexts and two manifest origins for finite candidate normalization. Preserve per-manifest identity.
- `🧪️tests/🔬️workspace-contract/🟦️.ts:1069–1078` verifies root, inline os_dsl, descendant component and actual mutation type-origin contexts. Valid descendants remain; only descendants of poisoned keys disappear.
- Same workspace test :4831–4836 verifies source and test-module manifest membership. No blanket expectation update is justified without an ambiguous/unresolved ancestor.

Keep context pruning keyed by manifest/crate plus canonical module chain and source origin, with descendant invalidation. Physical source identity alone would incorrectly destroy valid alternate crate roots. Authored physical references and unresolved declarations must remain inventory facts even when context proof disappears.
