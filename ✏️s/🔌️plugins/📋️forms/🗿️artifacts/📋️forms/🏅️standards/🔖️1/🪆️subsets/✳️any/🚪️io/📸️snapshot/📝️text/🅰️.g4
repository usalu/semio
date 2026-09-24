// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Forms_forms_snapshot;

artifactMark: 'semio forms.form.dsl v1' ;
document: artifactMark field* ;
field: 'schema' '=' TEXT
     | 'id' '=' TEXT
     | 'version' '=' TEXT
     | 'title' '=' TEXT
     | 'structure' '=' childHandle
     | 'results' '=' childHandle ;
childHandle: 'child_id' '=' TEXT 'target' '=' TEXT ;

// 📐 Framework dialect-primitive terminal: a bare word or a double-quoted, backslash-escaped string.
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\]]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
