// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start document`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Layout_layout_snapshot;

DOCUMENT: 'schema' [ ]+ 'layout.layout.snapshot' ;

artifactMark: 'semio layout.layout.dsl v1' ;
document: artifactMark schemaLine nameLine gridLine paragraphStylesLine characterStylesLine storiesLine linksLine parentPagesLine spreadsLine pagesLine printTargetLine dataFieldsJsonLine backgroundDrawingLine referencedModelLine ;
schemaLine: 'schema' '=' HEX ;
nameLine: 'name' '=' HEX ;
gridLine: 'grid' '=' jsonHex ;
paragraphStylesLine: 'paragraphStyles' '=' jsonHex ;
characterStylesLine: 'characterStyles' '=' jsonHex ;
storiesLine: 'stories' '=' jsonHex ;
linksLine: 'links' '=' jsonHex ;
parentPagesLine: 'parentPages' '=' jsonHex ;
spreadsLine: 'spreads' '=' jsonHex ;
pagesLine: 'pages' '=' jsonHex ;
printTargetLine: 'printTarget' '=' jsonHex ;
dataFieldsJsonLine: 'dataFieldsJson' '=' jsonHex ;
backgroundDrawingLine: 'backgroundDrawing' '=' jsonHex ;
referencedModelLine: 'referencedModel' '=' jsonHex ;
jsonHex: HEX ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
HEX: [0-9a-f]* ;
