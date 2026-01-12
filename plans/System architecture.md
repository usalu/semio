# System Architecture

Welcome! This document explains how the **semio** repository is structured and how its parts work together, written for developers new to the project.

## 🌟 Big Picture

Think of **semio** as a digital LEGO system for buildings.
- **The DNA (Specs)**: We define strict rules for what a "brick" (Piece) is, how they connect (Connection), and how they are grouped (Design).
- **The Engine (Core)**: We have logic to handle these rules in different programming languages.
- **The Tools (Products)**: We build apps (like Sketchpad) and plugins (for CAD software) so people can design with these bricks.

## 🏗️ Main Layers

### 1. The Specs (The DNA)
Everything starts here. We define the schemas (structures) for our data.
- **Where**: `jsonschema/`, `sql/`, `graphql/`.
- **What**: Definitions for `Kit`, `Design`, `Type`, `Piece`, etc.
- **Why**: So that a design created in Python can be opened in C# or JavaScript without breaking.

### 2. The Ecosystems (The implementations)
We implement the "DNA" in three main languages to support different platforms.

#### 🟨 JavaScript (`js/`)
- **Used for**: The Web and Desktop App (Sketchpad).
- **Key Package**: `@semio/js` (in `js/semio/`). Contains the core logic and React UI components.
- **Apps**:
  - `sketchpad`: The main editor (like a 3D Google Docs).
  - `vscode`: Extensions to help developers.

#### 🟪 .NET / C# (`net/`)
- **Used for**: Heavy CAD integrations on Windows.
- **Key Library**: `Semio.cs`.
- **Plugins**:
  - `Semio.Grasshopper`: Connects semio to Rhino/Grasshopper (a popular architecture tool).

#### 🐍 Python (`py/`)
- **Used for**: Backend logic, AI, and Automation.
- **Key Package**: `@semio/engine`.
- **Role**: It generates the schemas and handles complex geometric processing.

#### 🐹 Go (`go/`)
- **Used for**: Developer tools and CLI.
- **Role**: Runs the `repo` command-line tool and handles task tracking (tickets).

### 3. The Products (The User Tools)
These are what the end-user actually sees.
- **Sketchpad** (`js/semio/sketchpad`): The browser-based editor. It uses `@semio/js` to render designs and let users edit them.
- **Grasshopper Plugin** (`net/Semio.Grasshopper`): Lets architects use semio directly inside their CAD software.

## 🔄 How it all connects

1. **Schema Generation**:
   The `py/engine` scripts generate schemas (JSON, SQL, GraphQL) from the definitions.
   ⬇️
2. **Code Generation / Implementation**:
   - `Semio.cs` (C#) implements these schemas.
   - `semio.ts` (JS) implements these schemas.
   ⬇️
3. **App Usage**:
   - **Sketchpad** imports `semio.ts` to build the UI.
   - **Grasshopper** imports `Semio.cs` to create components.

## 📂 Simplified Folder Map

```text
d:\semio
├── assets/       # Icons, fonts, 3D models shared by everyone.
├── go/           # CLI tools for developers (repo management).
├── js/           # All things JavaScript (React, Web, Desktop).
│   └── semio/    # The heart of the JS ecosystem.
├── net/          # All things C# (Rhino, Grasshopper).
├── py/           # All things Python (Schema gen, AI).
├── tickets/      # We track our work here (Tasks, Logs).
├── scripts/      # Little helpers for automation.
└── jsonschema/   # The Single Source of Truth for data structures.
```

## 🧠 Key Relationships

- **Monorepo**: Everything is in one place. If you change a Spec, you update it in `py/`, regenerate schemas, and then update `js/` and `net/` to match.
- **No Database Server**: We often use simple files (JSON, SQLite inside ZIPs) so data is portable.
- **Agents**: We code with AI in mind. The folder structure is flat and predictable to help AI agents (like Copilot) understand the context easily.
