{{- if not .continue }}
Something is not working with {{ target }}.

{{ .task }}
{{- else }}
Continue.
{{- end }}

Extend the existing test file the missing parts that were not tested. Dont create any new test files. A single test should always cover one unit and do multiple tests for that unit. Tests should not interact with external systems. Make sure all tests pass.

Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The target is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to track everything (plan, todos, changes, summary, etc) in `.semio-repo/tickets/YYYY/MM/DD/TICKETSLUG*/ticket.md`
Dont keep any legacy api or backwards compatiblity.