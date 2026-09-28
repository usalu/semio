// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Forms_forms_snapshot;

artifactMark: 'semio forms.form.dsl v1' ;
document: artifactMark field* EOF ;
field: 'schema' '=' text
     | 'id' '=' text
     | 'version' '=' text
     | 'title' '=' text
     | 'definition' '=' definition
     | 'responses' '=' '[' response* ']'
     | 'structure' '=' childHandle
     | 'results' '=' childHandle ;
childHandle: 'child_id' '=' text 'target' '=' text ;

definition: 'steps' '=' '[' step* ']' ;
step: 'id' '=' text 'title' '=' text ('description' '=' text)? 'blocks' '=' '[' question* ']' ;
question: 'id' '=' text 'label' '=' text 'kind' '=' text questionField* ;
questionField: questionName '=' value | 'condition' '{' expression? '}' ;
questionName: 'description' | 'required' | 'placeholder' | 'default' | 'min' | 'max' | 'step' | 'unit' | 'text' | 'options' | 'fields' | 'schema' | 'src' | 'accept' | 'fixture-slug' | 'params' ;
response: 'id' '=' text 'submitted-at' '=' text 'definition-version' '=' text 'answers' '=' '[' answer* ']' ;
answer: 'question-id' '=' text 'label' '=' text 'kind' '=' text 'value' '=' value ;
value: text | '[' item* ']' | '{' entry* '}' ;
item: entry | value ;
entry: text '=' value ;
expression: 'const' 'value' '=' value | 'var' 'name' '=' text | 'eq' 'left' '{' expression '}' 'right' '{' expression '}' | ('and' | 'or') 'items' '{' expression* '}' | 'truthy' 'expr' '{' expression '}' ;
text: TEXT | questionName | 'id' | 'label' | 'kind' | 'title' | 'version' | 'definition' | 'responses' | 'structure' | 'results' | 'child_id' | 'target' | 'steps' | 'blocks' | 'condition' | 'submitted-at' | 'definition-version' | 'answers' | 'question-id' | 'value' | 'const' | 'var' | 'name' | 'eq' | 'left' | 'right' | 'and' | 'or' | 'items' | 'truthy' | 'expr' ;
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\]]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
