// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start document`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Lowpoly_lowpoly_snapshot;

DOCUMENT: 'schema' [ ]+ 'lowpoly.lowpoly.snapshot' ;

document: artifactMark docBody ;
artifactMark: 'lowpoly.lowpoly' ;
docBody: schemaLine objectsField? ;
schemaLine: 'schema' '=' IDENT ;
objectsField: 'objects' '=' '[' object* ']' | 'objects' '=' object* ;
object: '{' objectField* '}' ;
objectField: ('id' '=' IDENT) | ('name' '=' TEXT) | ('smooth-shading' '=' BOOL) | mesh | objectTransform | paintLayers ;
objectTransform: 'transform' '{' transformField* '}' ;
transformField: ('position' '=' vec3) | ('rotation' '=' vec3) | ('scale' '=' vec3) ;
mesh: 'mesh' '{' vertices halfedges faces '}' ;
vertices: 'vertices' '[' vertex* ']' | 'vertices' '{' vertex* '}' ;
vertex: '{' ('position' '=' vec3) ('normal' '=' vec3)? ('halfedge' '=' INT)? '}' ;
halfedges: 'halfedges' '[' halfedge* ']' | 'halfedges' '{' halfedge* '}' ;
halfedge: '{' ('vertex' '=' INT) ('next' '=' INT) ('twin' '=' INT)? ('face' '=' INT)? ('u' '=' FLOAT)? ('v' '=' FLOAT)? '}' ;
faces: 'faces' '[' face* ']' | 'faces' '{' face* '}' ;
face: '{' ('halfedge' '=' INT) ('smooth' '=' BOOL)? ('flipped' '=' BOOL)? '}' ;
paintLayers: 'paint-layers' '=' '[' paintLayer* ']' | 'paint-layers' '{' paintLayer* '}' ;
paintLayer: '{' paintLayerField* '}' ;
paintLayerField: ('name' '=' TEXT) | ('visible' '=' BOOL) | ('opacity' '=' FLOAT) | ('blend-mode' '=' IDENT) | ('pixels' '=' TEXT) ;
vec3: '(' (FLOAT | INT) ',' (FLOAT | INT) ',' (FLOAT | INT) ')' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
BOOL: 'true' | 'false' ;
FLOAT: '-'? [0-9]+ '.' [0-9]+ ;
IDENT: [A-Za-z_] [A-Za-z0-9_]* ;
INT: '-'? [0-9]+ ;
TEXT: [^ \t\r\n]+ ;
