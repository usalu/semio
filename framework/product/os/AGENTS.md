# os

os is a cooperative pseudo operating system with version control, collaboration, plugin-mechanisms, renderers and kernels built in.

[s](`s/AGENTS.md`) is one concrete os instance for the design domain.

# App

An app has a manifest, engine, 

# Manifest

A manifest for an app with schema, commands (along with cmd and cde), 

# Schema

A schema for a document with entity definitions.

# EntityDefinition

# Entity

An entity inside a document.

# Engine

A stateful headless computational engine with bidirectional streaming.

The engine maintains a pack buffer for the document. Alternative versions of the document are computed on the fly by materializing it with operations.

# Command

A command send to the engine.

# Protocol

A native binary protocol for commands.

Protocols are used for communication and storage.

# Cmd

An cmd is a native text representation for commands.

Cdes are used for logging and llms.

# Operation

An operation is a command that modifies a document.

# Patch

A patch is a protocol that modifies a document.

# Op

An op is protocol that modifies a document.

# Document

A document is the data for an app.

# Pack

A pack is a binary representation of a document.

# Dsl

A dsl is a textual representation of a document.

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

---

```
ui
    component
        <component>
            js
                react
                    index.tsx
            rs
                wgpu
                    lib.rs
os
    kernel
        math
        2d
        3d
        dsl
        vcs
        protocol
        neural
        flow
        workflow
        …
    renderer
        js
            react
                index.tsx
    <os> # e.g. s
        kernel
        plugin
            <plugin> # e.g. puzzle, draw, shooting, procedural, fem, energy, …
                app
                    <app> # single crate
                        rs
                            engine.rs
                            manifest
                                document.rs
                                pack.rs
                                command
                                    <command>.rs # each cd
    hub

```