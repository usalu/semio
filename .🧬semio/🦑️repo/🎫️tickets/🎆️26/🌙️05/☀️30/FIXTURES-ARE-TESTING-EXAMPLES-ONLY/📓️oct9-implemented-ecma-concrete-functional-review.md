# Implemented ECMA Concrete Functional Review

Current defining source SHA `fc8baa805ebb611aa5ed10e24f7a86f70ee2a9f49a3200745730942b9124f167`; source-only review, no test executed. Root reports lower GREEN2/255; this report does not independently claim runtime behavior.

Concrete unresolved lexical defects by source inspection:

- Brace goal treats every opening brace after ')' as statement block. `const f = function() {} / 2 / g;` is division after a function expression, but closing brace marks regex allowed and consumes `/ 2 /` as regex. Add contrast actual function declaration followed by regex, function expression division and arrow expression block division. Correct classification requires syntactic expression/declaration context, not only prior token.
- Malformed escaped identifiers such as truncated Unicode, nonhex and above10FFFF make identifierEnd stop, then become punctuation/plain identifiers rather than invalid. Numeric `0x`, `1__2`, `1e+` likewise split ordinary tokens. Add portable invalid cases; classification must fail closed before reader evidence.
- Cancellation callback runs only once per outer token loop. Long quoted strings, regex, template text, Unicode identifier, comment and numeric scans contain no cooperative checks. Add a plain long literal/source case and bounded callback that throws during scanning, including nested interpolation; callback must be checked inside atomic loops or bounded chunks. EcmaStringValue separately loops synchronously without cancellation.

Current UTF16 offset+source-slice emission and template expression spans preserve original coordinates; recursive Discovery parser now passes source.text/source.start. Runtime imports lower ecmaTokens/ecmaStringValue directly; no reverse Discovery/runtime cycle introduced. String escape decoder handles quote, continuation, hex, fixed/codepoint Unicode; verify strict lexical semantics with independent parser diagnostics and no invalid token provenance.

Current package is @semio-tech/compiler-syntax-ecma-ts:test, cache:false, outputs:[], explicit complete owner inputs, permanent script dispatch and thin runOwnedCommand. Live launch41964–67 uses Bun+Nx --skip-nx-cache; taxonomy roster contains 🟨ecma at12111. Source test runner60s is explicit; no normal producer credit. Seed launch field was not independently read in this bounded review and must be joined by Root exact own registration. Genuine schema DTOs live in canonical child schema; plain test examples remain schema-free.

No source edits/builds/tests; peer/current source observations are not authorship.
