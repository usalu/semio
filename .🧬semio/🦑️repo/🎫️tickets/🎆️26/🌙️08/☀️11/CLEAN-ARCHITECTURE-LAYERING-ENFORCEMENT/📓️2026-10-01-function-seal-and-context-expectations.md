# Function Seal and Context Expectations

Read-only review, 2026-10-01. No edits/tests/compiler. Root is implementing the bounded graph proof; this identifies smallest accepted body kind and exact affected existing expectations.

## Smallest allowed local body

For the present Home need, accept an ordinary **free function body** with a parser-proven fn declaration, exact signature/body delimiters and no unknown rewriting attributes or enclosing macro token tree. Home vector() is plain free `fn vector(id:&str)->Result<Vector,String>` with no attributes. Do not accept a nearest brace merely because a fn token appears somewhere before it. Do not initially accept impl/trait method bodies, closures, const/async/unsafe/extern function forms, anonymous consts, arbitrary expression blocks or nested macro transcribers unless each classification is schema/native-proven. This conservative set is sufficient for the current actual source; unsupported forms produce typed scope refusal rather than migration aliases.

A free function can still be rewritten by `#[foreign_attr]`, including cfg_attr that inserts a procedural attribute. A fn inside `quote!{...}` or a macro_rules transcriber is not an authored executable free function. Inspect **all** enclosing item attributes/unknown macro token-tree ancestors, including a rewritten enclosing impl/module if those containers are later supported. Allow only explicitly known inert/builtin attributes when their body-preservation is implemented; safest first cut requires no function attributes, with independently tested cfg/test support if needed by synthetic rows. Export attributes on the template itself always prevent local sealing.

Parse the signature separately from the body: generics/return types can contain brace-delimited const arguments or macro token trees. Choosing the first brace after fn/parameters can mistake a signature const argument for the body. Home's simple named return `Result<Vector,String>` is enough for a closed signature grammar. Existing private Rust mutation-codec function-range helper around discovery8879–8898 scans function ranges, but is not a general exported/attribute-safe classifier; do not claim it already closes this requirement.

Inside an accepted function body, a nested function/block/inline module can inherit the textual macro. Gather calls across the entire postdefinition body subtree until the function closing brace. Do not truncate at the nested block or count calls outside the body. Any out-of-line `mod` or include! in that region refuses external reachability; nested same-file module calls remain source-file-origin data inputs only if the invocation scanner covers them and no export/reexport/helper mechanism escapes. A method/trait/impl brace cannot serve as a function-local proof without its own fn body. Static literal data facts remain available when scope classification refuses a dynamic template.

Concrete hostile rows: fake fn inside outer macro/quote; attributed fn and cfg_attr foreign transform; fn-signature anonymous const brace; macro inside inline module adjacent to a fn; impl block with macro beside methods; method with rewriting impl attribute; nested inline module call versus nested external child; function-local macro_export. Compare native compiler reachability with typed scope refusal, not with unsupported form deletion.

## Existing context expectations affected

All paths below start at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

- `🧫️fixtures/🧱️rust-source-direction/🔣️.json:1124–1139` contains a directly authored context in an API rejection case (moduleBase general/different/sourceChain). Required incoming-origin wire must be added explicitly there and schema closed keys updated; do not make metadata optional just to preserve this fixture.
- `🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts:161–162` constructs standalone TypeScript declaration strings for RustModuleContext/RustModuleGraph. Add required mount origin union/declarations to this extracted test interface. This is an actual compiling test wrapper, not merely a comment.
- `🧪️tests/🔬️workspace-contract/🟦️.ts:1069,1074–1078` assert exactly one facade root/inline os_dsl/out-of-line component/wrapped leaf context for selected Cargo/module/source scopes. Additional origin alternatives may intentionally change lengths; verify genuine unchanged singleton source graphs, while dual-origin cases must assert alternatives rather than weakening checks to >=1.
- `🧪️tests/🔬️workspace-contract/🟦️.ts:4831` checks mounted module manifest identities via set projection; added fields should not alter unique manifest authority. New duplicates from origin alternatives must not create extra manifests.
- `🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:178,193,199` project context manifest sets and assert two-package/shared source authority. Preserve both actual manifests; never pick one origin to unblock scope.
- `🧪️tests/🧱️rust-source-direction/🟦️.ts:232–239` builds graph from native fixture code mounts and passes its source contexts to target/edge APIs. Existing mounted input parity must remain; incoming include origin becomes deliberate proof metadata, not an API escape.

Non-test live consumers also depend on sourceChain: `🧹️normalization/🟦️.ts:4524,4563–4573,4613` derives proof paths and matches parent-chain owners. New origin alternatives must remain inspectable; context dedup cannot silently collapse them. `🕸️dependencies/🧭️direction/🦀️source/🟦️.ts:38,51` uses sourceScope/moduleBase for inline path and manifest context. Preserve these meanings while adding origin; no caller may reconstruct absence from an empty chain.

Current graph context constructor/dedup is discovery8555–8591. Require root/module/include discriminator on each constructed context and include it (or an origin alternatives set) in canonical identity. A source mounted through private module and include into same module context must retain both. Actual PDF private cfg(test) file-module remains singleton if no other active mount exists; Home body sealing does not require inventing a private parent but still requires canonical manifest source context. Missing manifest/context is typed refusal.

No runtime result is claimed by this audit.
