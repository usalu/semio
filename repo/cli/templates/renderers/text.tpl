{{- /*
  Text/human-readable output templates for the repo CLI.
  Used by renderEntityHuman, formatResult, and HumanRenderer.
*/ -}}

{{- define "text/entity" -}}
{{- if .ID }}{{ colorize (sanitizeProp .ID) "bold" .IsTTY }}{{ end -}}
{{- range $i, $p := .Props -}}
{{- if or (ne $.ID "") (gt $i 0) }} {{ end -}}
{{- colorize $p (propColor $i) $.IsTTY -}}
{{- end -}}
{{- end -}}

{{- define "text/analyze_summary" -}}
{{ colorize "→" "blue" .IsTTY }} found {{ .Total }} breachs{{ if .Autofixable }} ({{ .Autofixable }} autofixable){{ end }}
{{- end -}}

{{- define "text/breach" -}}
  {{ colorize "breach" "red" .IsTTY }} {{ .Kind }} {{ colorize (printf "%s:%s" .Scope .Line) "dim" .IsTTY }} {{ .Summary }}
{{- end -}}

{{- define "text/fix" -}}
{{ colorize "→" "blue" .IsTTY }} fixed {{ .Fixed }} breachs ({{ .Remaining }} remaining)
{{- end -}}

{{- define "text/id_path" -}}
{{ colorize "→" "blue" .IsTTY }} {{ .ID }} {{ .Path }}
{{- end -}}

{{- define "text/id_only" -}}
{{ colorize "→" "blue" .IsTTY }} item {{ .ID }}
{{- end -}}

{{- define "text/done_success" -}}
{{- colorize "✓" "green" .IsTTY }} done  {{ .Command }}  {{ .Duration -}}
{{- end -}}

{{- define "text/done_failure" -}}
{{- colorize "✗" "red" .IsTTY }} failed {{ .Command }}  {{ .Duration }} (exit: {{ .ExitCode }})
{{- end -}}

{{- define "text/error" -}}
{{ colorize "✗" "red" .IsTTY }} error: {{ .Message }}
{{- end -}}

{{- define "text/error_detail" -}}
{{ .Detail }}
{{- end -}}

{{- define "text/log" -}}
{{ colorize "•" "dim" .IsTTY }} {{ .Message }}
{{- end -}}

{{- define "text/progress_tty" -}}
{{- printf "\r" }}{{ colorize "↻" "blue" true }} {{ .Percent }}% ({{ .Current }}/{{ .Total }}) {{ .Step -}}
{{- end -}}

{{- define "text/progress" -}}
progress: {{ .Percent }}% {{ .Step }}
{{- end -}}

{{- define "text/result_fallback" -}}
{{ colorize "→" "blue" .IsTTY }} {{ .Data }}
{{- end -}}
