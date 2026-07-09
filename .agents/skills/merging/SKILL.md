---
name: merging
description: Expert procedure for merging commits in the monorepo, ensuring standardized commit messages and proper attribution.
---

# Merging Commits

Follow this procedure to merge commits while maintaining monorepo standards.

## 1. Gather Information

1. **Identify Merged Commits**: Use `git log` to find the commits being merged.
2. **Determine Current User**: Get your emoji and alias from `.repo/🧑‍💻/{{user}}/contributor.json`.
3. **Identify Co-authors**: If the merged commits have authors other than yourself, add them as `Co-authored-by`. If you are not the author of the merged commits, ensure you are also listed appropriately.
4. **Identify Tickets**: Find the tickets associated with the changes. Ticket information is located in `.repo/🎫/YY/MM/DD/TICKETSLUG/ticket.json`.
5. **Identify Areas**: Determine the specific areas (bundles/technologies) the commits affect.

## 2. Format Commit Message

The commit message MUST follow this strict scheme:

```
{{dev-emoji}}{{dev-alias}}🎆{{YY}}🌙{{MM}}☀️{{DD}}🔀
🎆{{YY}}🌙{{MM}}☀️{{DD}}
- {{ticket-emoji}} {{ticket-title}}
{{co-authored-by}}
{{signed-off-by}}
```

### Components:

- **Header**: Your emoji and alias followed by the current date and the merge emoji (🔀).
- **Date Line**: The date of the changes being merged. If multiple dates, list them chronologically (newest first).
- **Ticket Line**:
  - Use the appropriate emoji from `dictionary.csv` based on the ticket's domain or content.
  - The title should be the official ticket title from `ticket.json`.
- **Co-authored-by**: `Co-authored-by: Name <email>` for every contributor other than the committer.
- **Signed-off-by**: `Signed-off-by: Name <email>` for the current committer.

## 3. Example

```
🐙ueli🎆26🌙04☀️07🔀
🎆26🌙04☀️07
- 🔌Add Max Children to Port and Type Connector
- 📚Refactor UI Levels from Stories to Storybook Decorator
🎆26🌙04☀️06
- 📊Server Baseline Diffs Studio Adapter E2e Tests
- 🔖Use Mod Instead of Region for Rust Sections
- 🧪Consolidate Server Into Bin Rs With Integration Tests
- ⌛Fix Mcp App Scene and Diagram Without Timeout
🎆26🌙04☀️03
- 💍Fix Type Connector Ring Detail Panel
- 🍩Fix Workbench Piece Addition to Show Correct Type 3D Model in Scene
Co-authored-by: Kinan Sarakbi <kinan.sarak@gmail.com>
Signed-off-by: Ueli Saluz <ueli@semio-tech.com>
```

## 4. Verification

1. Ensure all ticket emojis are accurate according to `dictionary.csv`.
2. Verify all co-authors are included if applicable.
3. Check that the date format uses the specific emojis: 🎆 (Year), 🌙 (Month), ☀️ (Day).
4. Ensure the header ends with the merge emoji 🔀.
