// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start mutation`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Forms_forms_mutations;

DOCUMENT: 'schema' [ ]+ 'forms.forms.mutations' ;

mutation: createStep | deleteStep | reorderStep | renameStep | changeStepDescription | createBlock | deleteBlock | moveBlockToStep | replaceBlock | changeFormTitle ;
createStep: 'create-step' SP 'step=' step SP 'index=' optUsize ;
deleteStep: 'delete-step' SP 'id=' text ;
reorderStep: 'reorder-step' SP 'id=' text SP 'to-index=' usize ;
renameStep: 'rename-step' SP 'id=' text SP 'new-title=' text ;
changeStepDescription: 'change-step-description' SP 'id=' text SP 'new-description=' optText ;
createBlock: 'create-block' SP 'step-id=' text SP 'block=' block SP 'index=' optUsize ;
deleteBlock: 'delete-block' SP 'step-id=' text SP 'id=' text ;
moveBlockToStep: 'move-block-to-step' SP 'step-id=' text SP 'block-id=' text SP 'to-step-id=' text SP 'index=' usize ;
replaceBlock: 'replace-block' SP 'step-id=' text SP 'block=' block ;
changeFormTitle: 'change-form-title' SP 'new-title=' optText ;
step: quotedText ;
block: quotedText ;
text: quotedText ;
optText: '-' | quotedText ;
usize: OCTET+ ;
optUsize: '-' | usize ;
quotedText: DQUOTE OCTET* DQUOTE ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
SP: ' ' ;
OCTET: . ;
DQUOTE: '"' ;
