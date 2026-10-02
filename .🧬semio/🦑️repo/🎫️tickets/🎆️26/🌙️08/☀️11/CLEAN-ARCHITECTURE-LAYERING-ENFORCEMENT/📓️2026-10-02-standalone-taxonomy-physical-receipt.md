# Standalone Taxonomy Physical Receipt

Read-only source/log inspection; no jobs or fixture mutations performed.

## Actual physical boundary

Canonical taxonomy loader `normalization/🔣️taxonomy/🟦️.ts:1251–1263` first calls input/assertLexicalInputOutsideOpaque, then semanticOwnedInputFileSnapshot before the cache lookup. Therefore a cached content hash cannot itself restore a deleted or linked leaf.

Discovery `🟦️.ts:5934–5990` snapshot rejects linked repoRoot itself, linked/non-directory intermediate children, and linked/nonregular leaf. It captures with descriptor O_NOFOLLOW when available, verifies descriptor identity/size/mode/timestamps before/after reading, verifies named leaf and root/child ancestor dev/ino after capture. Missing children return null, then loader throws absent schema. These are substantial actual fresh physical receipts.

Concrete remaining standalone gap: snapshot lstat begins at repoRoot, not its parents. Input/assertNoFollowAncestors12–24 also begins below resolved repoRoot and never checks root ancestry. Thus a physically linked ancestor of otherwise ordinary repoRoot can be traversed and accepted. Admission PrepareOptions separately checks the entire root chain, but standalone options-aware load does not inherit that guarantee. Proposed cache split does not fix it automatically.

Input/assertLexicalInputOutsideOpaque26–35 resolves raw path/root before checking. `missing/../taxonomy.json` or `link/../taxonomy.json` can become a regular admitted leaf without witnessing the required raw prefix. The snapshot sees only the normalized relative coordinate. If standalone loader's owned contract is strict raw physical traversal, raw lexical validation must occur before resolve, with actual root ancestor proof. Do not normalize away missing/linked prefixes then label the result no-follow proof.

## Authored closed fixture rows and independent oracle

Each row uses a ticket-owned synthetic directory and copied valid taxonomy bytes. The live repository/root is never renamed or linked.

* valid-then-delete-leaf: initial load succeeds, unlink schema, second load refuses despite warm parsed facts; independent node:lstatSync leaf yields ENOENT.
* valid-then-link-leaf: replace synthetic schema with link to identical bytes, second load refuses; lstat isSymbolicLink true.
* valid-then-link-inner-parent: move synthetic schema parent to sibling and replace parent with directory link, warm reload refuses; independent lstat raw parent identifies link.
* valid-then-link-root: initial synthetic root admitted, rename it and replace it with link to identical tree, reload refuses; snapshot already rejects this case.
* valid-then-link-root-ancestor: root resides beneath synthetic ancestor, rename ancestor and replace it with directory link; reload must refuse. Current standalone code predicts false grant; this is the smallest real gap law.
* missing-prefix-parent: same schema available at normalized target, authored taxonomyPath missing/../taxonomy.json; owned raw path contract must refuse. Independent raw-component lstat fails at missing prefix even though normalized path exists.
* linked-prefix-parent: equivalent linked prefix followed by ..; reject under no-follow raw contract rather than accepting normalized target.
* same-bytes-new-physical-root: two ordinary synthetic roots holding identical schema bytes yield fresh distinct input/path receipts and independent session facts; content parse cache may reuse only immutable facts.

Language-neutral rows declare action, root coordinate, taxonomy coordinate, expected admission/refusal, and expected physical observation kind. Native filesystem oracle uses lstat on authored components plus descriptor/read digest for ordinary files; use system symlink/junction construction behind owned test interface on Windows, never skip required authority semantics silently. Existing schema validation supplies independently valid starting bytes. No compiler required for these IO laws.

## Latest retained typecheck receipt

`🗑️generated/goal-root/types-after-source-owner-cut.log` is RED, Nx duration26.6s. Exact current diagnostics: workspace-contract duplicate loadTaxonomy imports22/32, options-aware call3061 resolved to zero-argument discovery loader, and consequent GeneratorInputTaxonomy errors3064/3072. This is an import name collision, not a missing canonical export: use distinct local symbol for actual canonical loader while keeping the separate discovery vocabulary loader.

Other actual errors: rust participation58 and source-direction437 optional expected string; canonical-json36/37 optional parseTree node and60 optional canonical expected; mutation evidence273 string passed to never includes union. No missing export diagnostic was present. Root/High own corrections; audit does not claim latest checkout still matches every stale logged location.

## Cache split constraint

Exclude path/input/pathMatcher from immutable parse-facts cache; fresh physical snapshot precedes every hit. structuredClone facts is feasible for RegExp slugRegex, arrays and plain records; matcher contains function closure and cannot be cloned, so excluding it is necessary. Fresh matcher should be attached to each cloned session with authored pattern behavior preserved. Caller mutation isolation and invalid→valid recovery require closed laws before revising original exact parse-count assertions.
