// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names, kebab-case -> camelCase): one
// `keyword key=value ...` line per EquationMutation variant, exactly what `print_op` writes.
grammar Mathematical_equation_mutations;

line: changeGraphDirected | updateGraphAlgorithm | createNode | deleteNode | deleteNodes | changeNodeLabel | moveNode | connectNodes | disconnectNodes | insertPoint | removePoint | movePoints | changeCoefficient | moveNodes | setNodePositions | setPointPositions ;
changeGraphDirected: 'change-graph-directed' SP 'new-directed=' boolean ;
updateGraphAlgorithm: 'update-graph-algorithm' SP 'new-algorithm=' quoted SP 'new-algorithm-seed=' optionalQuoted ;
createNode: 'create-node' SP 'id=' quoted SP 'label=' quoted SP 'x=' number SP 'y=' number ( SP 'index=' count )? ;
deleteNode: 'delete-node' SP 'id=' quoted ;
deleteNodes: 'delete-nodes' SP 'ids=' quoted ;
changeNodeLabel: 'change-node-label' SP 'id=' quoted SP 'new-label=' quoted ;
moveNode: 'move-node' SP 'id=' quoted SP 'x=' number SP 'y=' number ;
connectNodes: 'connect-nodes' SP 'id=' quoted SP 'source=' quoted SP 'target=' quoted ( SP 'index=' count )? ;
disconnectNodes: 'disconnect-nodes' SP 'id=' quoted ;
insertPoint: 'insert-point' SP 'index=' count SP 'x=' number SP 'y=' number ;
removePoint: 'remove-point' SP 'index=' count ;
movePoints: 'move-points' SP 'indices=' quoted SP 'dx=' number SP 'dy=' number ;
changeCoefficient: 'change-coefficient' SP 'label=' count SP 'numer=' quoted SP 'denom=' quoted ;
moveNodes: 'move-nodes' SP 'ids=' quoted SP 'dx=' number SP 'dy=' number ;
setNodePositions: 'set-node-positions' SP 'positions=' quoted ;
setPointPositions: 'set-point-positions' SP 'positions=' quoted ;
quoted: DQUOTE ( escaped | ~DQUOTE )* DQUOTE ;
escaped: '\\' ( '\\' | DQUOTE ) ;
optionalQuoted: quoted | '-' ;
number: OCTET+ ;
count: DIGIT+ ;
boolean: 'true' | 'false' ;
points: '[' ( pair ( ';' pair )* )? ']' ;
pair: number ',' number ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself).
DQUOTE: '"' ;
DIGIT: [0-9] ;
OCTET: . ;
SP: ' ' ;
