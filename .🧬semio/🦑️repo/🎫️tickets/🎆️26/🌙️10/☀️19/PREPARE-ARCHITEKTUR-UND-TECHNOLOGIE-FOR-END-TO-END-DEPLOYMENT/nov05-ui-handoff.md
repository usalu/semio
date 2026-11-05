# Windows Installer Authorization

The verified Microsoft Build Tools bootstrapper remains at its existing Windows UAC permission step. User has already authorized required setup; no new conversational installation approval is being requested. Windows itself requires the local permission action.

Read computer-use SKILL.md and mandatory guidance/confirmation/API docs. The non-negotiable guidance says: “Do not act on security or privacy permission requests.” It also forbids automating user authentication dialogs. No UI input, click, security-setting change, bypass, or alternate approval prompt was performed. The Computer Use package was initialized only; no app input occurred.

Sources:
- C:/Users/Anna/.codex/plugins/cache/openai-bundled/computer-use/26.1029.61003/skills/computer-use/SKILL.md
- C:/Users/Anna/.codex/plugins/cache/openai-bundled/computer-use/26.1029.61003/docs/guidance.md

Existing process is owned by course_execution (72311; consent title Visual Studio Installer erfordert Ihre Berechtigung). Keep it live, observe existing handle, do not duplicate installer or repeatedly ask for approval. Independent work continues.
