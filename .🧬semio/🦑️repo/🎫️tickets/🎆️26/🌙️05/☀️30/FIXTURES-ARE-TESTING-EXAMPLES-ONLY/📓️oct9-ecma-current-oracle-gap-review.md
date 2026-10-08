# ECMA Current Oracle Gap Review

Current test SHA `d0c6a38323a38e44a7b0d529d1af12073456e5a6fa94be0f49161004a1200166`. Plain examples were not yet present at read; Root scaffold is active. Recommendations below are coverage requirements, not claims that pending examples omit them. No source/test execution here.

Current independent AST oracle covers string and regex nodes only, then checks every own token slice and Ajv DTO admission. It does not independently classify identifiers, numbers or punctuation; own slice equality alone cannot prove their lexical kinds. Valid-source EOF is not asserted, unlike malformed-source EOF. Add a locked TypeScript scanner comparison for these lexical categories using parser-driven regex rescans/lexical goals where necessary; AST string/regex parity remains meaningful.

Minimal portable cases:

- Control close-parenthesis permits a regex expression statement: `if (ok) /[}]/u.test(x);` contrasted with call-result division `f() / 2` and parenthesized value division `(x) / 2`. A previous-token-only ')' heuristic misclassifies the first.
- Nested template interpolation contains regex character-class brace and another template: outer interpolation `/[}]/.test(x) ? nested-template-with-string : string`. Original expression spans must survive recursion and repeated literal texts. Template-end brace counting must ignore regex classes/escaped slash and nested template lexical modes.
- Strings include escaped matching quote/backslash, hex escape, four-digit Unicode, codepoint escape astral, escaped LF/CRLF continuation and actual non-BMP literal. Invalid cases include unescaped LF/CR, unfinished quote/backslash, truncated hex/Unicode and codepoint above10FFFF. Define supported ECMA semantics explicitly rather than treating every non-JSON escape invalid; do not use quote replacement+JSON parsing.
- Unicode identifier start/continue and escaped identifier: astral letter, combining continuation, `\u0061` binding, invalid escaped identifier start. Verify exact UTF16 start/end after astral content, not byte/codepoint offsets. Require each valid-source EOF at original source.length and nested expression EOF at span.start+span.text.length.
- Comments containing fake imports/compile/quotes/regex, escaped regex slash and invalid regex newline/class termination. Ensure comment tokens cannot yield provenance and regex flags are parsed according to actual lexical support.

Preserve current original text and offset DTOs. Independent grammar verdicts need actual TypeScript parse diagnostics for malformed forms; AST node shape alone can exist during parser error recovery. If implementation intentionally supports a closed subset, unresolved unsupported syntax must not invent import/validator evidence. Cases are plain data; no schema governing the test matrix is required.
