# Contract Gate Snapshot

Observed during active migrations, not a final conformance result. Bun/Nx contract target ran uncached and exited 1 after 12m 9s. Full result contained 12172 findings.

- ('testing/layout', 'invalid-test-module-wiring'): 4059
- ('testing/fixture', 'missing-fixture'): 3385
- ('testing/layout', 'inline-test-body'): 1816
- ('testing/contract', 'contribution-manifest-invalid'): 710
- ('testing/fixture', 'mutation-without-fixture'): 444
- ('testing/layout', 'test-implementation-depth'): 337
- ('testing/taxonomy', 'case-slug'): 248
- ('testing/contract', 'runtime-inventory-missing'): 169
- ('testing/dependency', 'oracle-in-production'): 145
- ('testing/contract', 'unregistered-mutation-vocabulary'): 144
- ('testing/contract', 'stub-serializer'): 136
- ('testing/contract', 'unknown-mutation-catalog'): 134
- ('testing/contract', 'binary-protocol-drift'): 97
- ('testing/layout', 'test-case-name'): 94
- ('testing/layout', 'inline-self-test-declaration'): 55
- ('testing/layout', 'legacy-test-filename'): 50
- ('testing/oracle', 'missing-external-oracle'): 42
- ('testing/contract', 'unknown-comparison'): 22
- ('testing/fixture', 'fixture-tolerance-profile-unknown'): 17
- ('testing/layout', 'legacy-test-directory'): 17
- ('testing/oracle', 'unknown-oracle'): 13
- ('testing/fixture', 'fixture-comparison-profile-unknown'): 10
- ('testing/layout', 'test-implementation-filename'): 7
- ('testing/layout', 'test-owner-delivery-scope'): 6
- ('testing/oracle', 'unknown-no-oracle-decision'): 5
- ('testing/taxonomy', 'unknown-case-child'): 4
- ('testing/fixture', 'fixture-generator-unregistered'): 4
- ('testing/contract', 'feature-syntax'): 1
- ('testing/contract', 'mutation-kind-undeclared'): 1

The older contract validator rejects every emoji-prefixed case slug, contradicting the new path rule requiring one canonical leading emoji. This requires a policy alignment fix, not renaming valid canonical cases. The layout scan also traversed temp/merge scratch copies and downloaded vendor JavaScript; authored-source scope and test-call classification require review.
