# Refined ECMA Current Remaining Boundaries

Current lower source SHA `5ea1bdc94243f5a73ad6f855db5cf87dc2c2f3809780a6dab20029f983780f4b`; Root reports actual3/457 GREEN, no independent test execution by this lane. Previous malformed identifier/numeric and long-atom cancellation repairs are visible; no blanket full ECMA grammar claim.

Concrete source-supported remaining cases:

- `class C {} /x/.test(s);`: class declaration body opens after identifier C, current brace goal marks it expression; following slash becomes division. TypeScript AST expects regular expression expression statement after class declaration. Contrast `const C = class {} / 2 / g;` expression division.
- `function f(): void {} /x/.test(s);`: TS return annotation separates close-paren from opening brace; current preceding-token goal cannot preserve declaration context. Likewise generic declarations and arrow return type annotation require explicit syntax context. Treat unsupported grammar as unresolved/failclosed rather than definite reference absence.
- Escaped identifiers retain raw spelling but no decoded binding identity. `import {readFileSync as \u0072ead} from "node:fs"; read("x");` binds the same identifier as read, while runtime alias pass compares raw token.text. Add canonical identifier value without changing original text/span; independent TypeScript identifier.escapedText supplies oracle. Keyword escapes need actual grammar diagnostics, not automatic keyword equivalence.
- Runtime reference consumer165 scans tokens but has no invalid-token rejection before using the reference set. Malformed trailing source can produce a valid prefix plus invalid token and silently omit later evidence. Current static registry scan and token evidence must not be advertised complete for this source. Propagate explicit unresolved parse evidence or refuse according to actual owner API; preserve original source/hash and no guessed empty result.

Atomic checkpoints now exist in comment/string/template/regex/numeric/identifier loops and decoder; cancellation callback forwards from runtime context. Identifier escape match uses source.slice(index), so repeated escaped segments can allocate suffixes repeatedly; avoid source-wide suffix copies for scaling, without attributing actual CPU behavior. Numeric radix/separators now fail on missingdigits and adjacent malformed identifier escapes; closed subset policy still needs independent diagnostics for unsupported forms.

Joined-reader minimum remains one source pass over inventory files with lexical bindings for schema/example reads, actual validator aliases and whole argument versus projection identity. Decode identifier identity, retain original UTF16 spans, invalidate bindings on reassignment/shadowing and preserve unresolved dynamic chains. Positive cases must include actual produced snapshot whole validation, raw.vertex_groups and row.input domain validation. No schema/reader naming waiver.

No edits, builds, compilers or tests. This is current source observation only; Root owns implementation.
