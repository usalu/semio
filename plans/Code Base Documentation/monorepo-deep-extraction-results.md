# Semio Monorepo Deep Extraction Report

> **Analysis Date**: January 12, 2026  
> **Repository**: semio (~75,000 LOC)  
> **Domain**: Design-Information-Modeling for Kit-of-Parts Architecture

---

## 1️⃣ Executive Summary

**semio** is a sophisticated multi-language monorepo for parametric/generative architectural design using a "Kit-of-Parts" paradigm. It provides tools for architects and designers to create, manage, and collaborate on modular design systems.

### Core Business Concept

A **Kit** contains reusable **Types** (building blocks with 3D models and connectors) and **Designs** (compositions of **Pieces** linked by **Connections**). The system enables:

- Hierarchical design composition (designs can contain design-pieces)
- Graph-based design representation
- Multi-platform design authoring (web, desktop, Grasshopper/Rhino)
- Real-time collaboration via Y.js CRDT

---

## 2️⃣ C4 Architecture View

### Context Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              SEMIO ECOSYSTEM                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │  Architect  │    │  Designer   │    │  Engineer   │    │ Developer   │  │
│  │   (User)    │    │   (User)    │    │   (User)    │    │   (User)    │  │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘  │
│         │                  │                  │                  │          │
│         ▼                  ▼                  ▼                  ▼          │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        SEMIO PLATFORM                                │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────┐  │   │
│  │  │Sketchpad │ │ Desktop  │ │  Rhino/  │ │  VS Code │ │   Docs    │  │   │
│  │  │  (Web)   │ │(Electron)│ │Grasshppr │ │Extension │ │  (Astro)  │  │   │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └───────────┘  │   │
│  │       │            │            │            │                       │   │
│  │       ▼            ▼            ▼            ▼                       │   │
│  │  ┌─────────────────────────────────────────────────────────────┐    │   │
│  │  │                    SEMIO CORE DOMAIN                         │    │   │
│  │  │  Kit • Type • Design • Piece • Connection • Connector        │    │   │
│  │  │  Quality • Interface • Model • File • Author • Attribute     │    │   │
│  │  └──────────────────────────────────────────────────────────────┘    │   │
│  │                              │                                       │   │
│  │                              ▼                                       │   │
│  │  ┌─────────────────────────────────────────────────────────────┐    │   │
│  │  │                      STORAGE LAYER                           │    │   │
│  │  │     SQLite (.semio/kit.db)  │  IndexedDB  │  Y.js CRDT       │    │   │
│  │  └──────────────────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│         ┌─────────────────┐     ┌─────────────────┐                        │
│         │  Python Engine  │     │   Go Repo CLI   │                        │
│         │  (FastAPI/MCP)  │     │   (MCP Server)  │                        │
│         └────────┬────────┘     └────────┬────────┘                        │
│                  │                       │                                  │
│                  ▼                       ▼                                  │
│         ┌─────────────────────────────────────────────────────────────┐    │
│         │                  EXTERNAL SYSTEMS                            │    │
│         │   OpenAI  •  Liveblocks  •  GitHub  •  Speckle  •  Ladybug   │    │
│         └─────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Container Diagram

| Container              | Technology        | Responsibility                                     |
| ---------------------- | ----------------- | -------------------------------------------------- |
| **@semio/js**          | TypeScript/React  | Core domain models, UI components, Sketchpad app   |
| **@semio/desktop**     | Electron/Forge    | Native desktop wrapper for Sketchpad               |
| **@semio/docs**        | Astro/Starlight   | User documentation site                            |
| **@semio-repo/vscode** | VS Code Extension | Developer tooling, kit validation, repo management |
| **@semio/engine**      | Python/FastAPI    | Backend services, GraphQL API, AI integration      |
| **@semio/net**         | C#/.NET           | Core library for Rhino/Grasshopper                 |
| **@semio/grasshopper** | C#/Grasshopper    | Visual programming plugin for Rhino                |
| **@semio/repo**        | Go                | CLI for monorepo management                        |
| **@semio/mcp**         | Go/MCP            | Model Context Protocol server for AI agents        |

---

## 3️⃣ Domain Architecture

### Core Domain: Kit-of-Parts Design

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           KIT (Root Aggregate)                          │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │ guid, name, version, description, license, homepage, remote        │ │
│  │ icon, image, concepts, attributes                                   │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                         │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐                │
│  │    TYPES     │   │   DESIGNS    │   │  QUALITIES   │                │
│  │   [1..256]   │   │   [1..128]   │   │   [1..1024]  │                │
│  └──────┬───────┘   └──────┬───────┘   └──────────────┘                │
│         │                  │                                            │
│         ▼                  ▼                                            │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐                │
│  │  INTERFACES  │   │    FILES     │   │   FOLDERS    │                │
│  │  (Ports)     │   │              │   │              │                │
│  └──────────────┘   └──────────────┘   └──────────────┘                │
│                                                                         │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐                │
│  │    TAGS      │   │   CONCEPTS   │   │   AUTHORS    │                │
│  └──────────────┘   └──────────────┘   └──────────────┘                │
└─────────────────────────────────────────────────────────────────────────┘

