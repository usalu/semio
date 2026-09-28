# Browser Acceptance Inputs and Checks

The hand-authored CSV input covers a quoted comma, an embedded newline, Unicode, an empty trailing cell, and doubled quote escaping. It is disposable test content and stays with this ticket as an input asset.

The actual hosted app must expose these values without flattening newlines or losing empty cells. Exercise one directly editable cell, preserve a draft until Apply, discard another edit, undo/redo the accepted edit, and save/reopen the file. Check both English and German control labels, accessible field names, keyboard focus and action activation, and failure visibility. Record actual UI evidence and console results in the ticket validation report. No browser action or test is considered executed by the existence of this checklist.
