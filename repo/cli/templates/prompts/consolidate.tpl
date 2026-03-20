{{- if not .continue }}
Consolidate the {{ target }}.

{{ .task }}
{{- else }}
Continue with the consolidation.
{{- end }}

Extend the existing test file to cover all new features. Dont create any new test files. A single tests should always cover one unit and do multiple tests for that unit. Tests should not interact with external systems. Make sure all tests pass.

Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The target is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Everything is tracked over agent hooks.
Dont keep any legacy api or backwards compatiblity.