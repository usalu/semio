# Prompt Strategy

> "Prompting is the new developing. In the old world devs should always write literate code, in the new world they should write literate prompts."

## Philosophy

Docs shouldn't be part of the code anymore (but instead inside `AGENTS.md` and `README.md`). As such prompts (and the process and the output) should be first class citizen in the source code. Similar to how both `package.json` and `package-lock.json` are checked into the repository.

- **Literate Prompts**: Prompts should be written with the same care as code.
- **First-Class Citizens**: Prompts, process logs, and outputs are part of the source code.
- **Persistence**: Condensed ticket information is not thrown away but augmented to create changelogs and stats.
- **Continuity**: The system addresses the difficulty agents have in continuing work across context windows or sessions.

## Ticket System Strategy

The ticket system (`tickets/`) is the implementation of this strategy. It treats development tasks as persistent artifacts with a lifecycle.

- **Structure**: Every task has a dedicated folder `tickets/YYYY/MM/DD/SLUG/` containing:
  - `ticket.md`: The definition of the task (YAML frontmatter + content).
  - `plan.md`: The agent's plan (never sent to chat, always written to file).
  - `log.md`: The execution log (chat messages are added here).
  - `summary.md`: The final result description.
  - `FILES...`: Any temporary artifacts created during the task.

- **Workflow**:
  1. **Open**: `repo ticket open` creates the structure.
  2. **Plan**: Agent writes `plan.md`.
  3. **Log**: Agent logs progress in `log.md`.
  4. **Progress/Checkpoint**: Whenever a logical step is complete (todo done), a checkpoint is created.
     - Checkpoints capture git diffs (files, lines added/removed).
     - Checkpoints compute metrics for affected sections/definitions.
  5. **Close**: `repo ticket close` finalizes the task with a summary and final stats.

## Agent Guidelines

(Extracted from `AGENTS.md`)

- **Always Open Tickets**: Never start work without an open ticket.
- **File-Based Communication**:
  - NEVER answer directly in the chat.
  - ALWAYS add messages to `log.md`.
  - ALWAYS write plans to `plan.md`.
  - ALWAYS write summaries to `summary.md`.
- **Atomic Work**:
  - Multiple agents/devs work on the codebase simultaneously.
  - NO `git stash` / `git checkout`.
  - Temporary artifacts belong in the active ticket folder.

## Code Generation Strategy

Specific prompting strategies derived from the repository's coding principles (`README.md`).

### Context Management
- **One File**: Encourage the agent to implement solutions in a single file when possible.
  - *Prompt*: "Implement the solution within `path/to/file.ts`. Do not create new files unless absolutely necessary."
- **Upside Down**: Ask the agent to define dependencies before usage (bottom-up in file, but prompt flow is top-down).
  - *Context*: LLMs predict better when definitions precede usage in the context window, even if humans read top-down.
  - *Strategy*: "Define helper functions and classes first, then the main logic utilizing them at the bottom of the file."

### Refactoring & Bug Fixing
- **Preserve Context**: When refactoring, create a `*.old` file copy.
  - *Prompt*: "I created `file.ts.old` with the current state. Refactor `file.ts` to..."
- **Neutral Diagnosis**: When debugging, describe the *symptom*, not the *suspected cause*.
  - *Prompt*: "The output is X, but I expected Y. Here are the logs. Diagnose the issue." (Avoid: "I think the loop is wrong.")
- **Atomic Diagnosis**: Ask for logs/diagnosis *before* asking for a fix.
  - *Prompt 1*: "Add detailed logging to `method` to trace variable `X`."
  - *Prompt 2*: "Here are the logs <logs>. Analyze the root cause."
  - *Prompt 3*: "Implement the fix."

### Stylistic Constraints
- **Inlining**: Explicitly ask to inline one-off functions or components.
  - *Prompt*: "Inline specific sub-components if they are only used once."
- **No Comments**: Instruct to minimize comments.
  - *Prompt*: "Do not add comments explaining the code. Only use code unless there is a licensing header."
- **Regions**: Enforce region usage for structure.
  - *Prompt*: "Wrap the code in `#region RegionName` ... `#endregion` blocks."
