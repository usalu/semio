# Reader Syntax and Parent Route Isolation

Current source-only audit; short output files and hash metadata only, no copied sources. Original private script now inventories root directly at320/416/435; cpSync defining operations are absent. No build/test ran.

Parent Discovery route remains a whitelist. ecmaRouteBlock10743–10781 explicitly handles const/expression/throw/return/block/if/finite for and rejects all other kinds; top-level route10871 rejects unknown/export/function statements by default. ecmaRouteValue10630 rejects unknown expression kinds. New async arrow, assertion or other AST kinds therefore remain refused unless explicitly admitted; lexical parser success alone cannot grant route ownership.

Namespace new member constructors stay invalid because ecmaRouteIdentifier(new.callee) accepts only identifier after its narrow unwrap, and imported-value check requires genuine declared symbol. Do not replace that with arbitrary imported-root traversal: it would allow namespace members beyond explicit ScriptRouter/Script bases. Imported Script/BundleScript base identity, type-only/local/shadowed refusal and one defining run method remain. Generic class parser still enforces one run; if generalized, parent must explicitly check body.length1 and run name rather than only body[0].

Templates are currently data only when every embedded expression is an admitted route argument. New nested semantic identifier decoding should preserve that scope check. Assertion wrappers may be transparent only after complete syntax validation and then the exact underlying authority checks; stripping an arbitrary token suffix or coercing unknown nodes to data would loosen policy. Async arrows need actual body authority checked under their own lexical parameter scope, with asynchronous invocation policy preserved; retaining kind arrow alone must not bypass body checks.

More real statement forms in eight consumers beyond earlier seven syntax gaps:

- Plugin lifecycle: actual try/finally around SQLite cleanup, for loops and async callback.
- Scene bounds: while advance loops and continue after expected-null bound; typed aliases/assertions.
- Scene path/image: try/finally spies, while loop, break, complex compound assignment and callbacks; both share one original test file.
- Pack ordering: nested try/finally and loops in the SQLite behavioral law.
- IO ownership: async function helper with typed return, while traversal, continue, try/finally and nested callback loops.
- Terrain: casts/types around actual row/refusal operations; simpler callbacks, but no full-file completeness claim.
- Draft wire: same larger text-splice file also contains unrelated while/switch/type constructs and control statements; a schema reader at the tail cannot establish whole-file absence if prefix syntax is unsupported.

Current parser supports const-only for-of, if, simple block/return/throw/function and restricted class. It does not currently parse try/finally/catch, while/switch, break/continue, TS type/interface statements, postfix assertion wrappers, or async arrow modifiers. Full-file parsing must explicitly report unresolved for these real sources. A partial reader index must preserve scopes/offsets and mark unparsed regions, not silently drop them or claim a definitive negative. Transparent type-only declarations can be represented/ignored only through validated syntax and must not shadow runtime values incorrectly.

Portable policy regressions: parsed exported router/helper still rejected; ordinary class with extra methods rejected route; local/type-only/shadowed Script rejected; namespace unknown constructor rejected; template hidden unknown call rejected; assertion around unknown constructor remains rejected; genuine async forwarded route accepted only according to existing route semantics/body requirements. Independent locked TypeScript AST should check actual node kinds and lexical binding, not only call text.

Exact observer hashes are in `📥️route-audit.json`. Source state may drift with peers; no authorship adoption or runtime success is inferred.
