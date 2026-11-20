# Retained certified solid patterns

The registered Linear, Circular and Grid Pattern operators now use the original retained geometry job. They accept a live constructor-certified primitive and publish the registered `compound` output. The first compound member is the exact authored source. The remaining members are private native solid drafts with independent stable topology labels. Linear and grid spacing scale the authored vectors, grid members are ordered with Y varying first, and circular positions rotate evenly around the supplied axis through the world origin.

The synchronous BRep pattern method uses a Boolean union fold. A compound represents the same geometric union only when every member is separated. The retained route proves separation by comparing finite analytic world-axis bounds with both the kernel contact tolerance and a floating-point guard. It checks one candidate or one pair per granted event. Bounds that overlap or touch cause an `UnsupportedOwner` refusal before drafting. A rotated box also requires an image that still fits the certified primitive family. Counts are whole values from zero through 4,096, with zero and one forming a single-member compound.

The original live root and shape mutation revision are checked before preparation and publication. No child receives a live handle. A single final native index insertion transfers custody of every root to the claimed compound. Closing an unpublished plan retires each partial or complete child in reverse dependency order. Closing a published plan releases only the transaction journals. Standard compound aliases follow the existing provenance rules, so new child aliases can represent native solids without restoring constructor-specific certificates for analytic-only queries. The direct synchronous BRep union API keeps its behavior.

The pairwise proof uses constant scratch state and quadratic work in the number of requested copies. The transaction retains one fixed-size draft per copy and a compact root list. All arena operations, checks, allocations and rollbacks use the caller's retained grant and can yield between turns.

## Verification

- The language-neutral fixture checks translated boxes with raw direction magnitude, rotated spheres with a non-unit axis, row-major grids, and overlapping copies.
- The independent Three.js test passed 27 assertions across fixture placement and separation.
- A standalone Rust test compiled the current solid-pattern and solid-transaction sources against the repository's real native primitive, arena and retained-effect crates. It verified every fixture center and member order, preservation of the original root, removal of all unpublished topology on rollback, committed topology survival after journal retirement, and rejection of overlap before draft allocation.
- The registered Flow test covers all three operators, limits each turn to one work item, checks that the live handle count increases by one per completed pattern, and checks cancellation and overlap against native arena counts. The parent ticket coordinates the full registered Cargo run.
- Repository source closure measured 3,985 active lines for the BRep engine, 3,967 for the native registry, 734 for the solid transaction, 2,332 for the Flow original jobs, and 1,703 for the Session core. All fit the 4,000-line source admission ceiling.

## Reproduce the standalone native check

Pass the current repository Cargo unit store's `prototype/build` directory to `🐍️solid-pattern-native-check.py` through `repo tools exec --light python`. The check reads native unit receipts and supplies both split metadata and code artifacts for each direct crate. Scoped scratch outputs are deleted automatically.
