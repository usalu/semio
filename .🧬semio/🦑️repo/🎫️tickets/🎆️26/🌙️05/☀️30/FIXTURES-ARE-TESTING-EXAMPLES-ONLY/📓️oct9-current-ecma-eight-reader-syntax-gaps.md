# Current ECMA Reader Syntax Gaps

Read-only current source inspection, not parser execution. Latest fixes are visible: malformed adjacent identifier patterns now refused, default import before `from` supported, spread ranges capture dot token start, parser raw scans call take/cancellation, return/throw use original source lineBreak, top-level semicolon skipping precedes statement start. These observations do not substitute for Root's active test result.

Concrete source coverage needed for eight chains:

- Async test callbacks `test("law",async()=>{...})` are not recognized as arrows: primary treats async as identifier and then ordinary call; following arrow cannot be parsed. Actual Plugin lifecycle and scene handoff tests use this form. Add async arrow/function syntax with explicit node semantics, preserve scope and original spans, or emit explicit unresolved source. Do not suppress all evidence as no reader.
- Postfix TS `as`/`satisfies` has no expression branch. Real typed example variables/record projections use casts; typed local declarations can be handled as pattern colon annotations, but expression assertions remain unresolved. Portable schema examples must test these real wrappers before claiming actual-file coverage.
- Helpers such as `(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"))` can be syntactically represented; current pattern accepts colon annotation. Independent binding must substitute literal arguments and retain captured fs import identity.
- Awaited Bun.file(new URL(...)).json chains are supported by current unary/new/member/call branches. The null risk is surrounding async callback/type declarations, not the genuine read form itself.
- Template local refs `${schema.$id}#/$defs/Grant` need binder/schema selector resolution. Parser template currently retains nested expression nodes but not literal chunk values in the AST; original range/source can supply chunks. Avoid treating Grant selector as schema root.
- Escaped identifier semantic mapping applies in ecmaProgram, but template nested parser350 instantiates directly from ecmaTokens without applying identifier decoding. `read` and escaped read inside interpolation can resolve different AST names. Use shared semantic token mapping for every nested source, preserve original spans/text separately.
- Namespace constructor `new oracle.Ajv()` remains conservative null because new callee is primary before member chain. Namespace default/member constructors require grammar support or unresolved classification.

The thirteen original candidates exercise simple whole/alias/readhelper/escaped/projection/row/root/export/shadow/reassignment/computed/comment/inline chains, but do not cover these actual async/cast/template selector combinations. Add the plain seven candidate sources in `📥️oct9-current-ecma-reader-syntax-gap-candidates.json`; no new fixture schema is required. Shape admission must stay canonical JSON/TS, with new async/assertion fields added schema-first if exposed.

Parent route still rejects export/function kinds through its whitelist, and genuine imported base checks remain. Generic class still only admits one run+extends; any later generalization must retain explicit parent body cardinality/name policy. Conservative null is acceptable only as unresolved evidence, never proof no whole-test reader exists.

No tests/builds or source edits were performed.
