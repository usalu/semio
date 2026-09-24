// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Mathematical_equation_snapshot;

artifactMark: 'semio mathematical.equation.dsl v1' ;
document: artifactMark field* ;
field: 'notation' '=' childHandle
     | 'results' '=' childHandle
     | 'computed' '=' childHandle
     | 'equation' '=' VALUE
     | 'graph' '=' VALUE
     | 'geometry' '=' VALUE ;
childHandle: 'child_id' '=' TEXT 'target' '=' TEXT ;

// 📐 Framework dialect-primitive terminals.
VALUE: TEXT ;
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\],]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
