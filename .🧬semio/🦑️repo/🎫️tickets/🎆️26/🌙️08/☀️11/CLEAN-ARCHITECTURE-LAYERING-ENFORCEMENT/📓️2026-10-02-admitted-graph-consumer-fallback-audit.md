# Admitted Graph Consumer Fallback Audit

Read-only inspection; no tests/compiler/jobs. Paths below relative to Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Remaining authority restoration paths

**Inventory direct fallback:** `🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts:171–187` calls the graph resolver, then for bare `sealed::Mutation` selects the first !unresolved authored path module and directly returns its existing physical file. It does not require admitted context or target. Thus known-before-unresolved same-key denial can be undone by the known declaration. Relative specifiers also return physical candidates (:173–176); these are path-reference facts and should not be conflated with Rust module ownership. Require an admitted source scope and exact canonical target proof for Rust route fallback, or remove that fallback as an authority source.

**Ancestor target shortening:** same file :162–166 searches successively shorter module paths. A denied `crate::sealed::nested::Type` can fall back to a surviving ancestor target. Missing terminal item versus denied module key are different facts. Stop on any denied canonical prefix before shortening; expose an owned refusal/ambiguous key set or a shared graph route selector rather than reconstruct denial from missing target. Pruning contexts alone does not prevent this route when the importer root context remains valid.

**Authoring reuse:** `🏗️authoring/🧬️mutation-tree/🟦️.ts:194–202` checks one top-level named mount and exact public/path shape but does not reject mount.unresolved or unknown root scope. Count-based rejection already handles two known/unresolved same-name rows once names are canonical. A single unknown attribute mount or unknown inner root can still be reused. Require the shared scope proof and unresolved refusal before admitting an existing mount; retain insertion behavior only when absence itself is proven in an authoritative root scope.

**Structural root and type origin:** `🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts:327–335` requires exactly one named mount and filters mount.unresolved, but reads direct source facts rather than admitted root scope. :339–351 collects public leaf declarations, then direct child modules for reexport origins. Single unknown inner root/leaf/inline scope can therefore restore mounted/wrapped/origin independently of graph denial. Shared root/inline scope selector must govern all three declaration locations; canonical same-name alternatives must be counted before filtering unknown rows.

## Consumers already gated by context

`🔍️discovery/🟦️.ts:12541–12544` metadata provider proof requires exactly one admitted consumer manifest/sourceScope context; no nearest-manifest fallback there. `🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts:45–49` projects only supplied context.manifestPath and refuses absent manifest. Import binding :206–207 refuses empty contexts/unmounted import scopes; its rootFacts fallback :222 operates only inside supplied admitted context. These entrypoints must continue receiving current graph contexts, never caller-nearest manifest inventions.

`🧹️normalization/🟦️.ts:4520–4526` finite manifest proof requires nonempty graph contexts, exactly one manifest and captured content hash. Its chain validation :4563–4578 searches direct module facts but also requires matching admitted parent contexts and exact graph.targets key. Consequently pruning denied context prefixes closes this path; retain the graph target check. Scope selector integration removes duplicate source trust scanning without granting missing authority.

## Exact key semantics

Prune by (crateRoot, canonical module chain), including descendants; preserve legitimate distinct crate roots mounting the same physical source. Root's intended contract: same-module-key-dual-origin becomes mounts []; include-and-module-dual-origin with distinct nonconflicting keys remains [include,module]. Keep physical authored references in inventory independently of admitted context.

Smallest additional consumer rows: importer root `use sealed::Mutation` with known+unresolved canonical sealed declaration (both orders); importer `use crate::sealed::nested::Mutation` with poisoned sealed parent and surviving crate context; single known mount under dormant unknown inner root attr; leaf public origin under dormant unknown inner leaf attr. Default Rust can accept the dormant branches while all-authored authority refuses. Extend the existing mutation-inventory consumer, mutation-scaffolding, mutation-reachability and mutation-type-origin owners rather than add parallel policy scanners.
