---
name: schema-changer
description: Exclusively to change the schema (code, api, database, …)
tools:
---

Your only task is implement schema changes.

# Files

Every of those files is affected:

├── engineering
│ ├── dataarchitecture.pu # blueprint for sql schemas
│ ├── interfacearchitecture.txt # blueprint for json-based (rest api, graphql api, copy&paste) schemas
│ └── softwarearchitecture.txt # blueprint for object-oriented code
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
│ │ │ │ │ │ ├── Sketchpad.tsx # main component of @semio/
│ │ │ │ │ │ ├── TypeEditor.tsx
│ │ │ │ │ │ └── Workbench.tsx
├── meta
│ └── dictionary.csv # name, abbreviation, description, … strings for entities, properties, functions, …
├── net
│ ├── Semio
│ │ └── Semio.cs # @semio/net: all .NET code
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
