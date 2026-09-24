// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Layout_layout_snapshot;

artifactMark: 'semio layout.layout.dsl v1' ;
document: artifactMark docField* ;
docField: ('schema' '=' TEXT) | ('name' '=' TEXT) | ('grid' '=' gridField+) | ('paragraph-styles' '=' '[' paragraphStyle* ']') | ('character-styles' '=' '[' characterStyle* ']') | ('stories' '=' '[' story* ']') | ('links' '=' '[' link* ']') | ('parent-pages' '=' '[' parentPage* ']') | ('spreads' '=' '[' spread* ']') | ('pages' '=' '[' page* ']') | ('print-target' '=' TEXT) | ('data-fields-json' '=' TEXT) | ('background-drawing' '=' backgroundDrawingField+) | ('referenced-model' '=' referencedModelField+) ;
gridField: ('baseline-grid' '=' NUMBER) | ('baseline-offset' '=' NUMBER) | ('snap-to-baseline' '=' BOOL) ;
paragraphStyle: paragraphStyleField+ ;
paragraphStyleField: ('id' '=' TEXT) | ('name' '=' TEXT) | ('font-family' '=' TEXT) | ('font-size' '=' NUMBER) | ('font-weight' '=' UINT) | ('leading' '=' NUMBER) | ('tracking' '=' NUMBER) | ('alignment' '=' TEXT) ;
characterStyle: characterStyleField+ ;
characterStyleField: ('id' '=' TEXT) | ('name' '=' TEXT) | ('font-family' '=' TEXT) | ('font-size' '=' NUMBER) | ('font-weight' '=' UINT) | ('italic' '=' BOOL) | ('color' '=' rgba) | ('tracking' '=' NUMBER) ;
story: storyField+ ;
storyField: ('id' '=' TEXT) | ('content' '=' TEXT) | styleRuns ;
styleRuns: 'style-runs' '[' 'start:UINT' 'end:UINT' 'paragraph-style-id:REF' 'character-style-id:REF' ']' '{' styleRun* '}' ;
styleRun: UINT UINT TEXT TEXT ;
link: linkField+ ;
linkField: ('id' '=' TEXT) | ('path' '=' TEXT) | ('hash' '=' TEXT) | ('width' '=' UINT) | ('height' '=' UINT) | ('dpi' '=' UINT) | ('color-profile' '=' TEXT) | ('state' '=' TEXT) | ('proxy-data-url' '=' TEXT) | ('artifact-kind' '=' TEXT) | ('artifact-ref' '=' TEXT) ;
parentPage: parentPageField+ ;
parentPageField: ('id' '=' TEXT) | ('name' '=' TEXT) | ('width' '=' NUMBER) | ('height' '=' NUMBER) | ('layer-ids' '=' '[' TEXT* ']') | layers | frames ;
spread: spreadField+ ;
spreadField: ('id' '=' TEXT) | ('name' '=' TEXT) | ('page-ids' '=' '[' TEXT* ']') ;
page: pageField+ ;
pageField: ('id' '=' TEXT) | ('name' '=' TEXT) | ('spread-id' '=' TEXT) | ('parent-page-id' '=' TEXT) | ('width' '=' NUMBER) | ('height' '=' NUMBER) | ('margins' '{' marginField* '}') | ('columns' '{' columnField* '}') | guides | ('layer-ids' '=' '[' TEXT* ']') | layers | frames | overrides ;
marginField: ('top' '=' NUMBER) | ('right' '=' NUMBER) | ('bottom' '=' NUMBER) | ('left' '=' NUMBER) ;
columnField: ('count' '=' UINT) | ('gutter' '=' NUMBER) ;
guides: 'guides' '[' 'x:NUM' 'y:NUM' 'width:NUM' 'height:NUM' ']' '{' guide* '}' ;
guide: NUMBER NUMBER NUMBER NUMBER ;
layers: 'layers' '[' 'id:TEXT' 'name:TEXT' 'visible:BOOL' 'locked:BOOL' 'object-ids:LIST' ']' '{' layer* '}' ;
layer: TEXT TEXT BOOL BOOL '[' TEXT* ']' ;
overrides: 'overrides' '[' 'object-id:REF' 'bounds:BLOCK' 'visible:BOOL' 'locked:BOOL' ']' '{' override* '}' ;
override: TEXT ('{' boundsField* '}')? BOOL? BOOL? ;
frames: 'frames' '{' frame* '}' ;
frame: imageFrame | rectFrame | textFrame ;
imageFrame: 'image' frameField* ('link-id' '=' TEXT) ;
rectFrame: 'rect' frameField* rectField* ;
textFrame: 'text' frameField* textField* ;
frameField: ('id' '=' TEXT) | ('layer-id' '=' TEXT) | ('bounds' '{' boundsField* '}') | ('locked' '=' BOOL) | ('visible' '=' BOOL) ;
rectField: ('fill' '=' rgba) | ('stroke' '=' rgba) ;
textField: ('story-id' '=' TEXT) | ('thread-next' '=' TEXT) | ('columns' '=' UINT) | ('inset' '{' insetField* '}') | ('wrap-mode' '=' TEXT) ;
boundsField: ('x' '=' NUMBER) | ('y' '=' NUMBER) | ('width' '=' NUMBER) | ('height' '=' NUMBER) | ('rotation' '=' NUMBER) ;
insetField: ('x' '=' NUMBER) | ('y' '=' NUMBER) | ('width' '=' NUMBER) | ('height' '=' NUMBER) ;
backgroundDrawingField: ('handle' '=' childHandle) | ('content' '=' VALUE) ;
referencedModelField: ('target' '=' TEXT) | ('pin' '=' pinField+) | ('role' '=' TEXT) ;
pinField: ('kind' '=' ('head' | 'checkpoint' | 'snapshot')) | ('checkpoint_id' '=' TEXT) | ('blob_hash' '=' TEXT) | ('blob_size' '=' UINT) | ('blob_media_type' '=' TEXT) ;
childHandle: ('child_id' '=' TEXT) ('target' '=' TEXT) ;
rgba: NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ;

// 📐 Framework dialect-primitive terminals.
BOOL: 'true' | 'false' ;
UINT: [0-9]+ ;
NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)? ;
VALUE: TEXT ;
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\],]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
