# Curriculum Audit: Architecture Optimization Videos

Read-only audit on 2026-10-27. Only this report was authored. Root AGENTS.md applies; no descendant AGENTS.md was found in architecture.

## Repository Evidence

- `🎓️teaching/🏛️architecture/README.md`: optimization is one of ten quizzes; two classification tasks and 25 examples.
- `🎓️teaching/🏛️architecture/🎯️optimization/❓️quiz/🔣️.json`: objective-versus-constraint classification, fixed scalar weighting, pairwise Pareto dominance, trade-offs and equal values. No sampling, DOE or solver teaching content exists here yet.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎯️optimization/🔣️.json`: language-neutral expected classifications and Pareto numeric baseline (100,50).
- `🎓️teaching/🏛️architecture/📚️course/🔣️.json`: bilingual English/German content; optimization topic is currently locked (null unlock). Do not change release policy as part of authoring.
- `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json`: existing manifests use video id, chapter scenes, segment scripts and output name. BIM now has six standalone concept entries, a useful precedent.
- Course README describes keyboard and screen-reader support, textual relations and semantic tables. Carry this principle into narration, transcripts and visual alternatives.

## Six Distinct Learning Outcomes

| Clip | Precise coverage | Suggested architectural demonstration | Boundary and correctness check |
|---|---|---|---|
| Minimierung / Maximierung | Design variables, domain, objective f(x), feasible constraints; improve a quantity in its stated direction; local versus global optimum | Window ratio influences energy demand; height limit filters designs; maximize usable area | Show min f equivalent to max -f, with same feasible domain. More is not always better. A constraint does not add an objective. Distinguish optimal variable x* from objective value f(x*). |
| Sampling | Choose representative evaluations before searching: full factorial, uniform random, stratified/LHS; budget and coverage | Two parameters: window ratio and shading depth, plotted as design-space points | Full factorial with k factors and m levels uses m^k runs (mixed levels: product). LHS with n samples has exactly one point in each marginal stratum, not one per 2D cell. Sampling alone does not certify an optimum. |
| Design of Experiments | Question, factors, levels, response, planned runs, main effects, interactions, confirmation | 2×2 glazing/shading study with energy response | Distinguish DOE purpose from the previous clip: learn effects, not merely fill a space. A shading effect can depend on glazing. Two-level designs cannot determine general curvature. Fractional factorials may alias effects. Deterministic simulations do not gain independent noise information by identical repeats. |
| Multicriteria / Index | Separate objective vector; normalize dimensions and preferred direction; explicit weighted scalar index | Cost versus carbon, or useful daylight versus glare | Do not add euros and tonnes directly. Use fixed stated reference bounds, nonnegative weights summing to one, and consistent cost/benefit direction. A weighted scalar is one objective, matching the quiz. Weights are value judgments; include sensitivity. Avoid zero normalization denominator. |
| Pareto | Dominance, nondominated alternatives, Pareto set in design space versus front in objective space | Reuse quiz baseline cost 100 thousand euros, carbon 50 tonnes, both minimized | Dominance means no worse in all and strictly better in at least one. Equal values do not dominate. (80,70) and (120,30) trade off; (120,70) is dominated by baseline; (100,50) is equal. Pairwise nondominance does not establish global Pareto optimality. A sampled front approximates the true front. Pareto is no universal best compromise. |
| Algorithms / Genetic Solvers | Evaluation loop, starting sample, selection, crossover, mutation, feasibility, stopping budget; single versus multiobjective ranking | Genes encode window ratio and shade depth; offspring generate complete designs evaluated by energy/cost model | Show that an optimizer calls a model rather than invents physical truth. Selection must follow objective direction. Genetic algorithms are stochastic and do not guarantee the global optimum. Include seeds, repeated runs, runtime/evaluation budget, and a withheld confirmation. Multiobjective solvers retain diversity, not a single scalar winner. |

## Sequence and Overlap

Use one persistent parametric building and consistent variable notation across all six. Order the conceptual progression as formulation → evaluation coverage → experimental learning → preference aggregation → dominance → adaptive search. Sampling and DOE deliberately share point sets but answer different questions. Multicriteria introduces a transparent scalar preference; Pareto preserves criteria; algorithms implement either formulation. Do not recast the quiz as a sampling or solver assessment without a separately authorized curriculum change.

## Accessibility and Localization

Provide equally complete English and German narration, visible labels, captions and transcripts; never rely on an implicit language default. Narrate axes, units, objective directions, point comparisons and transformations. Color must have redundant shape/label meaning. Use stable candidate labels and explain every equation in words. Avoid fast camera motion or flashing; reserve readable dwell time after the last graph update. German decimal commas and English decimal points must agree numerically. Use explicit units (thousand euros, tonnes CO₂, energy per year); illustrative performance values must be labeled as teaching examples rather than measured building data.

## Validation Targets

Create language-neutral numeric fixtures for full-factorial counts, LHS marginal occupancy, normalization/weighted scores, dominance including ties, and solver best-so-far feasibility. Validate equivalent outputs with existing third-party scientific tools behind the repo interface. Verify each language has matching concept coverage, numeric examples, captions and scene references. Mathematical visuals must agree with narrated directions at every beat.

## Primary References Checked

- [NIST DOE chapter](https://www.itl.nist.gov/div898/handbook/pri/pri.htm): experimental aims, design selection and confirmation workflow.
- [NIST DOE modeling](https://itl.nist.gov/div898/handbook/pri/section4/pri43.htm): factorial interactions, fractional alias structure and response-surface curvature.
- [SciPy LatinHypercube documentation](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.qmc.LatinHypercube.html): exact marginal-stratum definition.
- [pymoo getting started](https://www.pymoo.org/getting_started/index.html): distinguishes solution-set optimization, decision making and convergence analysis.
