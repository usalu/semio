// 🅰️ derived from 📖️.grammar.semio — kebab rule names rendered in camelCase.
grammar WfcWfc2dDiff;
diff : artifactMark change+ ;
change : scalarChange | collectionChange ;
artifactMark : 'wfc.wfc2d-diff' ;
scalarChange : ('schema' | 'seed') '=' TEXT ;
collectionChange : ('slots' | 'edges' | 'tiles' | 'rules') TEXT ;
TEXT : ~[\r\n]+ ;
