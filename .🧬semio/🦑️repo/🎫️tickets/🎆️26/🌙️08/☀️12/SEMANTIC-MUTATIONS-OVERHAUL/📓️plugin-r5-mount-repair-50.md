# Plugin R5 Mount Repair

## Observed Compiler Boundary

Runtime R5 stopped before tests with one error: the contributed-wire fixture mount was inside inline `plugin_runtime`, so Rust resolved the relative path through that virtual directory. The actual fixture at the Plugin root exists. Root read the retained compiler log at lines 1279–1286. This was a mount error, not evidence loss.

Runtime explicitly released all 698 captured source inputs. No Cargo process was launched by the mutation lane for this repair.

## Exact Source Changes

- Moved only the contributed-wire module declaration to crate root beside the existing publication, mutation and test-app fixture declarations; made it crate-visible.
- Changed the nested wire-client tests to import `crate::contributed_mutation_wire`.
- Added the missing `super::super::TestSnapshot` import inside the TestMutation child tests. This was independently source-reviewed; R5 had not reached that diagnostic.
- The declaration raw-wire fixture now correctly rejects negative zero for an i32 payload. No production codec coercion was added.

No fixture was copied, recreated or restored. Interaction, command ingress, lifecycle, paging and output code were untouched.

## Executed Source Checks

Contributed-wire: new placement checks first failed exactly three assertions (69/72) in `🧪️plugin-contributed-wire-43/🧫️run-TTriTV`; after repair all 72 passed in `run-8DKPXx`, with stable captured inputs.

TestMutation: the explicit child-scope import check first failed (40/41) in `🧪️test-mutation-direct-leaves/🧫️run-pn7fl2`; after the import all 41 passed in `run-ofujwo`. This older controller remains a limited source check; the broader schema, aggregate-purity and capture-hardening work in the TestDocument review is still pending.

Declaration: the corrected reference first failed 567/570 against the wrong negative-zero expectation, then passed 570/570 after correcting both parity vectors. Native execution remains pending. Its controller-only capture hardening is separate from the frozen domain inputs.

## Released Source Anchors

| Input | SHA256 |
| --- | --- |
| Plugin main | 12bc97e01166b3c50fccdd5221264174c14aaaa8a7aae36d11587f3cf4a9345d |
| TestMutation aggregate | c958acfeed0de940b9c14fe988132c3c4950b92b232627b1728c2cc3e0d48449 |
| Declaration domain cases | 9402b49f7396787e62168b293492dee128de1269ad8b15c3f05701ab1f1a7134 |
| Contributed-wire controller | dd7357d5a27f573cd4878aada9c76fa7fadd5c3e361dad0df926867f3c9c01c2 |
| TestMutation controller | 7425eadc2e0d2ed1e0fb3248354ed3385d8ab3250ed9172bd91a00f39d50782c |

These are observed anchors, not a full dependency closure. All mutation-owned Plugin compiled sources are now released for runtime's fresh native inventory and will remain unchanged during that capture. No Plugin native, Flow, GIS, guest or publication readiness is claimed.

## Remaining Direct-Ownership Work

Children still uses include-based mounting around private fixture types, and TestMutation still has behavioral tests in its aggregate. Those are explicit remaining transparency violations, not accepted final forms. Full monorepo acceptance remains open.

Repository MCP ticket tools and `repo://goals` remain unavailable in this session's tool/resource catalog; the existing open ticket is preserved.
