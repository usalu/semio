# Summary

Coda programming bundle for research programming artifacts. Includes a Go validator for programming targets (space program and adjacency matrix validation).

# Specs

- Validator reads translation JSON from stdin, program requirements from `.progam/config.json` or `.coda/programming-requirements.json`, outputs report JSON to stdout.
- Space program: validates area constraints (min/max) per plugin kind from `byKind`.
- Adjacency matrix: validates mandatory adjacencies between program kinds.

# Build

```bash
cd __KEEP_pluginming__/go && GOWORK=off go build -o __KEEP_pluginming__ .
```

Copy the binary to `.coda/validators/__KEEP_pluginming__` in the project root.

# 💯Requirements
