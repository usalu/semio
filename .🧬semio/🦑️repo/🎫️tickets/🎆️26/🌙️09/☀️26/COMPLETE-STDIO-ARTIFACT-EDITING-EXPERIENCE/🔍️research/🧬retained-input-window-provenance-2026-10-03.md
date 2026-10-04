# Retained Input Window Provenance

The preview33 browser exposed a command ownership defect while testing edits in different CSV cells. After Source/Details had focus, committing the Table cell dispatched `set-cell` to `s.stdio.window.snapshot-details`. Native routing refused the command because that window does not own it. The local drafts survived, but no canonical edit occurred. This was an ownership failure before the intended cross-cell causality test could run.

Each mounted document window now binds its interpreted actions and intents to the originating window ID. The shell resolves this explicit provenance before the currently focused window. Chrome without an owning document window retains its existing explicit-argument/focus fallback. Domain arguments and existing origin/causal provenance remain intact.

A language-neutral schema/fixture defines an owning Table window and a separately focused Details window. The actual retained interpreter test focuses another input to trigger blur and verifies the emitted intent retains the Table owner. A companion ledger law verifies command arguments and causal metadata remain unchanged. Testing Library supplies the independent DOM/focus oracle.

The red witness failed with an undefined owner. The selected interpreter/Table suite then passed 12 laws; the complete editable-controls suite passed 65/65. Aggregate renderer typecheck reached only two errors in the simultaneously developing natural Open/Save methods; the owning agent is completing those methods and will rerun that gate. No fresh browser acceptance of this routing change is claimed until the native Source producer checkpoint is integrated into preview34.

Receipts: `🗑️generated/retained-window-provenance-red-1.log`, `🗑️generated/retained-window-provenance-green-1.log`, `🗑️generated/retained-window-provenance-input-green-1.log`, `🗑️generated/retained-window-provenance-typecheck-1.log`.
