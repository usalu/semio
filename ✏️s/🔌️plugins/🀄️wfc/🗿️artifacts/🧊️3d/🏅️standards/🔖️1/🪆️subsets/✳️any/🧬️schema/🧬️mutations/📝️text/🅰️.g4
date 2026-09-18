grammar Wfc3dMutations;

line : keyword (SP arguments)? ;
keyword : 'create-slot' | 'delete-slot' | 'move-slot' | 'resize-slot' | 'connect-slots' | 'disconnect-slots' | 'pin-slot' | 'unpin-slot' | 'create-tile' | 'delete-tile' | 'change-tile-weight' | 'change-tile-media' | 'create-rule' | 'delete-rule' | 'change-seed' ;
arguments : OCTET+ ;

SP : ' ' ;
OCTET : . ;
