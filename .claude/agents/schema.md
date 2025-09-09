---
name: schema
description: Change in the schema (code, api, database, …)
tools:
---

Your only task is implement a schema change.

Files that need to be updated:

├── engineering
│ ├── dataarchitecture.pu # blueprint for sql schemas
│ ├── interfacearchitecture.txt # blueprint for json-based (rest api, graphql api, copy&paste) schemas
│ └── softwarearchitecture.txt # blueprint for object-oriented code
├── antlr
├── js
│ ├── ai
│ ├── core # @semio/js: all shared js code (ui, domain logic, configs, …)
│ │ ├── semio.ts # all domain logic
│ │ ├── store.ts # react hooks for state (uses yjs)
│ │ ├── components
│ │ │ ├── ui
│ │ │ │ ├── sketchpad
│ │ │ │ │ │ ├── Chat.tsx
│ │ │ │ │ │ ├── Commands.tsx
│ │ │ │ │ │ ├── Console.tsx
│ │ │ │ │ │ ├── DesignEditor.tsx
│ │ │ │ │ │ ├── Diagram.tsx
│ │ │ │ │ │ ├── Model.tsx
│ │ │ │ │ │ ├── Navbar.tsx
│ │ │ │ │ │ ├── Workbench.tsx
│ │ │ │ │ │ ├── Sketchpad.stories.tsx
│ │ │ │ │ │ ├── Sketchpad.tsx # main component of @semio/js
│ │ │ │ │ │ ├── TypeEditor.tsx
│ │ │ │ │ │ └── Workbench.tsx
│ ├── docs
│ ├── play
│ ├── playground
│ └── sketchpad
├── jsonschema
├── liveblocks
├── meta
├── net
│ ├── Semio
│ │ ├── Semio.cs # @semio/net: all .NET code
│ │ ├── UserObjects
│ │ │ ├── github
│ │ │ ├── gitlab
│ │ │ ├── monoceros
│ │ │ ├── semio
│ │ │ └── wasp
│ ├── Semio.Grasshopper
│ │ └── Semio.Grasshopper.cs # @semio/gh: all grasshopper code
│ ├── Semio.Grasshopper.Tests
│ └── Semio.Tests
├── rb
├── rdf
├── scripts
├── sqlite
├── yak
└── README.md # GFM dev docs
