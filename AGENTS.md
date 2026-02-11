You are a senior developer working inside with other senior developers at the same time on the same files in the semio monorepo. The codebase in under design and development and not used in production yet. There are many inconsistencies that you MUST refactor. You MUST use clean mechanisms that might require large refactorings and you MUST NOT care about backwards compatibility at any time. You MUST follow the following rules unless explicitly asked to do otherwise:

- You MUST use semio-repo mcp (or the cli `./semio-repo/cli/cli`) for repo-specific infrastructure.
   - You MUST work inside a ticket.
   - You MUST start by gathering information about the repo with mcp tool `tree` (or `./semio-repo/cli/cli tree <query>`). This includes all prefiltered information you will need later such as relevant projects, bundles, folders, files, sections, definitions, goals, tickets, drafts, policies, violation kinds, contributors, commits.
   - You MUST reopen a ticket with `ticket_reopen` (or `./semio-repo/cli/cli ticket reopen <ticket-id> <prompt> <client> <llm> --draft <draft-id>? --title <new-title?> --goal <new-goal-id>`) if an existing ticket is already covering the same task.
   - If no existing ticket is covering the same task, you MUST create a new ticket with mcp tool `ticket_open` (or `./semio-repo/cli/cli ticket open <goal-id> <title> <prompt> <client> <llm> --draft <draft-id>? --parent <parent-ticket-id>?`). This creates a ticket folder `.semio-repo/tickets/YYYY/MM/DD/TICKETSLUG` along with a ticket file `ticket.md` in it.
   - You MUST add all temporary files, logs, scripts, … inside the ticket folder.
   - You MUST NOT create any additional files outside the ticket folder.
   - You MUST NOT answer directly in the chat and MUST interact by editing the ticket file for everything for everything related to the plan, todos, changes, summary.
   - You MUST use the mcp tool `ticket_close` (or `./semio-repo/cli/cli ticket close <ticket-id> <summary> <files...>`) to finish the ticket along with the summary and at all the files you worked on (created, updated or removed). When a dev sends a new message to the chat YOU MUST reopen the same ticket with mcp tool `ticket_reopen` (or `./semio-repo/cli/cli ticket reopen <ticket-id> <prompt> <client> <llm> --draft <draft-id>? --title <new-title?> --goal <new-goal-id>? --parent <new-parent-ticket-id>?`).
   - You MUST create a goal with mcp tool `goal_open`(or `./semio-repo/cli/cli goal open <title> <description> <prompt> <client> <llm> --due <due-date>? --parent <parent-goal>?`). NEVER create a goal when not excplicly asked to do so. Close a goal with mcp tool `goal_close`(or`./semio-repo/cli/cli goal close <GOALSLUG/SUBGOALSLUG> <summary>`). The due date is a date in the format `YYYY-MM-DD`. Reopen a goal with mcp tool `goal_reopen`(or `./semio-repo/cli/cligoal reopen <GOALSLUG/SUBGOALSLUG> <prompt> <client> <llm> --title <new-title>? --description <new-description>? --due <new-due-date>? --parent <new-parent-goal>?`).
   - A ticket id is `YYYY/MM/DD/TICKETSLUG`.
   - A goal id is `GOALSLUG/SUBGOALSLUG/...`.
   - A title MUST be titleized (e.g. "Some Title on Something") and MUST NOT be a slug or all caps.
   - Available LLMs are: `opus-4-6`, `opus-4-5`, `sonnet-5`, `sonnet-4-5`, `haiku-4-5`, `gemini-3-pro`, `gemini-3-flash`, `gpt-5-3-codex`, `gpt-5-2-codex`, `swe-1-5`, `gpt-5-mini`. Available Clients are: `copilot-chat`, `windsurf-chat`, `cursor-chat`, `antigravity-chat`, `claude-code`, `codex`, `droid`.
- You MUST NOT use `git stash`, `git stash pop`, `git checkout`, … because others will lose their work.
- You MUST document every key decision and mechanism.
   - You MUST document every specification decision called `specs` under the following locations:
      - **Project-level**: `README.md` at the project root (under `# Specs`)
      - **Bundle-level**: `README.md` at the bundle root (under `# Specs`)
      - **Folder-level**: `README.md` in the folder (under `# Specs`)
      - **File-level**: Header `Specs` region
      - **Section-level**: Comments after section start markers
      - **Definition-level**: Language-native docstrings

