# Optimization Clip Implementation

## Shared Visual Interface

The optimization domain owns `🎯️optimization/🎬️clip/🎨️visuals/🐍️.py`. `ClipScene` isolates the existing Manim runtime and exposes explicit language selection, localized labels and numbers, narration-linked animation, a reusable pavilion drawing and geometric axes. The existing teaching font, palette, subtitle, timing and audience interfaces remain authoritative. English and German are both explicit; no content language is inferred from the host.

`step` binds one literal narration row to one visible caption and timing window. `explain` sequences short semantic clauses named `key`, `key_2`, and so on while holding the diagram from the initial animation. This preserves readable two-line captions without shrinking typography. The decision animation implementation resides in its own adjacent mixin, avoiding concurrent edits to unrelated scene implementations. Cache provenance includes both visual implementations, the complete semantic model and schema, language-neutral cases, typography and palette inputs.

## First Three Independent Videos

- **Extrema:** A changing pavilion introduces the design variable and response. An animated quartic distinguishes a local valley at minus one from the lower global valley at two. The optimum design and objective value are separate. Reflecting the curve shows the equivalence of minimizing a response and maximizing its negative. A dashed feasibility boundary excludes the old global solution and changes the feasible optimum.
- **Sampling:** A persistent two-parameter pavilion becomes a two-dimensional design space. Equal-budget full grid, recorded random draws and Latin hypercube point sets transform into one another. Nine marginal strata, one highlighted vertical strip and one horizontal strip explain the precise Latin hypercube rule. Narration separates coverage from evaluated performance and global optimality.
- **Experiments:** Four actual pavilion parameter combinations become a factorial response diagram. Separate circle and square markers distinguish shading levels. Nonparallel lines reveal interaction; the main and interaction effects match the schema-bound example. Competing straight and curved responses share the measured end points. A confirmation point at the intermediate setting agrees with the semantic teaching model and distinguishes the alternatives. Narration explains deterministic repeats, physical replication and fractional-design confounding.

Every canonical source defines a fresh native scene class with complete English/German literal narration. Each topic has its own public wrapper delegating composition, explicit language, progress, cancellation and audience settings to the shared runtime. Illustrative numbers are labeled as teaching models, not measured building performance.

## Verification

The language-neutral mathematics and independent third-party checks are documented by the semantic-model agent. `smoke-first-three.py` is the retained input script for real Manim construction across both languages and both audience modes, with generated PNG keyframes and media confined to the ticket's generated folder. All three English scenes completed real Manim construction in both speaker and spectator modes. The captured grid, Latin hypercube, factorial plan, interaction and confirmation diagrams passed visual inspection.

Initial capture exposed one long caption, so all narration was split into short authored clauses. It also revealed a marker being faded in and indicated simultaneously; the competing animations were removed. The coordinator's final encoded-movie inspection moved the global minimum label into the upper annotation row, clear of the curve, and the refreshed extrema movie passed its final image audit.

The coordinator reports all nineteen targeted acceptance tests and the independent Ajv schema validation passing, including bilingual caption checks. The refreshed German speaker extrema movie passed final image verification through the canonical shared pipeline. No sources were changed after the coordinator's source freeze; the final handoff updates this report only. The smoke script remains a retained verification input, while generated images, logs and movies remain subject to the ticket's generated-output cleanup policy.
