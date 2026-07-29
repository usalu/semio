---
technology: semios
emoji: 🖥️
---

# s

semio s (semi os) is a collaborative operating system for designers to share and store any kind of design knowledge.

It is the ultimate technology that unifyies the complete monorepo.

# Plugin

A plugin is a collection of apps.

# App

An app is 

# Space

A space is the ultimate version controlled container for artifacts.

# Artifact

An artifact is 

## Draft

A draft is a volatile artifact.

## Asset

An asset is a persisted artifact (with optional time to live).

# User

## Author

An author is a user with write permission.

## Specator

A spectator is auser with only read permission.



## Layering

`s/core` → `framework/product/os/core` → `framework/product/platform/core`

S is an **os instance**: composition (store, media graph, program registry, app host resolution) lives in `@semio-tech/framework-os-core`; `s/core` adds S branding (`S_SYSTEM_PROGRAM`), technology registration, and the S playground harness in `s/core/playground.ts`.

Standalone dev for any technology: `bun ./script.ts dev <kind>` via `@semio-tech/framework-playground-dev`.


- space (collections, users)
  - collection (a tree of folders with artifacts - Exportable and importable as zip file)
    - artifact (puzzles, meshes, breps, layouts, flows, files, workflows, etc - Exportable and importable as files)
      - workflow (dynamic, editable non-destructive pipelines of connected apps - Input is a collection and flow parameters and output is a collection)
        - run (a workflow for specific fixed inputs, readonly)
        - automation (an event triggered run)
  - user
    - author (read and write)
    - spectator (readonly)

A draft is a volatile artifact.
An asset is a persisted artifact (optional ttl)
A space for personal use is an atelier (private or public, single writer, multi reader)
A space for a group of users is a studio (private or public, multi writer, multi reader)
A space that is not changing anymore is an archive (private or public, no writer, multi reader)
All apps are accessible over a node in the workflow.
All apps are nondestructive.
All apps have a core library that computes headlessly and a ui to visualize and edit configuration of the app node.
Make sure to identify all gaps and plan all mechanisms and refactor to achieve this architecture.
End to end for a workforce of agents