TYPE ENTITY                          DESIGN ENTITY
┌─────────────────────────┐          ┌─────────────────────────┐
│ • guid, name, variant   │          │ • guid, name, variant   │
│ • parent (TypeId)       │          │ • parent (DesignId)     │
│ • isVirtual, canScale   │          │ • view, canScale        │
│ • canMirror, unit       │          │ • canMirror, unit       │
│                         │          │                         │
│ ┌─────────────────────┐ │          │ ┌─────────────────────┐ │
│ │ MODELS [1..32]      │ │          │ │ PIECES [1..512]     │ │
│ │ • file, tags, name  │ │          │ │ • type | design     │ │
│ └─────────────────────┘ │          │ │ • plane, center     │ │
│                         │          │ │ • scale, color      │ │
│ ┌─────────────────────┐ │          │ └─────────────────────┘ │
│ │ CONNECTORS [1..32]  │ │          │                         │
│ │ • point, direction  │ │          │ ┌─────────────────────┐ │
│ │ • interface, t      │ │          │ │ CONNECTIONS         │ │
│ │ • mandatory         │ │          │ │ • connected side    │ │
│ └─────────────────────┘ │          │ │ • connecting side   │ │
│                         │          │ │ • gap, shift, rise  │ │
│ ┌─────────────────────┐ │          │ │ • rotation, turn    │ │
│ │ PROPS               │ │          │ └─────────────────────┘ │
│ │ • quality → value   │ │          │                         │
│ └─────────────────────┘ │          │ ┌─────────────────────┐ │
└─────────────────────────┘          │ │ LAYERS, GROUPS      │ │
                                      │ │ STATS               │ │
                                      │ └─────────────────────┘ │
                                      └─────────────────────────┘
```

### Domain Bounded Contexts

| Context                | Responsibility                      | Key Entities                            |
| ---------------------- | ----------------------------------- | --------------------------------------- |
| **Kit Management**     | Kit CRUD, versioning, import/export | Kit, Author, File, Folder               |
| **Type Definition**    | Reusable component definition       | Type, Connector, Model, Prop            |
| **Design Composition** | Assembling pieces into designs      | Design, Piece, Connection, Layer, Group |
| **Quality System**     | Measurement and benchmarking        | Quality, Benchmark, Stat, Prop          |
| **Interface System**   | Connector compatibility rules       | Interface, compatible ports             |
| **Validation**         | Constraint checking and fixes       | Constraints, Problems, Fixes            |

### Supporting Domains

| Domain             | Purpose                                | Implementation            |
| ------------------ | -------------------------------------- | ------------------------- |
| **Repo Tooling**   | Monorepo management, tickets, policies | Go CLI, VS Code Extension |
| **Documentation**  | User guides, tutorials                 | Astro + MDX               |
| **AI Integration** | LLM-assisted design                    | MCP Server, OpenAI        |

---

## 4️⃣ Service Map

### @semio/js (Core Domain Library)

| Module                    | Purpose                                | APIs                         | Data                           | Dependencies             |
| ------------------------- | -------------------------------------- | ---------------------------- | ------------------------------ | ------------------------ |
| `semio.ts`                | Domain models, diff system, validation | All entity types, schemas    | Kit, Type, Design, Piece, etc. | Three.js, Zod, Cytoscape |
| `sketchpad/Sketchpad.tsx` | Main app shell, state machine          | useSketchpadActor, stores    | SketchpadState, KitStore       | XState, Y.js, React      |
| `sketchpad/Design.tsx`    | Design editor app                      | DesignAppStore, hooks        | DesignAppState                 | React Three Fiber        |
| `sketchpad/Type.tsx`      | Type editor app                        | TypeAppStore, hooks          | TypeAppState                   | React Three Fiber        |
| `sketchpad/Kit.tsx`       | Kit management app                     | KitAppStore, hooks           | KitAppState                    | Golden Layout            |
| `sketchpad/Home.tsx`      | Kit browser                            | HomeStore, hooks             | HomeState                      | -                        |
| `sketchpad/Quality.tsx`   | Quality editor                         | QualityAppStore              | QualityAppState                | -                        |
| `sketchpad/Docs.tsx`      | In-app documentation                   | MDX loading                  | Headings, sections             | MDX                      |
| `sketchpad/elements.tsx`  | UI primitives                          | Navbar, Footer, Window, etc. | Level, Transaction             | Radix UI, React Flow     |
| `sketchpad/shared.ts`     | Shared types, registries               | AppPlugin, event handlers    | Enums, interfaces              | -                        |

### @semio/engine (Python Backend)

| Module      | Purpose              | APIs               | Data                    | Dependencies                |
| ----------- | -------------------- | ------------------ | ----------------------- | --------------------------- |
| `engine.py` | Full backend service | REST, GraphQL, MCP | Kit entities via SQLite | FastAPI, SQLModel, Graphene |

### @semio/net (C# Core)

| Module     | Purpose                   | APIs             | Data             | Dependencies                |
| ---------- | ------------------------- | ---------------- | ---------------- | --------------------------- |
| `Semio.cs` | Domain models, validation | All entity types | Same as semio.ts | FluentValidation, QuikGraph |

### @semio/grasshopper (Rhino Plugin)

| Module                 | Purpose                | APIs                 | Data         | Dependencies   |
| ---------------------- | ---------------------- | -------------------- | ------------ | -------------- |
| `Semio.Grasshopper.cs` | Grasshopper components | Component parameters | Kit entities | Rhino.Geometry |

### @semio/repo (Go CLI)

| Module    | Purpose             | APIs                  | Data                            | Dependencies       |
| --------- | ------------------- | --------------------- | ------------------------------- | ------------------ |
| `repo.go` | Monorepo management | CLI commands, GraphQL | Tickets, policies, contributors | graphql-go, SQLite |

### @semio/mcp (MCP Server)

| Module    | Purpose        | APIs                 | Data            | Dependencies |
| --------- | -------------- | -------------------- | --------------- | ------------ |
| `main.go` | AI agent tools | MCP tool definitions | Proxies to repo | mcp-go       |

---

## 5️⃣ Dependency Graph

### Language/Platform Matrix

```
                    ┌──────────────────────────────────────────────────────┐
                    │               CROSS-PLATFORM SCHEMA                   │
                    │                                                       │
                    │  jsonschema/kit.json  ←→  sql/sqlite/semio/schema.sql│
                    │         ↑                        ↑                    │
                    │         │                        │                    │
                    └─────────┼────────────────────────┼────────────────────┘
                              │                        │
        ┌─────────────────────┼────────────────────────┼─────────────────────┐
        │                     │                        │                     │
        ▼                     ▼                        ▼                     ▼
