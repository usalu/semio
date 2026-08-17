# Interaction Multilanguage Consumer Audit

## Owner State

The authored interaction TypeScript, Rust, schema leaves, framework glue, manifest TypeScript, generated manifest, and direct TypeScript consumers are clean. Their current fingerprints are recorded below.

| Surface | SHA-256 |
|---|---|
| Interaction TypeScript | `5999548126ce1e52748e39cc96405987cfe5c1ad9e006d8c7215dbd152daf1b7` |
| Interaction Rust | `fc84e01a8053d3632fa179500f3ff2fa21f30f3588ae93b4beea3e351afd9350` |
| Schema TypeScript | `8117fd509505d667bfd94917110dd510b85162dc1af79e4a59c7e6ead4dc8694` |
| Schema Rust | `298289e1ba5e4bd6cde48f975a77ab5a18b0319aad36209fa54cfde01f4e19e6` |
| Schema JSON | `e1db5428a37f36688e98199a4bb623603ee942ba73d1ab6def1d8bd8cfb8fcb9` |
| Schema GraphQL | `6014a1caa54995a5452ed797d1e1c79a96c74ad055d21b564579b9a664e42a3a` |
| Framework TypeScript glue | `45f9e589322aaf7001ef89750d3fc9a89c04a7bdeb4b5b647a83eabc0ac2b743` |
| Framework Rust glue | `b74df535f4fa71a095462373fb3857140625d0403444dbf45cfcfc8cf1f0b8fe` |
| Manifest TypeScript | `ad8aabcc4a90e1f5c41e247a80659b9eb349c50398bab30f580b127cf645ceb3` |
| Generated manifest TypeScript | `0e45108c611d6d334f8be2b29a44993ce5ae6ba0a3cfa118e9c46c10a7aeda18` |
| React package index | `c3b144495c317d83c5a9911e0fca0568732ac33bacef87ccbad2920be15eed22` |
| ShellScope | `2244253e260f825710bd9a87dde0001acb889e9d3f0dbfbc3db4134dbeeb8734` |
| Tree | `b97d40a3e35e871339026750af7ba4aa7cc4e6dbc3e86fd3c4db0a43cbc99edc` |

## Responsibility Disposition

- TypeScript `nextSelection` and its private topology/set helpers have exactly one production terminal, Tree. TypeScript `validateState` likewise has exactly one production terminal, Tree. They qualify for a future atomic inline into Tree, not retention as reusable runtime behavior.
- TypeScript `nextHover`, `HoverInput`, and `DEFAULT_HOVER_SPEC` have zero production consumers. `domainTopologyAncestors` also has zero production consumers.
- TypeScript `MergeMode` has at least two independent production terminals, the React package and ShellScope, so the shared contract must remain at their framework LCA.
- Rust definition/runtime contracts are independently consumed by manifest construction, plugin dispatch, and multiple plugin applications. They remain framework-owned contracts.
- Rust `next_selection`, `next_hover`, and `validate_state` each have one direct production consumer, plugin dispatch. Rust `DomainTopology::ancestors` has zero external production consumers.
- JSON/GraphQL are canonical schema leaves. Absence of in-repository imports is not evidence that their authored contract may be deleted.
- Generated manifest and glue are mirrors/assembly, not production consumers.

## Lease Decision

Do not issue a language-only edit. The plan requires language/schema facets for one semantic component to move together, while the current graph gives different terminal consumers to the handwritten TypeScript runtime and Rust protocol runtime. The clean TypeScript Tree inline and the dirty Rust plugin-dispatch inline must be designed as one graph-coloured cross-language responsibility lease after the OS plugin/protocol owner is released. Until then, keep the clean interaction owner unchanged. The zero-consumer ancestor helpers and TypeScript-only hover surface remain recorded candidates within that atomic lease.
