# Summary

Refactored the ticket mechanism with the following changes:

1. **New file structure**: Tickets now create `ticket.json`, `plan.md`, `log.md`, `summary.md` instead of a single `ticket.md` with frontmatter
2. **Title-based naming**: Ticket folder name is derived from the title (capitalized slug)
3. **Flexible LLM field**: The `llm` field is now a free string that gets slugified
4. **Plan file support**: Optional `--plan` flag to move an existing markdown file to `plan.md`
5. **Checkpoint section metrics**: Checkpoints compute affected sections from git diffs, with definitions listed under their parent sections
