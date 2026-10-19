# Framework Source-Only Static Audit

No compiler, typechecker, build, tests, resolver, or application runtime. Parsed current owned non-plugin/non-renderer JSON/TOML and audited relocated output identifiers against B/H/S plus mapped authored source spans.

{
  "parsed": {
    "json": 380,
    "toml": 22
  },
  "syntaxIssues": [
    {
      "path": "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🧬️schema/🔣️.json",
      "error": "Expecting ',' delimiter: line 2 column 1 (char 630)",
      "authored": {
        "stash": "Expecting ',' delimiter: line 2 column 1 (char 630)"
      }
    }
  ],
  "introducedWords": []
}
