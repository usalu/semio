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

