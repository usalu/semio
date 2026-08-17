# UI Popover Adapter Family Audit

## Classification

`Popover/🟦️component.tsx` is a coherent Radix popover adapter family. It owns root, trigger, content, and anchor wrappers; menu-level direction/surface presentation stays with the content adapter. It is cycle-free after the Flow relocation.

## Consumers

- ActionDropdown, Toggle dropdown, and OS SyncAttachCard independently use Popover and PopoverContent.
- ActionDropdown and Toggle use PopoverTrigger.
- SyncAttachCard uses PopoverAnchor; the framework Search semantic component also uses it from the authored barrel behavior.
- Stories and barrel registration are excluded as terminal evidence.

The wrappers therefore remain together as one adapter family. No zero-consumer public symbol exists and no one-consumer facet warrants inlining away from the external-library boundary.

## Boundary and Cycle Evidence

- Every inferred wrapper signature leaks React types parameterized by Radix primitives. A later type-isolation lease should replace those with repository-owned adapter contracts.
- The runtime graph is one-way: React barrel -> Popover -> Radix/direct Flow/Surface/presentation modules.
- Direct `flow-direction-context` import avoids the former barrel path.
- No product change is needed if names/signatures remain stable.

## Disposition

Retain the complete adapter family. No source edit is justified by consumer disposition. Queue a bounded repository-owned prop-contract cleanup separately.

## Baseline SHA-256

- Popover: `0ca2c5f79fe9ed8a8efa6d73a699c20326224f8fcab79d2466fcede332cba8be`
- story: `a744f9872e535f6493db0f8bbce1104b7cb8218077881c4fe9820222bb32da7c`
- React barrel: `537138eb89f28302991e6b38f2aea879f7ee19cacbd495d5e23517a7755b4e5d`
- ActionGroup: `b798641562d12be9eddb6e8cbdf25f321747f0c14616a649f924aa456e090eed`
- Toggle: `5bd32b0c107de82c8a663b50bd860d7f87c2e24c7013ce94364ad93f924c3fdb`
- OS ShellSync: `229ff874b0aa51be9f48e8883817a48212dd7fc3cb6e1f64741b38b882a91cb4`
