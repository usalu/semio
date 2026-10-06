// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). meshContent carries the half-edge mesh JSON.
grammar Lowpoly_lowpoly_snapshot;

artifactMark: 'semio lowpoly.lowpoly.dsl v1' ;
document: artifactMark docField* ;
docField: 'schema' '=' TEXT
        | 'objects' '=' '[' object* ']' ;
object: objectField+ ;
objectField: 'id' '=' TEXT
           | 'name' '=' TEXT
           | 'smooth-shading' '=' BOOL
           | 'mesh-content' '=' TEXT
           | 'transform' '=' transformField+
           | 'mesh' '=' childHandle
           | 'paint-layers' '=' '[' paintLayer* ']' ;
transformField: 'position' '=' coord3
              | 'rotation' '=' vec3
              | 'scale' '=' vec3 ;
paintLayer: paintLayerField+ ;
paintLayerField: 'name' '=' TEXT
               | 'visible' '=' BOOL
               | 'opacity' '=' FLOAT
               | 'blend-mode' '=' TEXT
               | 'pixels' '=' TEXT ;
childHandle: 'child_id' '=' TEXT 'target' '=' TEXT ;
coord3: '@' FLOAT ',' FLOAT ',' FLOAT ;
vec3: FLOAT ',' FLOAT ',' FLOAT ;

// 📐 Framework dialect-primitive terminals.
BOOL: 'true' | 'false' ;
FLOAT: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)? ;
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\],@]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
