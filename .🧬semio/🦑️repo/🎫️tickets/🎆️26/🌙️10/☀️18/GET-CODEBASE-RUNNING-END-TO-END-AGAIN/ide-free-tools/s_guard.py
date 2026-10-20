"""Group guard: the ide-census and the dashboard authority forbid .cursor, .windsurf and .github/hooks outright."""
import json
from blib import *

g = "guard"
DC = L + "/🧪️tests/🎛️dashboard-control"
CF = DC + "/🧫️fixtures/🔍️ide-census.json"
ED(g, CF, '    ".kateproject.d"\n  ],\n  "files": [', '    ".kateproject.d",\n    ".cursor",\n    ".windsurf"\n  ],\n  "directoryPaths": [\n    ".github/hooks"\n  ],\n  "files": [')
ED(g, CF, '''      "path": "unknown/ignored/tasks.vs.json",
      "kind": "file",
      "expected": "retired"
    }
  ],
  "manifestCases"''', '''      "path": "unknown/ignored/tasks.vs.json",
      "kind": "file",
      "expected": "retired"
    },
    {
      "path": "unknown/ignored/.cursor",
      "kind": "directory",
      "expected": "retired"
    },
    {
      "path": "unknown/ignored/.cursor/plans",
      "kind": "directory",
      "expected": "retired"
    },
    {
      "path": "unknown/ignored/.windsurf",
      "kind": "directory",
      "expected": "retired"
    },
    {
      "path": "unknown/ignored/.github/hooks",
      "kind": "directory",
      "expected": "retired"
    },
    {
      "path": "unknown/ignored/.github/hooks/repo.json",
      "kind": "file",
      "expected": "retired"
    },
    {
      "path": "unknown/ignored/.github/workflows",
      "kind": "directory",
      "expected": "accepted"
    },
    {
      "path": "unknown/ignored/.github/hooks-notes",
      "kind": "directory",
      "expected": "accepted"
    }
  ],
  "manifestCases"''')
CS = DC + "/🧬️schema/🔍️ide-census.json"
ED(g, CS, '    "directories",\n    "files",', '    "directories",\n    "directoryPaths",\n    "files",')
ED(g, CS, '''    "files": {
      "type": "array",''', '''    "directoryPaths": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "type": "string",
        "minLength": 3
      }
    },
    "files": {
      "type": "array",''')
CT = DC + "/🟦️.ts"
ED(g, CT, "const ideDirectories=new Set(censusFixture.directories),ideFiles=", "const ideDirectories=new Set(censusFixture.directories),ideSequences=censusFixture.directoryPaths,ideFiles=")
ED(g, CT, "  return parts.some(part=>ideDirectories.has(part))||(reference?", "  return parts.some(part=>ideDirectories.has(part))||ideSequences.some(sequence=>(\"/\"+parts.join(\"/\")+\"/\").includes(\"/\"+sequence+\"/\"))||(reference?")
ED(g, CT, "OR EXISTS(SELECT 1 FROM parts WHERE (?2=1 OR (?5<>'directory' AND n=(SELECT max(n) FROM parts)))", "OR EXISTS(SELECT 1 FROM json_each(?8) sequence WHERE instr('/'||lower(\"+(reference?normalized:\"replace(?1,char(92),'/')\")+\")||'/','/'||sequence.value||'/')>0) OR EXISTS(SELECT 1 FROM parts WHERE (?2=1 OR (?5<>'directory' AND n=(SELECT max(n) FROM parts)))")
ED(g, CT, "JSON.stringify(censusFixture.files),JSON.stringify(censusFixture.suffixes))as{disposition:IdeDisposition})", "JSON.stringify(censusFixture.files),JSON.stringify(censusFixture.suffixes),JSON.stringify(censusFixture.directoryPaths))as{disposition:IdeDisposition})")
ED(g, CT, "disposition=retiredIdePath(entry.name,kind);", "disposition=retiredIdePath(path,kind);")

AU = M + "/🎛️dashboard"
AF = AU + "/🧪️tests/🧫️fixtures/🧭️authority/🔣️.json"
ED(g, AF, '    ".atom",\n    ".brackets"\n  ],', '    ".atom",\n    ".brackets",\n    ".cursor"\n  ],')
ED(g, AF, '    ".cursor/rules"\n  ],\n  "pathCases": [\n    {\n      "path": ".cursor",\n      "accepted": true\n    },', '    ".cursor/rules",\n    ".github/hooks"\n  ],\n  "pathCases": [\n    {\n      "path": ".github/hooks",\n      "accepted": false\n    },\n    {\n      "path": ".github/workflows",\n      "accepted": true\n    },\n    {\n      "path": ".cursor",\n      "accepted": false\n    },')
ED(g, AF, '"path": ".cursor/plans/audit.md",\n      "accepted": true', '"path": ".cursor/plans/audit.md",\n      "accepted": false')
ED(g, AF, '"path": ".cursor/plans/mcp.json",\n      "accepted": true', '"path": ".cursor/plans/mcp.json",\n      "accepted": false')
ED(g, AF, '"text": "Consult .cursor/plans/archive.md for historical planning evidence.",\n      "accepted": true', '"text": "Consult .cursor/plans/archive.md for historical planning evidence.",\n      "accepted": false')
AT = AU + "/🧪️tests/🧭️authority/🟦️.ts"
ED(g, AT, ' && !parts.some((part, index) => part === ".cursor" && parts[index + 1] !== undefined && parts[index + 1] !== "plans")', "")
ED(g, AT, "\\.cursor[/\\\\](?!plans(?:[/\\\\]|$))", "\\.cursor[/\\\\]")
AS = AU + "/🧬️schema/🧭️authority/🔣️.json"
ED(g, AS, '"pattern": "\\\\.cursor[/\\\\\\\\](?!plans(?:[/\\\\\\\\]|$))"', '"pattern": "\\\\.cursor[/\\\\\\\\]"')
ED(g, AS, '"pattern": "(^|/)\\\\.cursor/(?!plans(?:/|$))"', '"pattern": "(^|/)\\\\.cursor(/|$)"')
ED(g, AS, "Cursor MCP/hooks/settings/rules have no authority; compiler project files and historical Cursor plans remain valid.", "Cursor directories, .github/hooks and every other client directory have no authority; compiler project files remain valid.")
FM = M + "/🏠️workspace/📦️packages/🐹️go/🎨️formatter/🐹️.go"
ED(g, FM, "	name := parts[len(parts)-1]\n	for _, file := range []string{\".project\"", "	if strings.Contains(\"/\"+strings.Join(parts, \"/\")+\"/\", \"/.github/hooks/\") {\n		return true\n	}\n	name := parts[len(parts)-1]\n	for _, file := range []string{\".project\"")
