# Simulation Accessibility

Removed literal ARIA syntax from editor and viewer labels. The editor status node now carries actual AccessibilitySpec.live=Polite and Activity::Loading for a nonterminal matching run. The browser Interpreter maps these fields to real aria-live and aria-busy attributes; status wording stays localized. The static viewer instruction carries plain prose.

Native window tests now serialize the actual BuiltNode while retiring its original tree, so activity is observable (the previous convenience projection omitted activity). They assert actual liveness and busy activity for all framework run states and both locales, with no literal fake ARIA text. The complete editor dependency graph is still compiling; these tests have not run yet.
