{{- if not .continue }}
Extend the {{ target }} tests by testing more features and edge cases. Ensure comprehensive test coverage for all functionality. Dont create new test files, only extend existing ones. Make sure all tests pass.

{{ .task }}
{{- else }}
Continue.
{{- end }}