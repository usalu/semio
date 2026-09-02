// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start document`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Forms_forms_snapshot;

DOCUMENT: 'schema' [ ]+ 'forms.forms.snapshot' ;

artifactMark: 'semio forms.form.dsl v1' ;
document: artifactMark schemaLine idLine versionLine titleLine structureLine resultsLine ;
schemaLine: 'schema' '=' HEX ;
idLine: 'id' '=' HEX ;
versionLine: 'version' '=' HEX ;
titleLine: 'title' '=' optHex ;
structureLine: 'structure' '=' childHandle ;
resultsLine: 'results' '=' childHandle ;
optHex: '-' | HEX ;
childHandle: '[' HEX ',' HEX ']' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
HEX: [0-9a-f]* ;