┌──────────────┐    ┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│ TypeScript   │    │    Python    │      │     C#       │      │     Go       │
│  @semio/js   │    │@semio/engine │      │  @semio/net  │      │ @semio/repo  │
│              │    │              │      │              │      │              │
│ semio.ts     │←──►│  engine.py   │←────►│  Semio.cs    │      │   repo.go    │
│ (7,741 LOC)  │    │ (7,727 LOC)  │      │ (5,734 LOC)  │      │ (10,110 LOC) │
└──────┬───────┘    └──────────────┘      └──────┬───────┘      └──────────────┘
       │                                         │
       ▼                                         ▼
┌──────────────┐                         ┌──────────────┐
│  Sketchpad   │                         │ Grasshopper  │
│   (React)    │                         │   Plugin     │
│  15,835 LOC  │                         │ 10,000+ LOC  │
└──────────────┘                         └──────────────┘
```

### Internal Dependency Hierarchy

```
Level 0 (Foundation):
  @semio/assets ← shared icons, fonts, models

Level 1 (Core):
  @semio/js/semio ← domain models, validation, diff system

Level 2 (UI):
  @semio/js/sketchpad ← depends on semio
  @semio/js/elements ← depends on semio

Level 3 (Apps):
  @semio/desktop ← depends on @semio/js
  @semio/docs ← depends on @semio/js
  @semio/play ← depends on @semio/js
  @semio-repo/vscode ← depends on @semio/js

Level 4 (Backend):
  @semio/engine ← independent Python implementation
  @semio/net ← independent C# implementation
  @semio/grasshopper ← depends on @semio/net

Level 5 (Tooling):
  @semio/repo ← Go CLI for monorepo
  @semio/mcp ← wraps @semio/repo
```

---

## 6️⃣ Data Architecture

### Data Stores

| Store            | Technology     | Contents                | Read By          | Written By   |
| ---------------- | -------------- | ----------------------- | ---------------- | ------------ |
| **Kit SQLite**   | SQLite in .zip | Kit entities, files     | Engine, .NET, JS | Engine, .NET |
| **Kit Y.js Doc** | Y.js CRDT      | Live kit state          | Sketchpad        | Sketchpad    |
| **IndexedDB**    | Browser DB     | Persisted Y.js docs     | Sketchpad        | Sketchpad    |
| **Repo SQLite**  | SQLite         | Ticket cache            | Go CLI           | Go CLI       |
| **File System**  | Disk           | .semio folders, tickets | All              | All          |

### Data Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                            USER ACTIONS                                  │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         XState State Machine                             │
│                                                                          │
│  Events: SET_THEME, NAVIGATE, DESIGN.SET_HOVER, KIT.CREATE_TYPE, etc.   │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │
                    ┌────────────┼────────────┐
                    │            │            │
                    ▼            ▼            ▼
            ┌───────────┐ ┌───────────┐ ┌───────────┐
            │SketchpadUI│ │  KitStore │ │AppStores  │
            │  (React)  │ │  (Y.js)   │ │ (XState)  │
            └───────────┘ └─────┬─────┘ └───────────┘
                                │
                                ▼
                        ┌───────────────┐
                        │ KitDiff       │
                        │ (immutable)   │
                        └───────┬───────┘
                                │
                    ┌───────────┼───────────┐
                    │           │           │
                    ▼           ▼           ▼
            ┌───────────┐ ┌───────────┐ ┌───────────┐
            │IndexedDB  │ │ Remote    │ │ Export    │
            │Persistence│ │ Provider  │ │ .zip Kit  │
            └───────────┘ └───────────┘ └───────────┘
```

---

## 7️⃣ Critical Flows

### 1. Kit Loading Flow

```
User opens kit
      │
      ▼
┌─────────────────────────────────┐
│ 1. Check kit kind (temp/local/  │
│    remote)                      │
├─────────────────────────────────┤
│ 2. Create KitStore with Y.Doc   │
├─────────────────────────────────┤
│ 3. If .zip: importKit() →       │
│    populate Y.Doc from SQLite   │
├─────────────────────────────────┤
│ 4. Setup IndexeddbPersistence   │
├─────────────────────────────────┤
│ 5. Navigate to kit route        │
└─────────────────────────────────┘
```

### 2. Design Edit Flow (Piece Creation)

```
User drags type to canvas
      │
      ▼
┌─────────────────────────────────┐
│ 1. DesignAppStore.startTransact │
├─────────────────────────────────┤
│ 2. executeCommand("createPiece",│
│    origin, typeGuid, position)  │
├─────────────────────────────────┤
│ 3. Compute PieceDiff, KitDiff   │
├─────────────────────────────────┤
│ 4. Record Edit for undo/redo    │
├─────────────────────────────────┤
│ 5. Apply diff to Y.Doc          │
├─────────────────────────────────┤
│ 6. finalizeTransaction          │
└─────────────────────────────────┘
```

### 3. Kit Validation Flow

```
Kit modified
      │
      ▼
┌─────────────────────────────────┐
│ 1. buildValidationContext(kit)  │
├─────────────────────────────────┤
│ 2. Run all constraints          │
│    • GUID uniqueness            │
│    • Name uniqueness (scoped)   │
│    • Layer path uniqueness      │
│    • etc.                       │
├─────────────────────────────────┤
│ 3. Generate Problems with Fixes │
├─────────────────────────────────┤
│ 4. Fixes are KitDiff objects    │
├─────────────────────────────────┤
│ 5. User can apply fix or ignore │
└─────────────────────────────────┘
```

