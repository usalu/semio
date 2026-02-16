{{- if not .continue }}
Analyze the {{ target }} in detail. Identify what is missing, what needs to be updated, and any inconsistencies. Provide a comprehensive analysis of the current state and required changes.

{{ .task }}
{{- else }}
Continue.
{{- end }}