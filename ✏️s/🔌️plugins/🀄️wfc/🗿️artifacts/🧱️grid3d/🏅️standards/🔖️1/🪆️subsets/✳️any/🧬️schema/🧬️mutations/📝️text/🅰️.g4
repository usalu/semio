// 🅰️ wfc.grid3d.mutations — derived from 📖️.grammar.semio; rule names are the grammar's own, kebab folded to camelCase.
grammar MutationsGrid3d;

line : keyword [SP arguments] ;
keyword : 'change-seed' / 'resize-grid' / 'change-cell-sizes' / 'change-periodicity' / 'create-tile' / 'delete-tile' / 'change-tile-weight' / 'change-tile-media' / 'create-rule' / 'delete-rule' / 'pin-cell' / 'unpin-cell' / 'mask-cell' / 'unmask-cell' ;
arguments : OCTET+ ;

SP : ' ' ;
NL : '\r'? '\n' ;
OCTET : . ;