### 4. Ticket Workflow (Development)

```
Developer starts task
      │
      ▼
┌─────────────────────────────────┐
│ 1. repo ticket open SLUG        │
│    Creates tickets/YYYY/MM/DD/  │
│    SLUG/ticket.md               │
├─────────────────────────────────┤
│ 2. Agent writes plan.md         │
├─────────────────────────────────┤
│ 3. Agent logs to log.md         │
├─────────────────────────────────┤
│ 4. On complete:                 │
│    repo ticket close ... --files│
├─────────────────────────────────┤
│ 5. Git diff computed for stats  │
└─────────────────────────────────┘
```

---

## 8️⃣ Dependency Hotspots & Risks

### God Modules

| File                               | LOC    | Risk Assessment                                                                                                |
| ---------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------- |
| `js/semio/sketchpad/Sketchpad.tsx` | 15,835 | **HIGH** - Contains Store base classes, state machine, all app stores, kit management. Single-file complexity. |
| `go/repo/repo.go`                  | 10,110 | **HIGH** - Entire repo CLI in one file. GraphQL, policies, tickets, sections all together.                     |
| `py/engine/engine.py`              | 7,727  | **MEDIUM** - Full backend in one file, but Python handles this better.                                         |
| `js/semio/semio.ts`                | 7,741  | **MEDIUM** - Core domain, but well-organized with regions.                                                     |
| `net/Semio/Semio.cs`               | 5,734  | **MEDIUM** - C# core, mirrors semio.ts closely.                                                                |

### Tight Coupling Points

1. **Y.js ↔ XState**: Bidirectional sync between Y.js stores and XState machine context requires careful coordination.

2. **Cross-language schema**: Kit schema must stay synchronized across TypeScript, Python, C#, and SQL. Currently done manually with tests.

3. **Diff System**: The diff/inverse/merge/apply pattern is duplicated across 20+ entity types. Any schema change requires updating all.

4. **App Plugin System**: While extensible, the event handler registry (`registerEventHandler`) creates runtime coupling.

### Architectural Violations

1. **Missing abstractions**: `FileProvider` interface exists but implementations are scattered.

2. **Inconsistent state access**: Some components use hooks, others access stores directly.

3. **Translation bypass**: Some UI still uses raw strings instead of i18n keys.

---

## 9️⃣ Change Impact Map

### "What breaks if X changes?"

| Change                      | Impact Scope                                                      | Risk         |
| --------------------------- | ----------------------------------------------------------------- | ------------ |
| **Kit schema change**       | semio.ts, engine.py, Semio.cs, SQL schema, JSON schema, all tests | **CRITICAL** |
| **Y.js document structure** | KitStore, all app stores, persistence                             | **HIGH**     |
| **XState machine events**   | All app hooks, event handlers                                     | **HIGH**     |
| **Element component props** | All Sketchpad apps using elements                                 | **MEDIUM**   |
| **Repo GraphQL schema**     | VS Code extension, MCP server                                     | **MEDIUM**   |
| **Diff function signature** | All applyDiff call sites                                          | **MEDIUM**   |
| **i18n key structure**      | All translated UI, validation scripts                             | **LOW**      |

---

## 🔟 Key Insights & Recommendations

### What This System Really Is

1. **A design version control system** - Like Git but for parametric architectural designs with semantic diff/merge.

