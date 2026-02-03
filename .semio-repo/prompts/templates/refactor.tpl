{{- if not .continue }}
Refactor the {{ target }} implementation to improve code quality, maintainability, and structure. Dont stop until all tests pass. Ensure the refactoring follows best practices and doesn't break existing functionality.

{{ .task }}
{{- else }}
Continue.
{{- end }}