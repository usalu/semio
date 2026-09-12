# Package Bodies Need Independent Ownership Checks

Anonymous source filenames and target-first placement are necessary but do not establish that a language package contains only packaging code. The initial inventory's external-name classification for Next routes requires a content audit.

The current coordinator TypeScript package contains substantial implementation in `app/api/v1/ticket/route.ts`: request parsing, ticket construction, lifecycle dispatch, repository calls, and event publication. Its `app/page.tsx` also contains the dashboard implementation. These are domain behaviors, even though Next imposes the route/page leaf convention.

The appropriate taxonomy execution packet is to put the existing implementations in their semantic owner directories as anonymous TypeScript leaves, with the externally required Next files containing thin, declared export glue. The same inspection must cover all 13 coordinator app files and other admitted package descendants. Preserve the current behavior and verify the existing endpoint/UI contracts after extraction.

The package-purity gate must inspect the bodies of admitted wrappers. A broad permission for a package `app` directory, or an exact filename exception for `route.ts`, must not confer permission to place arbitrary domain implementation there. Add a language-neutral fixture contrasting a thin route export with an implementation-bearing route body, and validate the classification independently.

This finding is based on direct source inspection during the first execution wave. It remains an outstanding lane after the current target moves; no fix or runtime pass is claimed here.

## Existing Gate Drops Content Findings

Direct inspection confirms that `collectPackageRoles` in discovery recursively classifies package source and emits `package-implementation` or `package-role-unresolved` for substantive or unclassified source bodies. However, the root script's `policyPackageLanguagePurityBreaches` filters that result to only `packaging-violation` and `unknown-lang`. Thus the registered package-purity gate does not presently enforce the body diagnostics already computed by discovery. The new implementation-basename report intentionally has a narrower contract and cannot close this gap by itself.

The future execution lane must first validate these diagnostic categories with language-neutral thin-entry and body-bearing fixtures and a parser-backed independent oracle. It must then include justified content findings in the gate and extract domain implementations from package entrypoints into anonymous owner leaves, keeping exact required package glue. This extends beyond the coordinator Next example: the repository library TypeScript package entry itself contains substantial authored implementation. Implementations behind language-first package directories require separate ownership review even where their basename is already anonymous.

The Terra enforcement audit has been asked to independently confirm this gate coverage observation. No source mutation or test pass is claimed by this planning note.

## Classifier Precision Before Enforcing Bodies

The independent census reported 173 implementation and 161 unresolved diagnostics, not 334 confirmed implementation moves. The existing classifier uses keyword and regular-expression heuristics: TypeScript type-only exports, required thin Rust function/derive registration wrappers, configuration objects, and strings can require more exact treatment. The execution lane must add positive and negative grammar fixtures and validate them against existing third-party parsers/native compilers before enabling these categories in the gate. Real domain bodies move out of package roots; thin package registrations and compiler glue remain exact, minimal, and declared. Do not move a correct glue file solely because the old heuristic calls it implementation.

## Two Divergent Body Classifiers

There are currently two implementations: discovery exports `classifyPackageSourceRole`, while normalization exports `classifyPackageGlueContent` through its separate `classifyGlue`. They do not enforce the same contract. Normalization classifies any fixed/configurable filename as configuration without inspecting its body, and its Rust/Go/registration shortcuts use statement counts or presence of a registration call. Consequently, fixing only the root package-purity filter leaves inconsistent planning behavior and possible false admissions. The later body-policy lane must define one shared schema-backed contract, test both public surfaces against it, and use proper lexical/parser evidence to distinguish real delegation from computation. Small function bodies are not necessarily glue, and a registration call does not make the rest of a file packaging code.
