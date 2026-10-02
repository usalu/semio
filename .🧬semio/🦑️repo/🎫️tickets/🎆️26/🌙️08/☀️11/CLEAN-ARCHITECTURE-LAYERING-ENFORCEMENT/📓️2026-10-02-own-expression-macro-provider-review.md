# Own Expression Macro Provider Review

Read-only current helper; no compiler/test execution. Current closed roster40 binding/17 activation equals schema40/17. Added canonical-root-local, canonical-parent-local, canonical-expression-macro and canonical-shadowed-expression-macro retain earlier36 binding rows.

Ownership helper `ownedExpressionMacro:177–185` restricts current allowances to canonical owned context and assert_eq/json names; refuses macro_rules definitions across captured sourceChain; requires visible named import constraints; individually checks each reserved argument token against localDirect/direct. Opaque/unknown macro branch186 otherwise remains refused. These bounds are useful but exact provider proof remains incomplete.

Concrete false-provider route: a visible `use other::*;` may import a foreign assert_eq macro. Current imports filter only `(alias ?? last path segment) === problem.path`, so the glob contributes no match and assert_eq is accepted as standard-prelude. Native-valid foreign macro can alter or emit code. Require refusal for competing visible globs unless their macro export identity is actually proven; do not exempt macro spelling globally. Similarly no_implicit_prelude disables the assumed standard route and needs explicit proof/refusal rather than assumed presence.

For json, presence of a dev dependency key serde_json is not exact provider identity: it could carry package override/local path or be shadowed by a local serde_json module providing another exported macro. Current helper does not inspect that root shadowing or prove selected package/library authority. Preserve explicit unknown refusal until provider identity is established through captured manifest facts and lexical import binding. A third-party oracle name alone is not authority.

Argument checking must continue to retain every family occurrence, including paths inside transparent expression arguments, and generic/local shadowing. The helper's final direct check currently does not call shadowed; outside bare/qualified checks must demonstrably cover each accepted argument or the helper must require the same occurrence proof. This is a proof obligation, not an independently executed counterexample.

High received the precise glob/prelude/json provider findings. Add closed native-valid foreign-glob and shadowed-json negatives before claiming complete own-expression macro closure; retain original assertions/native rows and no generic macro-name exemption.
