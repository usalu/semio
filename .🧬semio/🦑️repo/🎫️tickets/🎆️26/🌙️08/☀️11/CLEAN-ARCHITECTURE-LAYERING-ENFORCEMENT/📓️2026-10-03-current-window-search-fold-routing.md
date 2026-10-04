# Window Search Fold Routing

A printable keystroke targeted Actions after Search had become an independently folded pane. The documented handler promised to unfold Search, but changed the wrong fold state. One production line now changes Search's fold state; all event admission guards and dispatch behavior remain unchanged.

The new closed language-neutral schema/corpus has eight native keyboard scenarios across both explicit English/German locales: Latin, umlaut, Space, Control, Meta, Alt, composition and named keys. Ajv accepts the exact corpus and refuses foreign ambient locale metadata and a missing case. Testing Library dispatches real KeyboardEvents against the actual shell-scoped Window and independently observes mounted Search and Actions DOM. Admitted printable keys expose only Search; the six refused classes keep both panes folded.

Genuine tests-only RED was9 originals pass/1 new failure, en:latin Search remained absent (session82031). After the one-line production correction, the complete original9 plus additive1 are actual GREEN10/10 (session1163), with DEBUG witnessing sixteen explicit-locale event cases. Original9 bodies are preserved; all whole-source inverses are exact in generated/current-native-worker/window-search-routing-full-inverses-and-runtime-1.json. No new command/script/target is required because the existing permanent registered Window file is already in the whole owner route.

The original Mode printable-key assertion is unchanged and will be observed in the fresh whole genericUI runtime. Strict new fixture compilation is separate.
