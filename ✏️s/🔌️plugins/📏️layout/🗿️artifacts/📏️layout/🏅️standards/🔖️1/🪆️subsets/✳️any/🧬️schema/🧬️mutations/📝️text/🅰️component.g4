// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Layout_layout_mutations;

DOCUMENT: 'schema' [ ]+ 'layout.layout.mutations' ;

line: renameLayout | changePrintTarget | changeDataFields | createPage | deletePage | renamePage | changePageWidth | changePageHeight | updatePageMargins | updatePageColumns | reorderPages | createStory | deleteStory | editStory | createLink | deleteLink | changeLinkPath | createFrame | deleteFrame | moveFrame | resizeFrame | changeFrameFill | changeFrameStroke | changeFrameWrapMode | changeFrameColumns ;
renameLayout: 'rename-layout' SP text ;
changePrintTarget: 'change-print-target' SP text? ;
changeDataFields: 'change-data-fields' SP text? ;
createPage: 'create-page' SP block SP number? ;
deletePage: 'delete-page' SP id ;
renamePage: 'rename-page' SP id SP text ;
changePageWidth: 'change-page-width' SP id SP number ;
changePageHeight: 'change-page-height' SP id SP number ;
updatePageMargins: 'update-page-margins' SP id SP number SP number SP number SP number ;
updatePageColumns: 'update-page-columns' SP id SP number SP number ;
reorderPages: 'reorder-pages' SP id SP number ;
createStory: 'create-story' SP block SP number? ;
deleteStory: 'delete-story' SP id ;
editStory: 'edit-story' SP id SP text ;
createLink: 'create-link' SP block SP number? ;
deleteLink: 'delete-link' SP id ;
changeLinkPath: 'change-link-path' SP id SP text ;
createFrame: 'create-frame' SP id SP block SP number? SP id? ;
deleteFrame: 'delete-frame' SP id SP id ;
moveFrame: 'move-frame' SP id SP id SP number SP number ;
resizeFrame: 'resize-frame' SP id SP id SP number SP number ;
changeFrameFill: 'change-frame-fill' SP id SP id SP block? ;
changeFrameStroke: 'change-frame-stroke' SP id SP id SP block? ;
changeFrameWrapMode: 'change-frame-wrap-mode' SP id SP id SP text ;
changeFrameColumns: 'change-frame-columns' SP id SP id SP number ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;
boolean: 'true' | 'false' ;
block: '{' NL OCTET+ '}' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
