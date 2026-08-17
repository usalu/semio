# 🎫️ Ticket: Enhance Contributor Information and Discovery

## 📝️ Description

The contributors should have more information in `contributor.json`. There should always be the preferred name/email and aliases in plural `names`/`emails`. Implement a contributor discovery mechanism from a string like 'Name <email@example.com>' that updates the contributor if more information is found. Match by email first, then by name.

## 🎫️ Details

- **ID**: `2026/02/09/ENHANCE-CONTRIBUTOR-INFORMATION-AND-DISCOVERY`
- **Goal**: `AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-CONTRIBUTOR-MECHANISM`
- **Author**: `copilot-chat`
- **Client**: `copilot-chat`
- **LLM**: `gemini-3-flash`

## ✅️ Todos

- [ ] Update Contributor model in `cli.go` and `main.go`
- [ ] Update GraphQL schema in `schema.graphql` and `main.go`
- [ ] Implement contributor discovery logic in `cli.go` or `main.go`
- [ ] Update contributor persistence logic
- [ ] Extend `main_test.go` to cover new contributor information and discovery
- [ ] Update `AGENTS.md` and `README.md`
- [ ] Close ticket

## 🪵️ Log

- Started task to enhance contributor information and discovery.
