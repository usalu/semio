# Native PDF page edits

The page-size action now treats width and height as distances from the existing MediaBox lower-left corner. The regression opens a two-page PDF with a nonzero origin, dispatches the page-size action, saves it, and compares the resulting document projection with the same edit performed independently by lopdf.

The first page draft uses the editor’s existing text insertion path to select a font and set the text position. With an empty font collection the action publishes the standard font and page content together. The base PDF 1.7 editor and its six sibling profile editors use the same command implementation. Multiline insertion sets the leading and emits one text-show operator per line with an explicit line advance. The regression checks the exported line-advance operators and both text operands with lopdf and reopens the result with the production codec.

The initial focused Cargo run stopped before compilation during source admission because the UI locale module declared an unresolved `axes` source. The parent task is tracking the shared package verification.
