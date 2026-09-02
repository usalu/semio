// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Mathematical_mathematical_mutations;

DOCUMENT: 'schema' [ ]+ 'mathematical.mathematical.mutations' ;

line: changeGraphDirected | updateGraphAlgorithm | replaceGraph | createNode | deleteNode | deleteNodes | changeNodeLabel | moveNode | connectNodes | disconnectNodes | replacePoints | insertPoint | removePoint | movePoint ;
changeGraphDirected: 'change-graph-directed' SP boolean ;
updateGraphAlgorithm: 'update-graph-algorithm' SP text SP text? ;
replaceGraph: 'replace-graph' SP block ;
createNode: 'create-node' SP id SP text SP number SP number ;
deleteNode: 'delete-node' SP id ;
deleteNodes: 'delete-nodes' SP block ;
changeNodeLabel: 'change-node-label' SP id SP text ;
moveNode: 'move-node' SP id SP number SP number ;
connectNodes: 'connect-nodes' SP id SP text SP text ;
disconnectNodes: 'disconnect-nodes' SP id ;
replacePoints: 'replace-points' SP block ;
insertPoint: 'insert-point' SP number SP number SP number ;
removePoint: 'remove-point' SP number ;
movePoint: 'move-point' SP number SP number SP number ;
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
