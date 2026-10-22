# Final Integration Sealing Dependency Audit

Read-only static review; final pins not yet refreshed. Old historical seals are intentional pending inputs, not current-ready evidence.

Required dependency order is acyclic: finalize154Source schema/phase/control metadata and guest7 implementation; seal guest own7 inputs; replace prefix-core exact guest inputs SHA literal; finish prefix6 implementation/schema markers; seal combined template83controls+154Source; seal prefix inputs own6+83controls+guest8+raw template. Guest8 includes own7 inputs.json. Prefix inputs is the terminal root and is not part of its own6 hash roster. Guest own7 does not include prefix files or combined template, so no cycle exists.

Actual prefix readOwnInputs verifies own6,83controls,guest8, hardcoded guest inputs SHA, raw template SHA, exact template control roster and every control pin equality. Guest derive authenticates copied combined schema through template pin, then requires exact source required/properties roster and equality to guest policy.sourcePaths. Combined schema closes explicit Source properties. Therefore final ordering must update prefix guest literal before prefix core pin, and combined template only after all83 control bytes are final; metadata mutations afterward require cascading reseal.

Current prefix hardcoded guest SHA is historical f37e0661c26a14d6be01b35e4ff75fca517e86d7548ddc8f63b480b6cc353c12; this is intentionally pending replacement, not an additional defect. Current guest own7 and prefix/template pins remain historical while Root finishes current Source cohort. No execution/Source-evaluation/CLI/cache effects performed.
