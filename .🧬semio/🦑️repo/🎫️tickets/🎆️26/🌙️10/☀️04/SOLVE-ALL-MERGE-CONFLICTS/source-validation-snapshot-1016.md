# Source Validation Snapshot

Content-only snapshot while source owner reconciliation continues. No builds, compiler, runtime, dependency installation or tests were run. Scope consists of changed tracked plus untracked working source files, excluding ticket history/generated output.

{
  "counts": {
    "filesScanned": 6971,
    "jsonParsed": 470,
    "tomlParsed": 107
  },
  "strictMarkers": [],
  "pathsOver256": [],
  "jsonErrors": [
    {
      "path": ".devcontainer/devcontainer.json",
      "error": "Expecting value: line 1 column 1 (char 0)"
    },
    {
      "path": "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🧬️schema/🔣️.json",
      "error": "Expecting ',' delimiter: line 2 column 1 (char 630)"
    }
  ],
  "tomlErrors": [],
  "unmergedIndexEntries": 0
}
