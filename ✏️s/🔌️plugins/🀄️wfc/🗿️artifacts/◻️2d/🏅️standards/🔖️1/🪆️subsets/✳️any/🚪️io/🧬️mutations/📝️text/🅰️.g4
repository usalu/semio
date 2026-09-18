// 🅰️ derived from 📖️.grammar.semio — kebab rule names rendered in camelCase.
grammar WfcWfc2dMutations;
line : keyword (' ' arguments)? ;
keyword : 'change-seed' | 'create-slot' | 'delete-slot' | 'move-slot' | 'resize-slot' | 'connect-slots' | 'disconnect-slots' | 'pin-slot' | 'unpin-slot' | 'create-tile' | 'delete-tile' | 'change-tile-weight' | 'change-tile-media' | 'create-rule' | 'delete-rule' ;
arguments : OCTET+ ;
OCTET : . ;
