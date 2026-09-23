// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Mathematical_equation_snapshot;

artifactMark: 'semio mathematical.equation.dsl v1' ;
document: artifactMark notationLine resultsLine computedLine equationLine graphLine geometryLine ;
notationLine: 'notation' '=' childHandle ;
resultsLine: 'results' '=' childHandle ;
computedLine: 'computed' '=' childHandle ;
equationLine: 'equation' '=' HEX ;
graphLine: 'graph' '=' HEX ;
geometryLine: 'geometry' '=' HEX ;
childHandle: '[' HEX ',' HEX ']' ;

// 📐 Framework dialect-primitive terminal (not defined in the .semio itself).
HEX: [0-9a-f]* ;
