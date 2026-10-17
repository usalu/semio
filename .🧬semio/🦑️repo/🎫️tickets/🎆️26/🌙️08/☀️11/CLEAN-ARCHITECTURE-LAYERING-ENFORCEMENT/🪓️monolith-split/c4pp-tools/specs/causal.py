from specs.lib import *

CH = [
    ch('⚠', 'errors-and-dag-model', 'errors_and_dag_model', [(19, 239)],
       'Errors and the DAG model: the shared causal error type with its conversions, the pair normaliser and the causal DAG with its graph queries.'),
    ch('🧬', 'cpdag-and-meek-rules', 'cpdag_and_meek_rules', [(243, 479)],
       'CPDAG model: the completed partially directed graph, its orientation queries and the Meek orientation rules.'),
    ch('🧱', 'd-separation-and-ci-tests', 'd_separation_and_ci_tests', [(483, 601)],
       'D-separation and conditional independence tests: the d-separation query, the implied independencies, the CI test trait and the Fisher z and G-squared tests.'),
    ch('🧮', 'discovery-helpers-and-pc-stable', 'discovery_helpers_and_pc_stable', [(605, 741)],
       'Discovery helpers and PC-stable: the standardisation, covariance and combination helpers, the PC options and result, PC-stable and the v-structure orientation.'),
    ch('📈', 'score-based-and-lingam-discovery', 'score_based_and_lingam_discovery', [(743, 928)],
       'Score based and LiNGAM discovery: the local and DAG BIC scores, DirectLiNGAM and greedy equivalence search.'),
    ch('🪟', 'identification', 'identification', [(932, 1037)],
       'Identification: the backdoor and frontdoor criteria, the minimal backdoor sets and the ID-algorithm identification result.'),
    ch('📐', 'linear-gaussian-scm', 'linear_gaussian_scm', [(1041, 1227), (1708, 1726)],
       'Linear-Gaussian structural causal model: fitting, sampling, interventional and counterfactual queries and the what-if evaluation.'),
    ch('🎲', 'discrete-scm', 'discrete_scm', [(1231, 1496)],
       'Discrete structural causal model: conditional probability tables, factor arithmetic, exact inference and interventional queries.'),
    ch('📊', 'effect-estimation', 'effect_estimation', [(1500, 1678)],
       'Effect estimation: the effect estimate and bootstrap options, the naive difference, the g-formula and inverse probability weighting estimators.'),
    ch('❓', 'what-if-query', 'what_if_query', [(1682, 1706)],
       'What-if query: the what-if request record and its builder methods.', glob=False),
]

SPEC = dict(
    orig='causal.rs',
    file=CAUSAL + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[CAUSAL + '/🧪️tests/🔬️unit/🦀️.rs'],
)
