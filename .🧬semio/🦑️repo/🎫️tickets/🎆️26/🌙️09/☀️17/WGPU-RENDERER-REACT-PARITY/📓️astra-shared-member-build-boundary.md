# Shared Member Message Build Boundary

Native123 did not execute any tests. The shared composed-document implementation introduced `BackboneMessage::Member`, while the per-document Store sync decoder and native actor relay remained non-exhaustive. The failed gate ended after 41.6 seconds. The analogous wasm relay also required an explicit match.

The integration now rejects a member-tagged message at the canonical per-document mutation ingress and returns a typed backbone error from the native and wasm actor relay. It does not flatten child envelopes into the parent document's persistence log, and it does not report the member batch as successfully relayed. The existing exact document transport accepts only root canonical mutations. A complete composed member transport needs to retain its slot and child identity through persistence and delivery; that implementation is under read-only audit and remains unfinished.

Native124 is the first fresh compilation/test attempt after this boundary repair. Its output is retained under generated/astra-runtime/renderer-native124-retained-owner-red. No runtime pass is claimed from the compile repair itself.

Native124 reached renderer compilation but stopped on the analogous native Shell egress match after 4m9s. That per-document boundary now returns the same explicit member-lane routing error. Native125 reruns the renderer selection. The existing Store mailbox test additionally asserts that a refused member packet is returned byte-for-byte and leaves ingress empty.

## Executed Boundary Regression

The kernel regression `document_backbone_mailbox_and_retention_are_exact_bounded_and_terminal` passed1/1 with1175 filtered tests (0.035s; Nx1m10s). Receipt: `🗑️generated/astra-runtime/kernel-native-member-boundary/run.log`. This verifies the exact refused Member bytes and empty per-document ingress; it does not establish complete composed-member transport.
