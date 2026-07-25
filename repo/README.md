---
name: repo
kind: infrastructure
summary: Monorepo tools that are ai-ready.
---

## ✨ Features

- 🤖 AI integrations
  - ✅ Currently
    - 🧑‍✈️ Copilot
    - 🌊 Cascade
    - 🖱️ Cursor Agent
    - ❄️ Claude Code
    - ⚙️ Codex
  - 📅 Possible
    - 🦾 Droid
    - …
- 🧑‍💻 IDE integrations
  - ✅ Currently
    - 💻 VSCode
    - 🌊 Windsurf
    - 🖱️ Cursor
    - Antigravity
    - ⚙️ Codex
    - ❄️ Claude Code
    - 🦾 Droid
  - 📅 Possible
    - InteliJ
    - PyCharm
    - WebStorm
    - Rider
    - Android Studio
    - …
- 🗣️ Language integrations
  - ✅ Currently
    - 🟦 Typescript
    - 🐹 Go
    - 🐍 Python
    - 🦀 Rust
    - 🟣 C#
  - 📅 Possible
    - ♨️ Java
    - 🟪 Kotlin
    - ➕ C++
    - ©️ C
    - 🐘 PHP
    - 💎 Ruby
    - 🐦 Swift
    - …
- 📦 Sandbox integrations
  - ✅ Currently
    - 🐋 Devcontainers
  - 📅 Possible
    - 🦭 Podman
    - …
- 👥 Tracker integrations
  - ✅ Currently
    - 🐙 GitHub
  - 📅 Possible
    - 🦊 GitLab
    - 🪣 Bitbucket
    - …

### 🤖 AI

| System          | Agents | Skills | Hooks |
| --------------- | :----: | :----: | :---: |
| 🧑‍✈️ Copilot      |   ✅   |   ✅   |  ✅   |
| 🌊 Cascade      |   ✅   |   ✅   |  ✅   |
| 🖱️ Cursor Agent |   ✅   |   ✅   |  ✅   |
| ❄️ Claude Code  |   ✅   |   ✅   |  ✅   |
| ⚙️ Codex        |   ✅   |   ✅   |  ❌   |
| (🦾 Droid)      |   ✅   |   ✅   |  ✅   |

###

## 🥇 Why repo is a game changer

### 📈 Requirements + Docs + Stats + Semantics

#### 🚀 Agents love TDD

[Test-Driven-Development (TDD)](https://en.wikipedia.org/wiki/Test-driven_development) is a game changer for agents 🚀

##### ↔️ Multi-lanugage development

Ever had the problem that you domain-expert devs write Python/C++/C/Javascript/Ruby/Lua/… but not Typescript/Go/Rust/C#/Julia/… or vice versa?

No problem, let them write what they know, until the tests are extended and let agents reimplement it natively until the tests succeeds ✅

## 😥 What, you'll have to abandon

### ❌ No granular files, only godfiles

### ❌ No normal docstrings

### ❌ No inline comments

## 😲 What, you'll get

### 🚀 Zero-touch development

### 🧪 Shared test infrastructure

### 💯 Consistent requirements

### 📑 Conistent docs

### 🔮 Future proof infrastructure

### 📊 Meaningful stats

## Configuration

Repo-wide settings live in [`.repo/config.toml`](../.repo/config.toml) at the monorepo root.

| Key                  | Default    | Description                                                                 |
| -------------------- | ---------- | --------------------------------------------------------------------------- |
| `logging.session`    | `false`    | Write per-session `session.json` under `.repo/⚡/🤖/…` on agent hooks       |
| `logging.operations` | `true`     | Append derived `agent.<operation>.<phase>` events (requires `session = true`)      |
| `logging.plan`       | `true`     | Track agent plan steps in `session.json` (requires `session = true`)        |
| `logging.detail`     | `standard` | `minimal` (event only), `standard` (+ response), or `full` (+ native stdin) |

Set `logging.session = true` to enable session-file logging for debugging or coordinator ingestion.

# 📦 Bundles

- [cli](cli/README.md) – Command line tool for monorepo interactions
- [go](go/README.md) – Go shared libraries or server components
- [graphql](graphql/README.md) – GraphQL schema and core typings
- [postgres](postgres/README.md) – PostgreSQL schema and configuration
- [server](server/README.md) – MCP Server and backend services for repo tooling
- [sqlite](sqlite/README.md) – SQLite schema and helpers
- [vscode](vscode/README.md) – Visual Studio Code extension for compose
