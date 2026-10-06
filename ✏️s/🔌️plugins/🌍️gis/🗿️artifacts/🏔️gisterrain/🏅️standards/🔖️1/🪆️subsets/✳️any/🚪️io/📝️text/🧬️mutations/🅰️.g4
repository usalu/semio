// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Gis_gisterrain_mutations;

DOCUMENT: 'schema' [ ]+ 'gis.gisterrain.mutations' ;

line: changeExaggeration | changeImportedFeatures ;
changeExaggeration: 'change-exaggeration' SP number ;
changeImportedFeatures: 'change-imported-features' ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;
boolean: 'true' | 'false' ;
block: '{' NL OCTET+ '}' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
