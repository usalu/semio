grammar Expression;

expr
    : atom
    | operation
    ;

operation
    : IDENTIFIER LPAREN expr* RPAREN
    ;

atom
    : NUMBER
    | IDENTIFIER
    ;

NUMBER
    : [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?
    ;

IDENTIFIER
    : [a-zA-Z_][a-zA-Z0-9._-]*
    ;

LPAREN : '(' ;
RPAREN : ')' ;

WS
    : [ \t\r\n]+ -> skip
    ;
