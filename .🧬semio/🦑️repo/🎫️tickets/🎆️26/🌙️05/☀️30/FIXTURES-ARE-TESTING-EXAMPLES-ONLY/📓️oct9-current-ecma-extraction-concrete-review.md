# Current ECMA Extraction Concrete Review

Read-only source-derived reproductions, not executed test outcomes. Exact source hashes and portable examples are in `📥️oct9-current-ecma-extraction-reproductions.json`.

Concrete acceptance/range hazards:

1. Pattern241–249 accepts the first identifier regardless of trailing tokens. `const schema fixture = value;` and `function f(a b){read(a);}` can become valid-looking first-binding AST nodes even though TypeScript reports syntax errors. Permit only a validated single binding with an explicitly parsed optional type annotation; malformed leftovers must be null. Destructuring fallback picks last identifier in a member, so nested/default/computed patterns need exact structure or unresolved treatment, not a fabricated simple binding.
2. Spread nodes281 and323 derive start from operand.start minus three. In `f(... /*original*/ value)` the offset points inside intervening text rather than original dots. Capture spread token.start before consumption and preserve operand independently.
3. Return528 ignores line-terminator restriction: `return\nread("x")` is represented as returned call instead of empty return and separate call statement. Throw line terminator must refuse. Original token gaps retain the source offsets but token API alone has no trivia text; parser needs original source or line-break metadata to implement this safely.
4. Statement505 captures start before statementValue skips semicolons; `;;const x=read("x")` gives const statement a preceding empty-statement start. Empty statements should be represented separately or skip before start capture.
5. Parser direct index++ in variable scanning405+, for initializer490, and class return annotation461 does not call supplied cancellation. Atomic scanner cancellation is useful but does not bound later parser scans after lexical processing. Invoke checkCancellation during each raw scan, including semantic token mapping if expensive.

Conservative unsupported syntax, not misclassification: new Namespace.Ajv() fails primary new parsing because constructor callee primary does not admit dotted member before arguments; ordinary classes without extends or with methods other than exactly one run return null; async function/export default/function expressions/type-only complex declarations likewise require explicit unsupported status. For a provenance index, null must become unresolved source evidence rather than no readers.

Escaped identifiers are now decoded by ecmaProgram semantic token mapping while start/end remain original; identifier property/pattern/import names use that semantic text. Import module literal uses ecmaStringValue, so escaped strings resolve semantically. Do not reuse semantic token.text to hash original callsite bytes.

Parent route policy remains conservative: root switch does not admit export/function nodes; type-only/local/shadowed Script bases fail real imported binding check. Generic class parsing currently still enforces one run method and extends, so moving that restriction to parent policy later must add explicit `body.length===1` and method name run validation there. Current parent takes only body[0]; relaxing generic class parse without strengthening parent would admit extra methods.

Independent existing TypeScript call-text parity does not expose malformed-pattern semantics or return parent association; add parseDiagnostics/statement kind and node offsets comparisons. No blanket ECMA/TypeScript grammar completeness or current test pass is inferred. No source edits, tests or compilers were run.
