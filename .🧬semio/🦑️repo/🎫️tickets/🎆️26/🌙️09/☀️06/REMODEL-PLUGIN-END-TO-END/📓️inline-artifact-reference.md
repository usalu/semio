# Inline Artifact Reference Reader

The Remodel TypeScript Text reader consumes the native inline `target=artifact-id=… artifact-kind=… standard=… subset=…` record, validates the four literal fields, and rejects the former braced target form. The SQLite literal fixture covers the canonical spelling, malformed and duplicate members, and an independent AJV schema check.

The focused SQLite TypeScript suite passed through `repo tools exec --light bun -- test` with 27 tests and 988 assertions, including the committed native Text examples. The source-only repository target was stopped after its infrastructure did not return; the focused run completed successfully.
