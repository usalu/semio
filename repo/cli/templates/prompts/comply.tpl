{{- if not .continue }}
Get the {{ target }} implementation to comply with the set of tests. Dont remove any functionality from the tests. Make sure all tests pass by implementing the missing functionality correctly.

{{ .task }}
{{- else }}
Continue with getting the implementation to comply.
{{- end }}