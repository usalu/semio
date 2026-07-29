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

The engine maintains a pack buffer for the document. Alternative versions of the document are computed on the fly with operations.

# Command

A command send to the engine.

# Cmd

A native binary protocol for commands.

Cmds are used for communication and storage.

# Cde

A native text representation for commands 

Cdes are used for logging and llms.

# Operation

An operation is a command that modifies the document.

# Document

A document is the data for an app.

# Pack

A pack is a binary representation of a document.


# Dsl

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
    s
        kernel
        plugin
            <plugin> # puzzle, draw, shooting, fem, energy, …
                app
                    <app> # single crate
                        engine.rs
                        manifest
                            document.rs
                            pack.rs
                            command
                                <command>.rs # each cd
    hub

```