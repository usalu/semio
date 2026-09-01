// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Sequence_sequence_mutations;

DOCUMENT: 'schema' [ ]+ 'sequence.sequence.mutations' ;

line: createStep | deleteStep | moveStep | editStepParams | changeStepCollapsed | connectSteps | disconnectSteps | duplicateStep ;
createStep: 'create-step' SP stepBlock ;
deleteStep: 'delete-step' SP id ;
moveStep: 'move-step' SP id SP number SP number ;
editStepParams: 'edit-step-params' SP id SP text ;
changeStepCollapsed: 'change-step-collapsed' SP id SP boolean ;
connectSteps: 'connect-steps' SP id SP id SP id ;
disconnectSteps: 'disconnect-steps' SP id ;
duplicateStep: 'duplicate-step' SP id SP id SP number SP number ;
stepBlock: '{' NL stepFields '}' ;
stepFields: OCTET+ ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;
boolean: 'true' | 'false' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
