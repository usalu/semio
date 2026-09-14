$ErrorActionPreference = "Stop"
$dir = "C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\⌨️cli\📦️packages\🐹️go"
$src = Join-Path $dir "🐹️.go"
$lines = [System.IO.File]::ReadAllLines($src)

$imports = @(
  @("bufio", 'bufio "bufio"'),
  @("context", 'context "context"'),
  @("json", 'json "encoding/json"'),
  @("errors", 'errors "errors"'),
  @("fmt", 'fmt "fmt"'),
  @("io", 'io "io"'),
  @("fs", 'fs "io/fs"'),
  @("math", 'math "math"'),
  @("os", 'os "os"'),
  @("exec", 'exec "os/exec"'),
  @("filepath", 'filepath "path/filepath"'),
  @("regexp", 'regexp "regexp"'),
  @("sort", 'sort "sort"'),
  @("strconv", 'strconv "strconv"'),
  @("strings", 'strings "strings"'),
  @("sync", 'sync "sync"'),
  @("template", 'template "text/template"'),
  @("time", 'time "time"'),
  @("utf8", 'utf8 "unicode/utf8"'),
  @("", ""),
  @("codebasepkg", 'codebasepkg "github.com/usalu/semio/repo/codebase"'),
  @("contributorspkg", 'contributorspkg "github.com/usalu/semio/repo/contributors"'),
  @("eventspkg", 'eventspkg "github.com/usalu/semio/repo/events"'),
  @("goalspkg", 'goalspkg "github.com/usalu/semio/repo/goals"'),
  @("graphqlpkg", 'graphqlpkg "github.com/usalu/semio/repo/graphql"'),
  @("hooks", 'hooks "github.com/usalu/semio/repo/hooks"'),
  @("languagespkg", 'languagespkg "github.com/usalu/semio/repo/languages"'),
  @("metricspkg", 'metricspkg "github.com/usalu/semio/repo/metrics"'),
  @("model", 'model "github.com/usalu/semio/repo/model"'),
  @("move", 'move "github.com/usalu/semio/repo/move"'),
  @("providers", 'providers "github.com/usalu/semio/repo/providers"'),
  @("statutes", 'statutes "github.com/usalu/semio/repo/statutes"'),
  @("testrunner", 'testrunner "github.com/usalu/semio/repo/testrunner"'),
  @("ticketspkg", 'ticketspkg "github.com/usalu/semio/repo/tickets"'),
  @("todos", 'todos "github.com/usalu/semio/repo/todos"'),
  @("treepkg", 'treepkg "github.com/usalu/semio/repo/tree"'),
  @("workspace", 'workspace "github.com/usalu/semio/repo/workspace"'),
  @("yamlpkg", 'yamlpkg "github.com/usalu/semio/repo/yaml"')
)

function New-Part {
  param([string]$path, [string]$summary, [string]$region, [int[][]]$ranges)
  $body = New-Object System.Collections.Generic.List[string]
  foreach ($r in $ranges) {
    for ($i = $r[0]; $i -le $r[1]; $i++) { $body.Add($lines[$i - 1]) }
    $body.Add("")
  }
  while ($body.Count -gt 0 -and $body[$body.Count - 1] -eq "") { $body.RemoveAt($body.Count - 1) }
  $text = ($body -join "`n")
  $head = New-Object System.Collections.Generic.List[string]
  $head.Add("// #region 🧲️Header")
  $head.Add("")
  $head.Add("// 2026 Ueli Saluz <ueli@semio-tech.com>")
  $head.Add("")
  $head.Add("// $summary")
  $head.Add("")
  $head.Add("// #endregion 🧲️Header")
  $head.Add("")
  $head.Add("package cli")
  $head.Add("")
  $head.Add("import (")
  $pending = ""
  foreach ($imp in $imports) {
    if ($imp[0] -eq "") { $pending = "blank"; continue }
    if ($text -match ("(?<![A-Za-z0-9_])" + [regex]::Escape($imp[0]) + "\.")) {
      if ($pending -eq "blank") { $head.Add(""); $pending = "" }
      $head.Add("`t" + $imp[1])
    }
  }
  $head.Add(")")
  $head.Add("")
  $head.Add("// #region $region")
  $head.Add("")
  $out = ($head -join "`n") + "`n" + $text + "`n`n// #endregion $region`n"
  [System.IO.File]::WriteAllText((Join-Path $dir $path), $out, (New-Object System.Text.UTF8Encoding $false))
}

New-Part "🖨️render.go" "🖨️render carries the cli stream renderers: NDJSON, human and markdown projections of the engine event stream." "🌩️CLI Renderers" @(, @(3755, 4454))
New-Part "🔌️mcp.go" "🔌️mcp carries the MCP handler surface of the cli: argument and path guards, tool and prompt handlers, resource handlers, descriptions, the server factory and the server adapter." "🔌️Mcp Surface" @(@(5257, 6664), @(7678, 8272), @(8922, 9258))

$drop = New-Object 'System.Collections.Generic.HashSet[int]'
foreach ($r in @(@(3755, 4454), @(5257, 6664), @(7678, 8272), @(8922, 9258))) {
  for ($i = $r[0]; $i -le $r[1]; $i++) { [void]$drop.Add($i) }
}
$kept = New-Object System.Collections.Generic.List[string]
for ($i = 1; $i -le $lines.Length; $i++) { if (-not $drop.Contains($i)) { $kept.Add($lines[$i - 1]) } }
$joined = ($kept -join "`n")
$joined = [regex]::Replace($joined, "(\r?\n){3,}", "`n`n")
[System.IO.File]::WriteAllText($src, $joined.TrimEnd() + "`n", (New-Object System.Text.UTF8Encoding $false))
Write-Output ("kept=" + $kept.Count)
