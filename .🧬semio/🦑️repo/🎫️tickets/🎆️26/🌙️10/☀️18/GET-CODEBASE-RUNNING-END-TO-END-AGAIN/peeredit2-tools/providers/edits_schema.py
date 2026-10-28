S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"
EDITS = [
(S, '''    "metadata": {
      "type": "object"
    }
  },
  "$defs": {''', '''    "metadata": {
      "type": "object",
      "properties": {
        "semio": {
          "type": "object",
          "properties": {
            "scriptProviders": {
              "description": "Declares this project the providers project: the file name of the script provider designation in its root. Every script provider closure becomes one of its named inputs per port and view, and every command closure read through a view names it by reference.",
              "type": "string",
              "pattern": "^[^/\\\\\\\\]+\\\\.json$"
            }
          }
        }
      }
    }
  },
  "$defs": {'''),
]
