{{- /*
  Markdown output templates for the repo CLI.
  Used by renderEntityMarkdownLink, formatMarkdownResult, and MarkdownRenderer.
*/ -}}

{{- define "md/entity_link" -}}
[{{ sanitizeProp .ID }}]({{ .URI }}){{ range .Props }} - `{{ . }}`{{ end }}
{{- end -}}

{{- define "md/entity_item" -}}
- [{{ sanitizeProp .ID }}]({{ .URI }}){{ range .Props }} - `{{ . }}`{{ end }}
{{- end -}}

{{- define "md/analyze_total" -}}
- **Total Breachs**: {{ .Total }}
{{- end -}}

{{- define "md/analyze_autofixable" -}}
- **Autofixable**: {{ .Autofixable }}
{{- end -}}

{{- define "md/breach" -}}
- [{{ .Kind }}](repo://statute/{{ pathToUriPath .Kind }}) - {{ .Scope }}:{{ .Line }} - {{ .Summary }}
{{- end -}}

{{- define "md/error" -}}
**Error: {{ .Message }}**
{{- end -}}

{{- define "md/error_detail" -}}
> {{ .Detail }}
{{- end -}}

{{- define "md/result_fallback" -}}
[{{ sanitizeProp .Name }}]({{ .URI }})
{{- end -}}

{{- define "md/tree_node_category" -}}
{{- .Indent }}- {{ if .URI }}[{{ .Label }}]({{ .URI }}){{ else }}{{ .Label }}{{ end }}
{{- end -}}

{{- define "md/tree_node_entity" -}}
{{- .Indent }}{{ .Content }}
{{- end -}}