2. **A multi-platform SDK** - The domain model is implemented 4 times (TS, Python, C#, Go) with cross-validation.

3. **An agent-first development environment** - The ticket system, MCP server, and AGENTS.md are designed for AI-assisted development.

### Where Business Logic Lives

- **Domain logic**: `semio.ts` (authoritative), mirrored in `engine.py` and `Semio.cs`
- **UI state logic**: `Sketchpad.tsx` XState machine and app stores
- **Validation logic**: Constraint functions in `semio.ts`
- **Repo logic**: `repo.go` policies and analyzers

### Refactoring Priorities

1. **Split Sketchpad.tsx** - Extract Store base classes, KitStore, and each AppStore to separate files.

2. **Code-generate diff functions** - The pattern is mechanical; generate from schema.

3. **Unify schema definitions** - Single source of truth (e.g., TypeBox or Zod) generating JSON Schema, SQL, and cross-language types.

4. **Formalize FileProvider** - Complete the abstraction for local/remote/memory file storage.

### What Would Explode

- **Changing `Guid` to a different type** → Every entity, every diff, every ID reference
- **Removing Y.js** → Complete rewrite of collaborative features
- **Changing the Plane coordinate system** → All 3D rendering, connection placement, model transforms
- **Altering the KitDiff structure** → Undo/redo, validation fixes, cross-platform sync

---

## Appendix A: Entity Hierarchy

```
Kit
├── Types[]
│   ├── Models[]
│   │   └── Tags[] (TagId refs)
│   ├── Connectors[]
│   │   ├── Props[]
│   │   └── Interface (InterfaceId ref)
│   ├── Attributes[]
│   └── Authors[] (AuthorId refs)
├── Designs[]
│   ├── Pieces[]
│   │   ├── Type (TypeId ref) OR Design (DesignId ref)
│   │   ├── Plane, Center, Scale
│   │   └── Attributes[]
│   ├── Connections[]
│   │   ├── Connected (Side: Piece + Connector)
│   │   ├── Connecting (Side: Piece + Connector)
│   │   └── Gap, Shift, Rise, Rotation, Turn, Tilt
│   ├── Layers[]
│   ├── Groups[]
│   ├── Stats[]
│   └── Attributes[]
├── Qualities[]
│   ├── Benchmarks[]
│   └── Kind (General, Type, Design, Piece, Connection, Connector)
├── Interfaces[] (Ports)
│   └── CompatibleInterfaces[]
├── Files[]
├── Folders[]
├── Tags[]
├── Concepts[]
├── Authors[]
└── Attributes[]
```

---

## Appendix B: Technology Stack

| Layer              | Technologies                                                                                                   |
| ------------------ | -------------------------------------------------------------------------------------------------------------- |
| **Frontend**       | React 19, TypeScript, Tailwind CSS v4, Radix UI, React Three Fiber, React Flow, Golden Layout, XState v5, Y.js |
| **Desktop**        | Electron Forge                                                                                                 |
| **Documentation**  | Astro, Starlight, MDX                                                                                          |
| **Python Backend** | FastAPI, SQLModel, Graphene, MCP, Uvicorn                                                                      |
| **C# Core**        | .NET 8, FluentValidation, QuikGraph, Newtonsoft.JSON                                                           |
| **Grasshopper**    | Rhino 8, Grasshopper SDK                                                                                       |
| **Go Tooling**     | graphql-go, mcp-go, SQLite                                                                                     |
| **Build**          | Nx, Vite, Storybook, TypeScript, ESLint, Prettier, Ruff, Husky                                                 |
| **Testing**        | Vitest, Playwright, pytest                                                                                     |
| **Storage**        | SQLite, IndexedDB, Y.js/CRDT                                                                                   |

---

## Appendix C: File Counts by Language

| Language    | Files   | ~LOC        |
| ----------- | ------- | ----------- |
| TypeScript  | 50+     | 35,000      |
| Python      | 5+      | 10,000      |
| C#          | 10+     | 15,000      |
| Go          | 5+      | 12,000      |
| GraphQL     | 3       | 1,500       |
| SQL         | 2       | 500         |
| JSON Schema | 8       | 2,000       |
| **Total**   | **~85** | **~75,000** |

---

## Appendix D: Detailed File Analysis

This section provides an in-depth analysis of the 7 main source files that form the core of the semio system.

---

### D.1 `js/semio/semio.ts` (7,741 LOC)

**Purpose**: The canonical source of truth for the semio domain model in TypeScript.

#### Structure

| Region          | Lines      | Purpose                                                                                        |
| --------------- | ---------- | ---------------------------------------------------------------------------------------------- |
| Header          | 1-20       | LGPL-3.0 license header                                                                        |
| Constants       | 40-45      | `ICON_WIDTH`, `TOLERANCE` from config                                                          |
| Utilities       | 47-130     | `cn()`, `guid()`, `normalize()`, `round()`, `jaccard()`, `deepEqual()`, `generateUniqueName()` |
| Entity IDs      | 150-250    | 21 ID types with Zod schemas and factory functions                                             |
| Weak Entities   | ~250-400   | `Coord`, `Vec`, `Point`, `Vector`, `Plane`, `Camera`                                           |
| Domain Entities | ~400-3000  | Full entity definitions with schemas                                                           |
| Diff System     | ~3000-5500 | Per-entity diff types, `getDiff`, `inverseDiff`, `mergeDiff`, `applyDiff`                      |
| Validation      | ~5500-6500 | Constraint functions, `validateKit()`, problem/fix generation                                  |
| Cytoscape       | ~6500-7000 | Graph visualization integration                                                                |
| THREE.js        | ~7000-7400 | Geometry helpers, coordinate transforms                                                        |
| Exports         | ~7400-7741 | Public API surface                                                                             |

#### Key Patterns

```typescript
// 1. Entity ID Pattern - Every entity has a typed ID
export type AttributeId = { guid: Guid };
export const createAttributeId = (guid: Guid): AttributeId => ({ guid });
export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.guid === b.guid;

// 2. Zod Schema Pattern - Runtime validation
export const AttributeSchema = z.object({
  guid: z.string(),
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});

// 3. Diff Pattern - Every entity has 4 diff functions
export const getAttributeDiff = (before: Attribute, after: Attribute): AttributeDiff => {...};
export const inverseAttributeDiff = (original: Attribute, diff: AttributeDiff): AttributeDiff => {...};
export const mergeAttributeDiff = (a: AttributeDiff, b: AttributeDiff): AttributeDiff => {...};
export const applyAttributeDiff = (base: Attribute, diff: AttributeDiff): Attribute => {...};

// 4. Coordinate Transform - Left-handed to Three.js right-handed
export const toThreeRotation = (): THREE.Matrix4 =>
  new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);
```

#### Risk Assessment

| Issue                                  | Severity | Recommendation                     |
| -------------------------------------- | -------- | ---------------------------------- |
| Manual diff functions for 20+ entities | HIGH     | Code-generate from schema          |
| 7,741 LOC in single file               | MEDIUM   | Consider splitting by entity group |
| Tight coupling to THREE.js             | LOW      | Acceptable for 3D domain           |

---

### D.2 `js/semio/sketchpad/Sketchpad.tsx` (15,835 LOC)

**Purpose**: Main web application shell with state management, stores, and core UI infrastructure.

#### Structure

| Region             | Lines        | Purpose                                                  |
| ------------------ | ------------ | -------------------------------------------------------- |
| Header             | 1-20         | LGPL-3.0 license                                         |
| Imports            | 23-247       | 200+ imports from React, XState, Y.js, Three.js, etc.    |
| Store Base Classes | 250-820      | `Store<TState>`, `AppStore`, `KitDiffAppStore`           |
| Plain App Store    | 822-1106     | Non-Y.js store variants                                  |
| File Providers     | 1110-1380    | Memory, Local (IndexedDB), Remote, Composite             |
| Y.js Entity Stores | 1380-8000    | Per-entity Y.js wrappers (Attribute, Point, Plane, etc.) |
| KitStore           | ~8000-10000  | Main kit data store with Y.js backing                    |
| SketchpadStore     | ~10000-12000 | Root store aggregating kits and settings                 |
| XState Machine     | ~12000-14000 | `sketchpadMachine` with navigation states                |
| Hooks              | ~14000-15500 | React hooks for state access                             |
| UI Components      | ~15500-15835 | Navbar, Footer, Canvas, Panels                           |

#### Store Hierarchy

```
Store<TState>                    # Base: snapshot caching, Y.js observation
    ├── AppStore                 # + Transaction support, undo/redo, command registry
    │       └── KitDiffAppStore  # + Kit modification tracking
    │
    └── PlainAppStore            # Non-Y.js variant (no CRDT sync)
            └── PlainKitDiffAppStore
```

#### Key Patterns

```typescript
// 1. Y.js Store with dirty tracking
export abstract class Store<TState> {
  protected dirty: boolean = true;
  protected cache?: TState;

  snapshot(): TState {
    if (!this.dirty && this.cache) return this.cache;
    this.cache = this.buildSnapshot();
    this.dirty = false;
    return this.cache;
  }
}

// 2. Transaction Pattern
startTransaction(): void {
  this.isTransactionActive = true;
}
finalizeTransaction(): void {
  // Merge all edits into single undo step
  const edits = currentStack.toArray();
  const mergedEdit = { do: lastEdit.do, undo: firstEdit.undo };
  pastStack.push([mergedEdit]);
}

// 3. Field-level subscriptions
onFieldChanged(key: string, subscribe: Subscribe, deep: boolean = false): Unsubscribe {
  // Granular reactivity without re-rendering entire tree
}
```

#### Risk Assessment

| Issue                            | Severity | Recommendation                        |
| -------------------------------- | -------- | ------------------------------------- |
| 15,835 LOC in single file        | CRITICAL | Split into Store/, Hooks/, Providers/ |
| Y.js ↔ XState bidirectional sync | HIGH     | Document invariants, add tests        |
| 200+ imports                     | MEDIUM   | Bundle analysis, tree shaking         |

---

### D.3 `js/semio/sketchpad/Design.tsx` (8,187 LOC)

**Purpose**: Design editor application with piece/connection management, 3D scene, and diagram views.

#### Structure

| Region              | Lines     | Purpose                                                 |
| ------------------- | --------- | ------------------------------------------------------- |
| Header              | 1-20      | License                                                 |
| Internal State      | 23-200    | Interfaces for selection, hover, presence, diff         |
| Imports             | 73-215    | Lazy loading of KitSection                              |
| State Types         | 219-310   | `DesignAppSelection`, `DesignAppState`, `DesignAppEdit` |
| Commands            | 310-500   | `semio.designApp.*` command implementations             |
| Plugin Registration | 500-800   | XState event handlers                                   |
| XState Hooks        | 800-2500  | `useDesignApp*` hooks (selection, hover, camera, etc.)  |
| UI Components       | 2500-8000 | Scene, Diagram, Panels, Tools                           |
| Providers           | 8000-8187 | Context providers for piece/connection scope            |

#### Commands

```typescript
export const commands = {
  "semio.designApp.selectAll": (context) => ({
    diff: { selection: { pieces: { added: allPieceGuids } } }
  }),
  "semio.designApp.deleteSelected": (context) => ({
    diff: { selection: { pieces: { removed: selectedPieces } } },
    kitDiff: { designs: { updated: [{ diff: { pieces: { removed: ... } } }] } }
  }),
  "semio.designApp.hoverPiece": (context, guid) => ({
    diff: { hover: { pieces: [guid] } }
  }),
  // ... 20+ more commands
};
```

#### State Shape

```typescript
interface DesignAppState {
  fullscreenWindow: DesignAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool?: ToolKind;
  selection?: DesignAppSelection; // pieces[], connections[], connector
  hover?: DesignAppHover; // pieces[], types[], designs[]
  presence?: DesignAppPresence; // cursor, camera
  others: DesignAppPresenceOther[]; // multiplayer cursors
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
  focusedPieceGuid?: Guid;
  selectedModelTags?: Record<Guid, string[]>;
  windowLayout?: any;
}
```

#### Risk Assessment

| Issue                            | Severity | Recommendation                    |
| -------------------------------- | -------- | --------------------------------- |
| 8,187 LOC                        | HIGH     | Split into Commands/, Hooks/, UI/ |
| Tightly coupled to Sketchpad.tsx | MEDIUM   | Extract shared types to shared.ts |

---

### D.4 `js/semio/sketchpad/Type.tsx` (3,399 LOC)

**Purpose**: Type editor for defining connectors, models, and type properties.

#### Structure

| Region              | Lines     | Purpose                                            |
| ------------------- | --------- | -------------------------------------------------- |
| Header/Imports      | 1-100     | Standard imports                                   |
| State Types         | 100-190   | `TypeAppSelection`, `TypeAppState`, `TypeAppHover` |
| Plugin Registration | 190-400   | XState event handlers for TYPE.\* events           |
| XState Hooks        | 400-750   | `useTypeApp*` hooks                                |
| Commands            | 750-1500  | Connector/model CRUD operations                    |
| UI Components       | 1500-3399 | Scene, connector visualization, model selector     |

#### Key Differences from Design.tsx

| Aspect           | Design.tsx                      | Type.tsx                              |
| ---------------- | ------------------------------- | ------------------------------------- |
| Primary Entities | Pieces, Connections             | Connectors, Models                    |
| Selection        | Multi-select pieces/connections | Single connector, multi-select models |
| 3D Focus         | Design composition              | Connector placement on model          |
| Tools            | Lasso, connection tool          | Connector creation tool               |

---

### D.5 `py/engine/engine.py` (7,727 LOC)

**Purpose**: Python backend with FastAPI REST, GraphQL, SQLite persistence, and AI integration.

#### Structure

| Region          | Lines     | Purpose                                   |
| --------------- | --------- | ----------------------------------------- |
| Header          | 1-35      | License, TODOs                            |
| Imports         | 39-120    | SQLModel, FastAPI, Graphene, MCP          |
| Constants       | 140-200   | Limits, paths, MIME types                 |
| Utility         | 200-280   | Encoding, normalization, logging          |
| Exceptions      | 280-350   | Custom error hierarchy                    |
| Modeling Base   | 350-500   | `Model`, `Entity`, `Table`, `Id`, `Props` |
| GraphQL Base    | 500-600   | `Node`, `TableNode`, `RelayNode`          |
| Domain Entities | 600-5000  | All entities as SQLModel tables           |
| API Routes      | 5000-6500 | FastAPI endpoints                         |
| GraphQL Schema  | 6500-7200 | Graphene queries/mutations                |
| MCP Server      | 7200-7727 | FastMCP tool definitions                  |

#### Key Patterns

```python
# 1. SQLModel Entity with validation
class Attribute(TableEntity, table=True):
    PLURAL = "attributes"
    __tablename__ = "attributes"

    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)
    value: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

    def idMembers(self) -> RecursiveAnyList:
        return [self.name]

# 2. Graphene integration
class TableEntityNode(TableNode):
    class Meta:
        abstract = True
        interfaces = (RelayNode,)

    def resolve_id(self, info):
        return self.guid()

# 3. FastAPI + MCP
mcp = FastMCP("semio")

@mcp.tool()
def get_kit(path: str) -> Kit:
    """Get kit from path."""
    return Kit.from_path(path)
```

#### Risk Assessment

| Issue                      | Severity | Recommendation                     |
| -------------------------- | -------- | ---------------------------------- |
| 7,727 LOC single file      | HIGH     | Split into models/, api/, graphql/ |
| Manual schema sync with TS | CRITICAL | Shared schema definition           |
| Python 3.13+ type hints    | LOW      | Compatible with modern Python      |

---

### D.6 `net/Semio/Semio.cs` (5,734 LOC)

**Purpose**: C# domain library for Rhino/Grasshopper integration with validation, graphs, and expression evaluation.

#### Structure

| Region           | Lines     | Purpose                                          |
| ---------------- | --------- | ------------------------------------------------ |
| Header           | 1-35      | License, TODOs                                   |
| Using            | 36-60     | External dependencies                            |
| Constants        | 63-120    | Matching Python/TS constants                     |
| Utility          | 123-400   | Serialization, encoding, unit conversion         |
| Expressions      | 400-800   | AST for formula evaluation (Sum, Multiply, etc.) |
| Validation       | 800-1000  | FluentValidation base classes                    |
| Domain Entities  | 1000-4500 | Entity definitions with validators               |
| Graph Algorithms | 4500-5200 | QuikGraph for design topology                    |
| SVG Generation   | 5200-5600 | Diagram export                                   |
| API Client       | 5600-5734 | Refit-based engine client                        |

#### Key Patterns

```csharp
// 1. Expression Evaluation System
public abstract class Operator : Symbol {
    public abstract string Keyword { get; }
    public abstract object Apply(object[] args, string targetUnit = "");
}

public class Sum : Operator {
    public override string Keyword => "sum";
    public override object Apply(object[] args, string targetUnit = "") {
        var unitValues = ConvertArgsToUnitValues(args);
        var commonUnit = DetermineCommonUnit(unitValues);
        return new UnitValue(unitValues.Sum(uv => uv.ConvertTo(commonUnit)), commonUnit);
    }
}

// 2. Unit Conversion via UnitsNet
public static class Units {
    public static float Convert(float value, string fromUnit, string toUnit) {
        return (float)UnitConverter.Convert(value, fromUnit, toUnit);
    }
}

// 3. FluentValidation
public class AttributeValidator : AbstractValidator<Attribute> {
    public AttributeValidator() {
        RuleFor(a => a.Name).NotEmpty().MaximumLength(Constants.NameLengthLimit);
    }
}
```

#### Dependencies

| Package          | Purpose                   |
| ---------------- | ------------------------- |
| Newtonsoft.Json  | Serialization (camelCase) |
| FluentValidation | Entity validation         |
| QuikGraph        | Graph algorithms          |
| Svg              | SVG generation            |
| UnitsNet         | Physical unit conversion  |
| Refit            | Type-safe HTTP client     |

---

### D.7 `net/Semio.Grasshopper/Semio.Grasshopper.cs` (5,978 LOC)

**Purpose**: Grasshopper plugin exposing semio entities as visual programming components.

#### Structure

| Region         | Lines     | Purpose                                                   |
| -------------- | --------- | --------------------------------------------------------- |
| Header/TODOs   | 1-60      | Notes on future improvements                              |
| Constants      | 68-90     | Category, version                                         |
| Utility        | 99-230    | Plane computation, connection placement                   |
| Converters     | 234-255   | Rhino ↔ semio type conversion                             |
| Base Classes   | 260-690   | `Goo<T>`, `Param<T>`, `Component`, `PassthroughComponent` |
| Entity Regions | 690-5800  | Per-entity Goo, Param, Component classes                  |
| Scripting      | 5800-5890 | Custom script component base                              |
| Engine         | 5890-5978 | Persistence and engine communication                      |

#### Component Pattern

Each entity gets 4-5 Grasshopper classes:

```csharp
// 1. Goo - Wrapper for Grasshopper data tree
public class AttributeGoo : Goo<Attribute> { }

// 2. Param - Parameter definition
public class AttributeParam : Param<AttributeGoo, Attribute> {
    protected override string ModelName => "Attribute";
    protected override string IconResourceName => "attribute_24x24";
}

// 3. PassthroughComponent - Construct/Deconstruct/Modify
public class AttributeComponent : PassthroughComponent<AttributeParam, AttributeGoo, Attribute> {
    protected override void RegisterModelInputParams(GH_InputParamManager pManager) {
        pManager.AddTextParameter("Key", "Ky", "The key.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Va?", "The optional value.", GH_ParamAccess.item);
    }
}

// 4. SerializeComponent - To JSON
public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute> { }

// 5. DeserializeComponent - From JSON
public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute> { }
```

#### Rhino Converters

```csharp
public static class RhinoConverter {
    public static Point3d Convert(this Point point) =>
        new Point3d(point.X, point.Y, point.Z);

    public static Point Convert(this Point3d point) =>
        new Point { X = (float)point.X, Y = (float)point.Y, Z = (float)point.Z };

    public static Rhino.Geometry.Plane Convert(this Plane plane) =>
        new(plane.Origin.Convert(), plane.XAxis.Convert(), plane.YAxis.Convert());
}
```

---

### D.8 `go/semio/semio.go` (4,960 LOC)

**Purpose**: Go implementation of semio domain with validation, flattening, and JSON serialization.

#### Structure

| Region             | Lines     | Purpose                                    |
| ------------------ | --------- | ------------------------------------------ |
| Header             | 1-5       | License                                    |
| Imports            | 7-17      | crypto/rand, encoding, gonum/mat           |
| Constants          | 20-24     | IconWidth, Tolerance                       |
| Utils              | 27-52     | Guid, Normalize, Round, DeepEqual          |
| Entity IDs         | 55-130    | ID structs with JSON tags                  |
| Weak Entities      | 135-185   | Coord, Vec, Point, Vector, Plane, Camera   |
| Attribute          | 190-210   | Attribute + AttributeDiff + AttributesDiff |
| ... (all entities) | 210-1120  | Full domain model                          |
| Serialization      | 1128-1148 | JSON marshal/unmarshal                     |
| Helpers            | 1152-1260 | Entity lookup, traversal                   |
| Factories          | 1264-1389 | Entity constructors                        |
| Kit Operations     | 1393-3489 | CRUD operations, cloning                   |
| Kit Diff Helpers   | 3493-3589 | Diff computation                           |
| Validation         | 3593-4495 | Constraint checking                        |
| Flatten Design     | 4501-4960 | Piece plane computation                    |

#### Key Patterns

```go
// 1. Entity with JSON tags
type Attribute struct {
    Guid       string  `json:"guid"`
    Key        string  `json:"key"`
    Value      *string `json:"value,omitempty"`
    Definition *string `json:"definition,omitempty"`
}

// 2. Diff structures
type AttributeDiff struct {
    Key        *string `json:"key,omitempty"`
    Value      *string `json:"value,omitempty"`
    Definition *string `json:"definition,omitempty"`
}

type AttributesDiff struct {
    Removed []AttributeId `json:"removed,omitempty"`
    Updated []struct {
        Attribute AttributeId   `json:"attribute"`
        Diff      AttributeDiff `json:"diff"`
    } `json:"updated,omitempty"`
    Added []Attribute `json:"added,omitempty"`
}

// 3. Matrix operations for plane flattening
func FlattenDesign(design *Design, kit *Kit) map[string]*Plane {
    // Uses gonum/mat for matrix multiplication
    transform := mat.NewDense(4, 4, ...)
    // Computes world-space planes for each piece
}
```

#### Risk Assessment

| Issue                 | Severity | Recommendation            |
| --------------------- | -------- | ------------------------- |
| 4,960 LOC single file | MEDIUM   | Acceptable for Go library |
| Manual JSON tags      | LOW      | Standard Go pattern       |
| gonum dependency      | LOW      | Well-maintained library   |

---

## Appendix E: Cross-Implementation Comparison

### Entity Alignment Matrix

| Entity     | TypeScript       | Python     | C#                 | Go      |
| ---------- | ---------------- | ---------- | ------------------ | ------- |
| Attribute  | ✓ Zod            | ✓ SQLModel | ✓ FluentValidation | ✓ JSON  |
| Point      | ✓                | ✓          | ✓                  | ✓       |
| Plane      | ✓ THREE.js       | ✓          | ✓ Rhino.Geometry   | ✓ gonum |
| Type       | ✓                | ✓          | ✓                  | ✓       |
| Design     | ✓                | ✓          | ✓                  | ✓       |
| Piece      | ✓                | ✓          | ✓                  | ✓       |
| Connection | ✓                | ✓          | ✓                  | ✓       |
| Kit        | ✓                | ✓          | ✓                  | ✓       |
| KitDiff    | ✓                | ✓          | ✓                  | ✓       |
| Validation | ✓ Problems/Fixes | ✓          | ✓                  | ✓       |

### Feature Parity

| Feature              | TS  | Py  | C#  | Go  |
| -------------------- | --- | --- | --- | --- |
| JSON Serialization   | ✓   | ✓   | ✓   | ✓   |
| Zod/Pydantic Schemas | ✓   | ✓   | -   | -   |
| FluentValidation     | -   | -   | ✓   | -   |
| SQLite Persistence   | -   | ✓   | -   | -   |
| Y.js CRDT            | ✓   | -   | -   | -   |
| GraphQL Server       | -   | ✓   | -   | -   |
| REST API             | -   | ✓   | -   | -   |
| Unit Conversion      | -   | -   | ✓   | -   |
| Expression Eval      | -   | -   | ✓   | -   |
| Plane Flattening     | ✓   | ✓   | ✓   | ✓   |
| Diagram Generation   | ✓   | -   | ✓   | -   |

---

_Report generated by deep codebase analysis. Last updated: January 12, 2026_
