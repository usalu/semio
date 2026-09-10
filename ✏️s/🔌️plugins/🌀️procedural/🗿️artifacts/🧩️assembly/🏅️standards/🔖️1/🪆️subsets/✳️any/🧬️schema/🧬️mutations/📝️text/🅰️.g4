grammar Procedural_assembly_mutations;
KEYWORD: 'create-slot' | 'delete-slot' | 'create-rule' | 'delete-rule' | 'change-weight' | 'remove-weight' | 'connect-slots' | 'disconnect-slots' | 'change-seed' ;
line: KEYWORD .*? ;
