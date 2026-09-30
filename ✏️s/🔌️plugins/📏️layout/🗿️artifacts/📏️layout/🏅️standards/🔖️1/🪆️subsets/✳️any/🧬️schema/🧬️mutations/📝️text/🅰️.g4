// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Layout_layout_mutations;

DOCUMENT: 'schema' [ ]+ 'layout.layout.mutations' ;

line: renameLayout | changePrintTarget | changeDataFields | createPage | deletePage | renamePage | changePageWidth | changePageHeight | updatePageMargins | updatePageColumns | reorderPages | createStory | deleteStory | editStory | createLink | deleteLink | changeLinkPath | createFrame | deleteFrame | moveFrame | resizeFrame | rotateFrame | changeFrameFill | changeFrameStroke | changeFrameWrapMode | changeFrameColumns | updateGrid | setFrameFlags | updateParagraphStyle | updateTextFrame | updateLayer | createCharacterStyle | deleteCharacterStyle | updateCharacterStyle | updateParentPage | updateSpread | setPageParent | setPageGuides | setStoryRuns | updateLink | setPageOverrides | createLayer | setFrameLayer | setDrawingText | reorderFrame ;
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
rotateFrame: 'rotate-frame' SP id SP id SP number ;
changeFrameFill: 'change-frame-fill' SP id SP id SP block? ;
changeFrameStroke: 'change-frame-stroke' SP id SP id SP block? ;
changeFrameWrapMode: 'change-frame-wrap-mode' SP id SP id SP text ;
changeFrameColumns: 'change-frame-columns' SP id SP id SP number ;
updateGrid: 'update-grid' SP number SP number SP boolean ;
setFrameFlags: 'set-frame-flags' SP id SP id SP boolean? SP boolean? ;
updateParagraphStyle: 'update-paragraph-style' SP id SP text SP text SP number SP number SP number SP number SP text ;
updateTextFrame: 'update-text-frame' SP id SP id SP text SP text SP number SP number SP number SP number ;
updateLayer: 'update-layer' SP id SP id SP text SP boolean SP boolean ;
createCharacterStyle: 'create-character-style' SP id SP text? ;
deleteCharacterStyle: 'delete-character-style' SP id ;
updateCharacterStyle: 'update-character-style' SP id SP text? SP text? SP number? SP number? SP boolean? SP block? SP number? ;
updateParentPage: 'update-parent-page' SP id SP text SP number SP number ;
updateSpread: 'update-spread' SP id SP text ;
setPageParent: 'set-page-parent' SP id SP id? ;
setPageGuides: 'set-page-guides' SP id SP block ;
setStoryRuns: 'set-story-runs' SP id SP block ;
updateLink: 'update-link' SP id SP number SP number SP number SP text? ;
setPageOverrides: 'set-page-overrides' SP id SP block ;
createLayer: 'create-layer' SP id SP id SP text SP boolean? ;
setFrameLayer: 'set-frame-layer' SP id SP id SP id ;
setDrawingText: 'set-drawing-text' SP number SP text ;
reorderFrame: 'reorder-frame' SP id SP id SP boolean ;
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
