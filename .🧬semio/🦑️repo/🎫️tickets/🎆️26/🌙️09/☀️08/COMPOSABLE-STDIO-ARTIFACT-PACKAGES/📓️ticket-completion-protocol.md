# Ticket Completion Protocol

Read-only MCP tool schema capture; no ticket or goal mutation was made. Final closure must wait for native acceptance and generated-output cleanup. The final ticket goal association is `AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-PROJECT-MECHANISM`.

## ticket_close

```json
{
  "type": "object",
  "properties": {
    "files": {
      "type": "array",
      "description": "Changed files.",
      "items": {
        "type": "string"
      }
    },
    "no_management": {
      "type": "boolean",
      "description": "Skip management integration."
    },
    "path": {
      "type": "string",
      "description": "YY/MM/DD/SLUG path."
    },
    "summary": {
      "type": "string",
      "description": "Completion summary."
    },
    "title": {
      "type": "string",
      "description": "Updated title."
    }
  },
  "required": [
    "summary"
  ],
  "additionalProperties": false
}
```

## ticket_reopen

```json
{
  "type": "object",
  "properties": {
    "client": {
      "type": "string",
      "description": "Agent client."
    },
    "draft": {
      "type": "string",
      "description": "Draft id."
    },
    "effort": {
      "type": "string",
      "description": "Reasoning effort."
    },
    "goal": {
      "type": "string",
      "description": "Goal id."
    },
    "llm": {
      "type": "string",
      "description": "Model."
    },
    "no_management": {
      "type": "boolean",
      "description": "Skip management integration."
    },
    "parent": {
      "type": "string",
      "description": "Parent ticket."
    },
    "path": {
      "type": "string",
      "description": "YY/MM/DD/SLUG path."
    },
    "plan_id": {
      "type": "string",
      "description": "Client plan id."
    },
    "prompt": {
      "type": "string",
      "description": "Additional task description."
    },
    "title": {
      "type": "string",
      "description": "Updated title."
    }
  },
  "additionalProperties": false
}
```


## Current Ledger Candidate

Refreshed a provisional source-path union from all five exact ownership ledgers. This is not the final close request: active repairs can add paths, so it must be regenerated after acceptance. Missing historical paths remain listed as touches only; no deletion attribution is inferred from absence. The Nx ledger is consumed only from its explicit Ticket Close Files section. Ticket Markdown reports and the retained PDF consumer inputs are included separately. Raw candidate: `🗑️generated/ticket-close-candidate-current.json`.

- `📓️artifact-execution-file-ledger.md`: 713 unique source/report path entries.
- `📓️nx-owned-file-ledger.md`: 564 unique source/report path entries.
- `📓️registry-execution-owned-files.md`: 859 unique source/report path entries.
- `📓️coordinator-owned-files.md`: 661 unique source/report path entries.
- `📓️brep-tolerance-touched-path-ledger.md`: 112 unique source/report path entries.

Union: 2805 source paths, 609 currently absent historical paths, 69 ticket Markdown reports, 2 retained consumer inputs.