- ALWAYS finish everything without asking in between.
- NEVER interrupt between TODOs or tickets.
- NEVER remove functionality. Not even to get the code to work quickly.
- ALWAYS be thorough.
- NEVER create scripts to automate manual tasks.
- NEVER leave a placeholder.
- NEVER stop halfways and ask if you should continue.
- If a task is too big, ALWAYS start with one small part and ALWAYS finish it and keep on as much as you can.
- ALWAYS finish the task.
- ALWAYS make the choice directly! If you have several options, don't ask in between, be opionionated and just go for it. Try to do as much as you can.
- ALWAYS toolfriendly over intuitive.
- ALWAYS expose the canonical CI/CD scripts `dev`, `build`, `test`, `update`, `prepublish`, and `publish` only at the root (which forwards them through `npx nx run-many -t <target>`). Do not add missing commands to workspace packages; keep only the scripts they already define, treat `dev` as the only long-running watch mode, and make sure the remaining commands exit so CI runners and agents can finish reliably.
- When multiple long-running dev processes exist for a single workspace, use hierarchical naming for VS Code tasks/launch configs (e.g. `dev js js storybook`, `dev js js sketchpad`) and use `dev:<...>` for root `package.json` scripts when spaces are not possible.
- NEVER create new files when not explicitly asked. ALWAYS add code to existing files using regions and subregions for structuring. Regions organize code into collapsible sections (e.g., `#region 🔖RegionName` / `#endregion` in C#, or `//#region 🔖RegionName` / `//#endregion` in JavaScript/TypeScript). Use subregions within regions for hierarchical organization. This keeps related code together and maintains a single source of truth per logical unit.
- NEVER create new folders unless required by the ticket workflow; temporary data belongs in the active ticket folder.
- NEVER create additional example files and implement it directly in the dependent parts.
- NEVER remove code that is commented out.
- NEVER add comments to the code. Especially not to communicate to the user.
- NEVER ask to run a command where you are not using the output. All dev servers, debugging and testing processes are running.
- NEVER run modifying `git` commands such as (`git checkout`, `git branch`, `git stash`, …) because there are other are ALWAYS agents/processes/devs working on the same set of files at the same time. Only read-only `git` commands are allowed. If you messed up, ALWAYS fix the file.
- NEVER create tests unless you are explicitly asked to.
- ALWAYS use inline syntax if possible.
- NEVER add two statements into the same line.
- ALWAYS inline code.
- NEVER create a variable, function, … class, that is only used once and inline it.
- NEVER add extra new blank lines/newlines inside of code.
- NEVER add raw text to client elements. ALWAYS use i18n setups and provide translations for the existing languages.
- ALWAYS add `[DEBUG] ` prefix to temporary logs so that they can be easily removed later.
- Keep Sketchpad runtime console output clean: avoid persistent `console.log` usage and rely on warnings/errors plus removable `[DEBUG]` diagnostics only when investigating.
- NEVER care about backwards compatibility unless explicitly asked to. Even on schema changes ALWAYS refactor to clean code and introduce breaking changes.
- NEVER use `type` for naming enums, ports, or types. ALWAYS use `kind` instead to avoid confusion with the native `type` concept in Semio. Examples: `ArtifactType` → `ArtifactKind`, `WindowType` → `WindowKind`, etc.
- When fixing problems, ALWAYS update the existing file and NEVER create new fixed, updated, migrated, etc. files next to the old one.
- NEVER change (e.g. simplify/remove functionality) or skip any test to pass. ALWAYS adjust implementation to pass the tests.
- NEVER create additional scripts, tests, fixtures, assets, …
- NEVER create scripts outside the folder of the current ticket. Not even when debugging or diagnosing a library problem.
- ALWAYS create temporary scripts, tests, fixtures, assets, … inside the active ticket folder.
- ALWAYS run specific tests and NEVER use default interactive test mode that creates a never ending process.
- NEVER say that a test is passing when you didn't run it. ALWAYS run the test and check the report.


Extend/Change/Refactor the existing test file to cover everything. Dont create any new test files. A single tests should always cover one unit and do multiple tests for that unit.
Make sure all tests pass.

Extend/Change/Refactor whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out. Create as many tickets as needed.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to track everything (plan, todos, changes, summary, etc) in `.semio-repo/tickets/YYYY/MM/DD/TICKETSLUG*/ticket.md`
Dont keep any legacy api or backwards compatiblity.