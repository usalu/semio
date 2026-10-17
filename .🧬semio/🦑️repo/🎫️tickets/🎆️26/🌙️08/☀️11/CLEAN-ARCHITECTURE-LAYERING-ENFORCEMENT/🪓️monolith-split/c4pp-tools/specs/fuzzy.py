from specs.lib import *

CH = [
    ch('⚠', 'error-and-helpers', 'error_and_helpers', [(24, 78)],
       'Fuzzy error and helpers: the fuzzy error type with its display and error impls, the result alias and the clamp, linspace and argmax helpers.', glob=False),
    ch('📈', 'membership-functions', 'membership_functions', [(82, 305)],
       'Membership functions: the membership function enum and its evaluation, shaping and conversion methods.'),
    ch('🧮', 'fuzzy-sets', 'fuzzy_sets', [(309, 393)],
       'Fuzzy sets: the type-1 fuzzy set, the interval type-2 set and the intuitionistic set.'),
    ch('➗', 'norms-conorms-and-hedges', 'norms_conorms_and_hedges', [(397, 502)],
       'T-norms, t-conorms and hedges: the triangular norm and conorm families and the complement, concentration and dilation hedges.'),
    ch('🔢', 'fuzzy-arithmetic-relations-and-possibility', 'fuzzy_arithmetic_relations_and_possibility', [(506, 683)],
       'Fuzzy arithmetic, relations and possibility: fuzzy numbers, fuzzy relations and the possibility measure.'),
    ch('🌐', 'universe-variables-and-rules', 'universe_variables_and_rules', [(687, 808)],
       'Universe, linguistic variables and rules: the discretised universe, linguistic variables, antecedent clauses, consequents, rules and rule bases.'),
    ch('🎯', 'defuzzification-and-inference', 'defuzzification_and_inference', [(812, 1082)],
       'Defuzzification and inference: the defuzzifiers, the inference models, rule traces, explanations, the multiple input multiple output system and the hierarchical system.'),
    ch('🧠', 'adaptive-membership-and-anfis', 'adaptive_membership_and_anfis', [(1086, 1275)],
       'Adaptive membership and ANFIS: trainable membership functions and the adaptive neuro-fuzzy inference system.'),
    ch('📚', 'rule-learning-and-optimizers', 'rule_learning_and_optimizers', [(1279, 1502)],
       'Rule learning and optimizers: Wang-Mendel rule generation, pruning and weighting, subtractive clustering, the genetic and particle swarm optimizers and the evolving fuzzy system.'),
    ch('🧩', 'clustering-and-decision-making', 'clustering_and_decision_making', [(1506, 1734)],
       'Clustering and multicriteria decision making: fuzzy c-means, Gustafson-Kessel clustering and the fuzzy AHP, TOPSIS and VIKOR methods.'),
    ch('🛰', 'temporal-spatial-hybrid-and-fan', 'temporal_spatial_hybrid_and_fan', [(1738, 1850)],
       'Temporal, spatial, hybrid and fan: the temporal and spatial evaluators, the probabilistic and possibilistic conversions, the hybrid fusion and the fan controller.'),
]

SPEC = dict(
    orig='fuzzy.rs',
    file=FUZZY + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[FUZZY + '/🧪️tests/🔬️unit/🦀️.rs'],
)
